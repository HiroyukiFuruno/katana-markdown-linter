from __future__ import annotations

import difflib
import json
import pathlib
import shutil
import subprocess
import tempfile
import time

from document_answer_validator import AnswerValidationRunner
from document_answer_manifest import Manifest, Sample

SCHEMA_VERSION = 1
ACCEPTED_KML_EXIT_CODES = {0, 1}
FIX_PASSES = 3


def evaluate_fix_passes(expected: bytes, outputs: list[bytes]) -> list[str]:
    """Check each fix result against the reviewed answer and its prior pass."""
    issues: list[str] = []
    previous: bytes | None = None
    for pass_number, output in enumerate(outputs, start=1):
        if output != expected:
            issues.append(f"fix pass {pass_number} differs from answer fixture")
        if previous is not None and output != previous:
            issues.append(f"fix pass {pass_number} changed output after the previous pass")
        previous = output
    return issues


class DocumentAnswerRunner:
    def __init__(self, manifest: Manifest, config: pathlib.Path, command: list[str]) -> None:
        self.manifest = manifest
        self.config = config
        self.command = command

    def run(self) -> dict[str, object]:
        started = time.perf_counter()
        with tempfile.TemporaryDirectory(prefix="kml-document-answer-") as raw_tmp:
            temp_root = pathlib.Path(raw_tmp)
            copied = self._copy_inputs(temp_root)
            paths = [path for _, path in copied]
            fix_runs: list[subprocess.CompletedProcess[str]] = []
            fix_reports: list[dict[str, object]] = []
            pass_outputs: list[dict[pathlib.Path, bytes]] = []
            for _ in range(FIX_PASSES):
                completed = self._run_kml(paths, fix=True)
                fix_runs.append(completed)
                fix_reports.append(self._parse_stdout(completed))
                pass_outputs.append({path.resolve(): path.read_bytes() for path in paths})
            final_check = self._run_kml(paths, fix=False)
            final_check_report = self._parse_stdout(final_check)
            sample_reports = self._sample_reports(copied, fix_reports[0], pass_outputs)
        answer_validation = AnswerValidationRunner(self.config, self.command).run(
            self.manifest.samples,
        )
        blocking = self._blocking_issues(
            fix_runs,
            fix_reports,
            final_check,
            final_check_report,
            sample_reports,
            answer_validation,
            paths,
        )
        safe_fixable_remaining = self._safe_fixable_count(final_check_report)
        return {
            "schema_version": SCHEMA_VERSION,
            "manifest": str(self.manifest.path),
            "config": str(self.config),
            "summary": {
                "total": len(self.manifest.samples),
                "public": self._kind_count("public"),
                "original": self._kind_count("original"),
                "mismatches": sum(1 for item in sample_reports if not item["matched"]),
                "answer_diagnostics": answer_validation["summary"]["answer_diagnostics"],
                "answer_fix_changes": answer_validation["summary"]["answer_fix_changes"],
                "answer_safe_fixable_remaining": answer_validation["summary"][
                    "answer_safe_fixable_remaining"
                ],
                "kml_exit_code": fix_runs[0].returncode,
                "fix_pass_exit_codes": [run.returncode for run in fix_runs],
                "safe_fixable_remaining": safe_fixable_remaining,
                "answer_check_exit_code": answer_validation["summary"][
                    "answer_check_exit_code"
                ],
                "answer_fix_exit_code": answer_validation["summary"]["answer_fix_exit_code"],
                "elapsed_ms": round((time.perf_counter() - started) * 1000, 3),
            },
            "samples": sample_reports,
            "answer_validation": answer_validation["samples"],
            "release_blocking_issues": blocking,
        }

    def _kind_count(self, kind: str) -> int:
        return sum(1 for item in self.manifest.samples if item.kind == kind)

    def _copy_inputs(self, temp_root: pathlib.Path) -> list[tuple[Sample, pathlib.Path]]:
        copied: list[tuple[Sample, pathlib.Path]] = []
        for sample in self.manifest.samples:
            destination = temp_root / f"{sample.id}.md"
            shutil.copyfile(sample.input_path, destination)
            copied.append((sample, destination))
        return copied

    def _run_kml(
        self,
        paths: list[pathlib.Path],
        *,
        fix: bool,
    ) -> subprocess.CompletedProcess[str]:
        command = [
            *self.command,
            "check",
            *(["--fix"] if fix else []),
            "--config",
            str(self.config),
            "--locale",
            "en",
            "--output",
            "json",
            *[str(path) for path in paths],
        ]
        return subprocess.run(command, text=True, capture_output=True, check=False)

    def _parse_stdout(self, completed: subprocess.CompletedProcess[str]) -> dict[str, object]:
        try:
            parsed = json.loads(completed.stdout)
        except json.JSONDecodeError:
            return {"files": []}
        return parsed if isinstance(parsed, dict) else {"files": []}

    def _sample_reports(
        self,
        copied: list[tuple[Sample, pathlib.Path]],
        report: dict[str, object],
        pass_outputs: list[dict[pathlib.Path, bytes]],
    ) -> list[dict[str, object]]:
        file_reports = self._file_reports(report)
        items: list[dict[str, object]] = []
        for sample, fixed_path in copied:
            file_report = file_reports.get(fixed_path.resolve(), {})
            outputs = [snapshot[fixed_path.resolve()] for snapshot in pass_outputs]
            expected = sample.answer_path.read_bytes()
            items.append(self._sample_report(sample, outputs, expected, file_report))
        return items

    def _sample_report(
        self,
        sample: Sample,
        outputs: list[bytes],
        expected: bytes,
        file_report: dict[str, object],
    ) -> dict[str, object]:
        actual = outputs[-1] if outputs else b""
        convergence_issues = evaluate_fix_passes(expected, outputs)
        matched = actual == expected and not convergence_issues
        return {
            "id": sample.id,
            "kind": sample.kind,
            "source": f"{sample.source_repository}:{sample.source_path}",
            "matched": matched,
            "rules": self._rules(file_report),
            "classification": "matched" if matched else "bug-candidate",
            "fix_passes": len(outputs),
            "converged": not any("changed output" in issue for issue in convergence_issues),
            "convergence_issues": convergence_issues,
            "diff": [] if matched else self._diff(expected, actual),
        }

    def _file_reports(self, report: dict[str, object]) -> dict[pathlib.Path, dict[str, object]]:
        files = report.get("files", [])
        if not isinstance(files, list):
            return {}
        reports: dict[pathlib.Path, dict[str, object]] = {}
        for file_report in files:
            if isinstance(file_report, dict) and isinstance(file_report.get("path"), str):
                reports[pathlib.Path(file_report["path"]).resolve()] = file_report
        return reports

    def _rules(self, file_report: dict[str, object]) -> list[str]:
        rules = set()
        for key in ["diagnostics", "fix_details"]:
            for item in file_report.get(key, []):
                if isinstance(item, dict) and isinstance(item.get("rule_id"), str):
                    rules.add(item["rule_id"])
        return sorted(rules)

    def _diff(self, expected: bytes, actual: bytes) -> list[str]:
        expected_lines = expected.decode("utf-8", errors="replace").splitlines(keepends=True)
        actual_lines = actual.decode("utf-8", errors="replace").splitlines(keepends=True)
        return list(difflib.unified_diff(expected_lines, actual_lines, "answer", "actual"))[:120]

    def _blocking_issues(
        self,
        fix_runs: list[subprocess.CompletedProcess[str]],
        fix_reports: list[dict[str, object]],
        final_check: subprocess.CompletedProcess[str],
        final_check_report: dict[str, object],
        sample_reports: list[dict[str, object]],
        answer_validation: dict[str, object],
        expected_paths: list[pathlib.Path],
    ) -> list[str]:
        issues = []
        for pass_number, (completed, report) in enumerate(
            zip(fix_runs, fix_reports, strict=True),
            start=1,
        ):
            if self._command_has_blocking_error(completed, report):
                issues.append(f"kml check --fix pass {pass_number} failed before document comparison")
            elif not self._report_covers_paths(report, expected_paths):
                issues.append(f"kml check --fix pass {pass_number} omitted one or more sample reports")
        if self._command_has_blocking_error(final_check, final_check_report):
            issues.append("kml check failed while verifying remaining safe fixes")
        elif not self._report_covers_paths(final_check_report, expected_paths):
            issues.append("kml check omitted one or more sample reports while verifying remaining safe fixes")
        for item in sample_reports:
            if not item["matched"] and not item["convergence_issues"]:
                issues.append(f"{item['id']}: fixed output differs from answer fixture")
            for convergence_issue in item["convergence_issues"]:
                issues.append(f"{item['id']}: {convergence_issue}")
        safe_fixable_remaining = self._safe_fixable_count(final_check_report)
        if safe_fixable_remaining is None:
            issues.append("kml check report is missing a valid safe_fixable_issues count")
        elif safe_fixable_remaining > 0:
            issues.append(
                f"kml check found {safe_fixable_remaining} remaining safe-fixable diagnostics"
            )
        answer_issues = answer_validation["release_blocking_issues"]
        if isinstance(answer_issues, list):
            issues.extend(str(item) for item in answer_issues)
        return issues

    def _safe_fixable_count(self, report: dict[str, object]) -> int | None:
        summary = report.get("summary")
        if not isinstance(summary, dict):
            return None
        count = summary.get("safe_fixable_issues")
        return count if isinstance(count, int) and not isinstance(count, bool) else None

    def _report_covers_paths(
        self,
        report: dict[str, object],
        paths: list[pathlib.Path],
    ) -> bool:
        reported_paths = self._file_reports(report)
        return all(path.resolve() in reported_paths for path in paths)

    def _command_has_blocking_error(
        self,
        completed: subprocess.CompletedProcess[str],
        report: dict[str, object],
    ) -> bool:
        errors = report.get("errors")
        if not isinstance(errors, list) or not isinstance(report.get("files"), list):
            return True
        if isinstance(errors, list) and errors:
            return True
        if completed.returncode in ACCEPTED_KML_EXIT_CODES:
            return False
        return True
