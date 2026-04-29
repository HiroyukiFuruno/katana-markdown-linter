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
            completed = self._run_kml([path for _, path in copied])
            report = self._parse_stdout(completed)
            sample_reports = self._sample_reports(copied, report)
        answer_validation = AnswerValidationRunner(self.config, self.command).run(
            self.manifest.samples,
        )
        blocking = self._blocking_issues(completed, report, sample_reports, answer_validation)
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
                "kml_exit_code": completed.returncode,
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

    def _run_kml(self, paths: list[pathlib.Path]) -> subprocess.CompletedProcess[str]:
        command = [
            *self.command,
            "check",
            "--fix",
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
    ) -> list[dict[str, object]]:
        file_reports = self._file_reports(report)
        items: list[dict[str, object]] = []
        for sample, fixed_path in copied:
            file_report = file_reports.get(fixed_path.resolve(), {})
            actual = fixed_path.read_bytes()
            expected = sample.answer_path.read_bytes()
            items.append(self._sample_report(sample, actual, expected, file_report))
        return items

    def _sample_report(
        self,
        sample: Sample,
        actual: bytes,
        expected: bytes,
        file_report: dict[str, object],
    ) -> dict[str, object]:
        matched = actual == expected
        return {
            "id": sample.id,
            "kind": sample.kind,
            "source": f"{sample.source_repository}:{sample.source_path}",
            "matched": matched,
            "rules": self._rules(file_report),
            "classification": "matched" if matched else "bug-candidate",
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
        completed: subprocess.CompletedProcess[str],
        report: dict[str, object],
        sample_reports: list[dict[str, object]],
        answer_validation: dict[str, object],
    ) -> list[str]:
        issues = []
        if self._command_has_blocking_error(completed, report):
            issues.append("kml check --fix failed before document comparison")
        for item in sample_reports:
            if not item["matched"]:
                issues.append(f"{item['id']}: fixed output differs from answer fixture")
        answer_issues = answer_validation["release_blocking_issues"]
        if isinstance(answer_issues, list):
            issues.extend(str(item) for item in answer_issues)
        return issues

    def _command_has_blocking_error(
        self,
        completed: subprocess.CompletedProcess[str],
        report: dict[str, object],
    ) -> bool:
        errors = report.get("errors")
        if isinstance(errors, list) and errors:
            return True
        if completed.returncode in ACCEPTED_KML_EXIT_CODES:
            return False
        return not (isinstance(errors, list) and "files" in report)
