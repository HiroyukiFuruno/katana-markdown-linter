#!/usr/bin/env python3
"""Compare kml dogfood diagnostics against a committed baseline."""

from __future__ import annotations

import argparse
import collections
import json
import pathlib
import subprocess
import sys
from typing import Any


SCHEMA_VERSION = 1


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline", required=True, help="Committed baseline JSON path")
    parser.add_argument("--report", required=True, help="Raw kml JSON report output path")
    parser.add_argument(
        "--update",
        action="store_true",
        help="Rewrite the baseline from the current kml diagnostics",
    )
    parser.add_argument("command", nargs=argparse.REMAINDER)
    args = parser.parse_args()
    if not args.command or args.command[0] != "--":
        parser.error("pass the kml command after --")
    args.command = args.command[1:]
    if not args.command:
        parser.error("missing kml command")
    return args


def run_kml(command: list[str]) -> dict[str, Any]:
    completed = subprocess.run(command, text=True, capture_output=True, check=False)
    if completed.returncode not in (0, 1):
        sys.stderr.write(completed.stderr)
        sys.stderr.write(completed.stdout)
        raise SystemExit(completed.returncode)
    try:
        return json.loads(completed.stdout)
    except json.JSONDecodeError as error:
        sys.stderr.write(completed.stderr)
        sys.stderr.write(completed.stdout)
        raise SystemExit(f"failed to parse kml JSON output: {error}")


def load_line(path: str, line_number: int) -> str:
    try:
        lines = pathlib.Path(path).read_text(encoding="utf-8").splitlines()
    except OSError:
        return ""
    if line_number < 1 or line_number > len(lines):
        return ""
    return lines[line_number - 1].strip()


def fingerprint(path: str, diagnostic: dict[str, Any]) -> tuple[str, str, str, str]:
    line = int(diagnostic.get("line") or 0)
    return (
        path,
        str(diagnostic.get("rule_id") or ""),
        str(diagnostic.get("message") or ""),
        load_line(path, line),
    )


def collect(report: dict[str, Any]) -> collections.Counter[tuple[str, str, str, str]]:
    counter: collections.Counter[tuple[str, str, str, str]] = collections.Counter()
    for file_report in report.get("files", []):
        path = str(file_report.get("path") or "")
        for diagnostic in file_report.get("diagnostics", []):
            counter[fingerprint(path, diagnostic)] += 1
    return counter


def baseline_payload(
    counter: collections.Counter[tuple[str, str, str, str]],
) -> dict[str, Any]:
    diagnostics = [
        {
            "path": path,
            "rule_id": rule_id,
            "message": message,
            "line_text": line_text,
            "count": count,
        }
        for (path, rule_id, message, line_text), count in sorted(counter.items())
    ]
    return {
        "schema_version": SCHEMA_VERSION,
        "description": "Known kml dogfood diagnostics. New diagnostics fail just dogfood.",
        "total_diagnostics": sum(counter.values()),
        "diagnostics": diagnostics,
    }


def load_baseline(path: pathlib.Path) -> collections.Counter[tuple[str, str, str, str]]:
    payload = json.loads(path.read_text(encoding="utf-8"))
    if payload.get("schema_version") != SCHEMA_VERSION:
        raise SystemExit(f"unsupported dogfood baseline schema: {payload.get('schema_version')}")
    counter: collections.Counter[tuple[str, str, str, str]] = collections.Counter()
    for item in payload.get("diagnostics", []):
        key = (
            str(item.get("path") or ""),
            str(item.get("rule_id") or ""),
            str(item.get("message") or ""),
            str(item.get("line_text") or ""),
        )
        counter[key] += int(item.get("count") or 0)
    return counter


def print_counter(title: str, counter: collections.Counter[tuple[str, str, str, str]]) -> None:
    if not counter:
        return
    print(title)
    for (path, rule_id, message, line_text), count in counter.most_common(50):
        suffix = f" | {line_text}" if line_text else ""
        print(f"- {count}x {path} {rule_id} {message}{suffix}")
    if len(counter) > 50:
        print(f"- ... {len(counter) - 50} more fingerprints")


def main() -> int:
    args = parse_args()
    report = run_kml(args.command)
    if report.get("errors"):
        print(json.dumps(report.get("errors"), indent=2, ensure_ascii=False), file=sys.stderr)
        return 2

    report_path = pathlib.Path(args.report)
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(json.dumps(report, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")

    current = collect(report)
    baseline_path = pathlib.Path(args.baseline)

    if args.update:
        baseline_path.parent.mkdir(parents=True, exist_ok=True)
        baseline_path.write_text(
            json.dumps(baseline_payload(current), indent=2, ensure_ascii=False) + "\n",
            encoding="utf-8",
        )
        print(f"Dogfood baseline refreshed: {sum(current.values())} diagnostics")
        return 0

    baseline = load_baseline(baseline_path)
    new = current - baseline
    resolved = baseline - current
    if new:
        print_counter("Dogfood failed: new diagnostics", new)
        if resolved:
            print_counter("Resolved diagnostics not yet reflected in baseline", resolved)
        print(f"Raw report: {report_path}")
        print("Run `just dogfood-refresh-baseline` only after intentional cleanup.")
        return 1

    print(
        "Dogfood passed: "
        f"{sum(current.values())} current diagnostics, "
        f"{sum(baseline.values())} baseline diagnostics, "
        f"{sum(resolved.values())} resolved"
    )
    print(f"Raw report: {report_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
