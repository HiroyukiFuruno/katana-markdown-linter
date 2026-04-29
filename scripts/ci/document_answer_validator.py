from __future__ import annotations

import difflib
import json
import pathlib
import shutil
import subprocess
import tempfile

from document_answer_manifest import Sample

ACCEPTED_KML_EXIT_CODES = {0, 1}


class AnswerValidationRunner:
    def __init__(self, config: pathlib.Path, command: list[str]) -> None:
        self.config = config
        self.command = command

    def run(self, samples: list[Sample]) -> dict[str, object]:
        with tempfile.TemporaryDirectory(prefix="kml-document-answer-validation-") as raw_tmp:
            temp_root = pathlib.Path(raw_tmp)
            copied = self._copy_answers(temp_root, samples)
            check_completed = self._run_kml([path for _, path in copied], False)
            check_report = self._parse_stdout(check_completed)
            check_reports = self._file_reports(check_report)
            fix_completed = self._run_kml([path for _, path in copied], True)
            fix_report = self._parse_stdout(fix_completed)
            sample_reports = self._sample_reports(copied, check_reports)
        return {
            "summary": {
                "answer_diagnostics": sum(
                    int(item["diagnostics"]) for item in sample_reports
                ),
                "answer_fix_changes": sum(
                    1 for item in sample_reports if item["fix_changed"]
                ),
                "answer_check_exit_code": check_completed.returncode,
                "answer_fix_exit_code": fix_completed.returncode,
            },
            "samples": sample_reports,
            "release_blocking_issues": self._blocking_issues(
                check_completed,
                check_report,
                fix_completed,
                fix_report,
                sample_reports,
            ),
        }

    def _copy_answers(
        self,
        temp_root: pathlib.Path,
        samples: list[Sample],
    ) -> list[tuple[Sample, pathlib.Path]]:
        copied: list[tuple[Sample, pathlib.Path]] = []
        for sample in samples:
            destination = temp_root / f"{sample.id}_answer.md"
            shutil.copyfile(sample.answer_path, destination)
            copied.append((sample, destination))
        return copied

    def _run_kml(
        self,
        paths: list[pathlib.Path],
        fix: bool,
    ) -> subprocess.CompletedProcess[str]:
        command = [
            *self.command,
            "check",
            *self._fix_flag(fix),
            "--config",
            str(self.config),
            "--locale",
            "en",
            "--output",
            "json",
            *[str(path) for path in paths],
        ]
        return subprocess.run(command, text=True, capture_output=True, check=False)

    def _fix_flag(self, fix: bool) -> list[str]:
        return ["--fix"] if fix else []

    def _parse_stdout(self, completed: subprocess.CompletedProcess[str]) -> dict[str, object]:
        try:
            parsed = json.loads(completed.stdout)
        except json.JSONDecodeError:
            return {"files": []}
        return parsed if isinstance(parsed, dict) else {"files": []}

    def _file_reports(self, report: dict[str, object]) -> dict[pathlib.Path, dict[str, object]]:
        files = report.get("files", [])
        if not isinstance(files, list):
            return {}
        reports: dict[pathlib.Path, dict[str, object]] = {}
        for file_report in files:
            if isinstance(file_report, dict) and isinstance(file_report.get("path"), str):
                reports[pathlib.Path(file_report["path"]).resolve()] = file_report
        return reports

    def _sample_reports(
        self,
        copied: list[tuple[Sample, pathlib.Path]],
        check_reports: dict[pathlib.Path, dict[str, object]],
    ) -> list[dict[str, object]]:
        items: list[dict[str, object]] = []
        for sample, answer_copy in copied:
            diagnostics = self._diagnostics(check_reports.get(answer_copy.resolve(), {}))
            actual = answer_copy.read_bytes()
            expected = sample.answer_path.read_bytes()
            fix_changed = actual != expected
            items.append({
                "id": sample.id,
                "kind": sample.kind,
                "diagnostics": len(diagnostics),
                "rules": self._rules(diagnostics),
                "fix_changed": fix_changed,
                "classification": self._classification(diagnostics, fix_changed),
                "fix_diff": [] if not fix_changed else self._diff(expected, actual),
            })
        return items

    def _diagnostics(self, file_report: dict[str, object]) -> list[dict[str, object]]:
        diagnostics = file_report.get("diagnostics", [])
        if not isinstance(diagnostics, list):
            return []
        return [item for item in diagnostics if isinstance(item, dict)]

    def _rules(self, diagnostics: list[dict[str, object]]) -> list[str]:
        rules = {
            item["rule_id"]
            for item in diagnostics
            if isinstance(item.get("rule_id"), str)
        }
        return sorted(rules)

    def _classification(
        self,
        diagnostics: list[dict[str, object]],
        fix_changed: bool,
    ) -> str:
        return "valid-answer" if not diagnostics and not fix_changed else "invalid-answer"

    def _diff(self, expected: bytes, actual: bytes) -> list[str]:
        expected_lines = expected.decode("utf-8", errors="replace").splitlines(keepends=True)
        actual_lines = actual.decode("utf-8", errors="replace").splitlines(keepends=True)
        return list(difflib.unified_diff(expected_lines, actual_lines, "answer", "fixed-answer"))[
            :120
        ]

    def _blocking_issues(
        self,
        check_completed: subprocess.CompletedProcess[str],
        check_report: dict[str, object],
        fix_completed: subprocess.CompletedProcess[str],
        fix_report: dict[str, object],
        sample_reports: list[dict[str, object]],
    ) -> list[str]:
        issues = []
        if self._command_has_blocking_error(check_completed, check_report):
            issues.append("answer fixture check failed before diagnostics")
        if self._command_has_blocking_error(fix_completed, fix_report):
            issues.append("answer fixture fix check failed before idempotence comparison")
        for item in sample_reports:
            if item["fix_changed"]:
                issues.append(f"{item['id']}: answer fixture changes when fixed again")
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
