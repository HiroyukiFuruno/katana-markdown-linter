from __future__ import annotations

import json
import pathlib
import subprocess
import sys
import tempfile
import time
from dataclasses import dataclass

from public_confidence_corpus import CorpusInventory

SCHEMA_VERSION = 1
KNOWN_CLASSIFICATIONS = (
    "true-positive",
    "false-positive",
    "false-negative",
    "unsafe-fix-risk",
    "fmt-policy-gap",
)
KNOWN_TRUE_POSITIVE_RULES = {
    "md-broken-link",
    "md018",
    "md037",
    "md038",
    "md039",
}



@dataclass(frozen=True)
class CommandEvidence:
    exit_code: int
    elapsed_ms: float
    report: dict[str, object]


class PublicConfidenceRunner:
    def __init__(
        self,
        inventory: CorpusInventory,
        mode: str,
        config: pathlib.Path,
        command: list[str],
    ) -> None:
        self.inventory = inventory
        self.mode = mode
        self.config = config
        self.command = command
        self.release_blockers: list[str] = []

    def run(self) -> dict[str, object]:
        before_hashes = self.inventory.hashes()
        check = self._run_kml("check", [item.path for item in self.inventory.files])
        source_unchanged = before_hashes == self.inventory.hashes()
        self._record_check_blockers(check, source_unchanged)

        with tempfile.TemporaryDirectory(prefix="kml-public-confidence-") as raw_tmp:
            copied = self.inventory.copy_to(pathlib.Path(raw_tmp))
            fix = self._run_kml("fix", copied)
            fix_again = self._run_kml("fix", copied)
            fmt = self._run_kml("fmt", copied)
            fmt_again = self._run_kml("fmt", copied)
            final_check = self._run_kml("check", copied)

        fix_converged = fix.exit_code in (0, 1) and self._changed_files(fix_again.report) == 0
        fmt_converged = fmt.exit_code == 0 and self._changed_files(fmt_again.report) == 0
        self._record_convergence_blockers(fix, fix_again, fmt_converged, final_check)

        check_diagnostics = self._classified_diagnostics(check.report)
        check_classification_summary = self._classification_summary(check_diagnostics)
        self._record_classification_blockers(check_classification_summary)

        return {
            "schema_version": SCHEMA_VERSION,
            "source": {"mode": self.mode},
            "inventory": self.inventory.report(),
            "check": self._command_report(check) | {
                "source_unchanged": source_unchanged,
                "diagnostics": check_diagnostics,
                "classification_summary": check_classification_summary["by_classification"],
                "unclassified_count": check_classification_summary["unclassified_count"],
            },
            "fix": self._command_report(fix) | {"converged": fix_converged},
            "fmt": self._command_report(fmt) | {"converged": fmt_converged},
            "final_check": self._command_report(final_check),
            "release_blocking_issues": self.release_blockers,
        }

    def _record_check_blockers(self, check: CommandEvidence, source_unchanged: bool) -> None:
        if check.exit_code not in (0, 1):
            self.release_blockers.append("check failed before diagnostics were reported")
        if not source_unchanged:
            self.release_blockers.append("check modified source corpus")

    def _record_convergence_blockers(
        self,
        fix: CommandEvidence,
        fix_again: CommandEvidence,
        fmt_converged: bool,
        final_check: CommandEvidence,
    ) -> None:
        if fix.exit_code == 2 or fix_again.exit_code == 2:
            self.release_blockers.append("fix failed before diagnostics were reported")
        if self._changed_files(fix_again.report) > 0:
            self.release_blockers.append("fix did not converge on a second run")
        if not fmt_converged:
            self.release_blockers.append("fmt did not converge on a second run")
        if final_check.exit_code == 2:
            self.release_blockers.append("post-fix fmt check failed before diagnostics were reported")

    def _record_classification_blockers(self, summary: dict[str, object]) -> None:
        unclassified_count = int(summary["unclassified_count"])
        if unclassified_count > 0:
            self.release_blockers.append(
                f"check diagnostics include {unclassified_count} unclassified finding(s)"
            )

    def _run_kml(self, command_name: str, paths: list[pathlib.Path]) -> CommandEvidence:
        command = [
            *self.command,
            command_name,
            "--config",
            str(self.config),
            "--locale",
            "en",
            "--output",
            "json",
            *[str(path) for path in paths],
        ]
        started = time.perf_counter()
        completed = subprocess.run(command, text=True, capture_output=True, check=False)
        elapsed_ms = (time.perf_counter() - started) * 1000
        report = self._parse_report(command_name, completed)
        if completed.returncode not in (0, 1, 2):
            sys.stderr.write(completed.stderr)
            raise SystemExit(completed.returncode)
        return CommandEvidence(completed.returncode, elapsed_ms, report)

    def _parse_report(
        self,
        command_name: str,
        completed: subprocess.CompletedProcess[str],
    ) -> dict[str, object]:
        try:
            return json.loads(completed.stdout)
        except json.JSONDecodeError as error:
            sys.stderr.write(completed.stderr)
            sys.stderr.write(completed.stdout)
            raise SystemExit(f"{command_name} did not emit JSON: {error}") from error

    def _command_report(self, evidence: CommandEvidence) -> dict[str, object]:
        return {
            "exit_code": evidence.exit_code,
            "elapsed_ms": round(evidence.elapsed_ms, 3),
            "changed_files": self._changed_files(evidence.report),
            "total_issues": self._total_issues(evidence.report),
        }

    def _classified_diagnostics(self, report: dict[str, object]) -> list[dict[str, object]]:
        diagnostics: list[dict[str, object]] = []
        for file_report in report.get("files", []):
            if not isinstance(file_report, dict):
                continue
            diagnostics.extend(self._diagnostics_for_file(file_report))
        return diagnostics

    def _diagnostics_for_file(self, file_report: dict[object, object]) -> list[dict[str, object]]:
        path = str(file_report.get("path", ""))
        diagnostics: list[dict[str, object]] = []
        for diagnostic in file_report.get("diagnostics", []):
            if isinstance(diagnostic, dict):
                classification = self._classify_diagnostic(diagnostic)
                diagnostics.append({
                    "source_path": path,
                    "rule": diagnostic.get("rule_id", ""),
                    "severity": diagnostic.get("severity", "warning"),
                    "message": diagnostic.get("message", ""),
                    "classification": classification,
                })
        return diagnostics

    def _classify_diagnostic(self, diagnostic: dict[object, object]) -> str:
        rule = str(diagnostic.get("rule_id", "")).strip().lower()
        message = str(diagnostic.get("message", "")).strip().lower()

        if rule in KNOWN_TRUE_POSITIVE_RULES:
            return "true-positive"
        if rule.startswith("md") and rule[2:].isdigit():
            return "true-positive"
        if "broken" in message and "link" in message:
            return "true-positive"
        if "unsafe" in message or "potentially unsafe" in message:
            return "unsafe-fix-risk"
        if "fmt" in message or "format" in message:
            return "fmt-policy-gap"
        if any(
            marker in message
            for marker in (
                "false positive",
                "false-positive",
                "noise",
            )
        ):
            return "false-positive"
        if any(
            marker in message
            for marker in (
                "missed",
                "not detected",
                "should report",
            )
        ):
            return "false-negative"

        return "unclassified"

    def _classification_summary(self, diagnostics: list[dict[str, object]]) -> dict[str, object]:
        by_classification = {classification: 0 for classification in KNOWN_CLASSIFICATIONS}
        by_classification["unclassified"] = 0
        for diagnostic in diagnostics:
            classification = str(diagnostic.get("classification", "unclassified"))
            if classification in by_classification:
                by_classification[classification] += 1
            else:
                by_classification["unclassified"] += 1
        return {
            "by_classification": by_classification,
            "unclassified_count": by_classification["unclassified"],
        }

    def _changed_files(self, report: dict[str, object]) -> int:
        files = report.get("files", [])
        if not isinstance(files, list):
            return 0
        return sum(1 for item in files if isinstance(item, dict) and item.get("changed") is True)

    def _total_issues(self, report: dict[str, object]) -> int:
        summary = report.get("summary", {})
        if isinstance(summary, dict):
            return int(summary.get("total_issues", 0))
        return 0
