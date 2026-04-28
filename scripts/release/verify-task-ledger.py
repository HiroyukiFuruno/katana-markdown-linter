#!/usr/bin/env python3
"""Verify that the OpenSpec task ledger is release-ready."""

from __future__ import annotations

import argparse
import pathlib
import re
import sys


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--version", required=True, help="Release version such as v0.12.21")
    parser.add_argument("--change", help="OpenSpec change id or tasks.md path")
    parser.add_argument(
        "--allow-open-containing",
        action="append",
        default=[],
        help="Allow an unchecked task only when its line contains this text",
    )
    return parser.parse_args()


def normalized_change_prefix(version: str) -> str:
    version_bare = version.removeprefix("v")
    if not re.fullmatch(r"\d+\.\d+\.\d+(?:[-+][0-9A-Za-z.-]+)?", version_bare):
        raise SystemExit(f"Invalid release version: {version}")
    return "v" + version_bare.split("+", maxsplit=1)[0].split("-", maxsplit=1)[0].replace(".", "-")


def resolve_task_path(args: argparse.Namespace) -> pathlib.Path:
    if args.change:
        candidate = pathlib.Path(args.change)
        if candidate.is_file():
            return candidate
        direct = pathlib.Path("openspec/changes") / args.change / "tasks.md"
        if direct.is_file():
            return direct
        raise SystemExit(f"OpenSpec tasks.md was not found for --change {args.change!r}")

    prefix = normalized_change_prefix(args.version)
    active = sorted(pathlib.Path("openspec/changes").glob(f"{prefix}-*/tasks.md"))
    archived = sorted(pathlib.Path("openspec/changes/archive").glob(f"*-{prefix}-*/tasks.md"))
    matches = active or archived
    if not matches:
        raise SystemExit(f"OpenSpec tasks.md was not found for release {args.version}")
    if len(matches) > 1:
        paths = "\n".join(str(path) for path in matches)
        raise SystemExit(f"Multiple OpenSpec tasks.md files matched {args.version}:\n{paths}")
    return matches[0]


def open_task_violations(lines: list[str], allow_open_containing: list[str]) -> list[str]:
    violations = []
    for line_number, line in enumerate(lines, start=1):
        if not re.match(r"\s*-\s+\[[ /]\]", line):
            continue
        if any(token in line for token in allow_open_containing):
            continue
        violations.append(f"{line_number}: {line}")
    return violations


def score_violations(lines: list[str]) -> list[str]:
    violations = []
    total_seen = False
    score_row = re.compile(r"^\|\s*([^|]+?)\s*\|\s*(\d+)\s*\|\s*(\d+)\s*\|")
    for line_number, line in enumerate(lines, start=1):
        match = score_row.match(line)
        if not match:
            continue
        label = match.group(1).strip()
        maximum = int(match.group(2))
        current = int(match.group(3))
        if label == "項目":
            continue
        if label == "合計":
            total_seen = True
        if current != maximum:
            violations.append(f"{line_number}: {label} score is {current}/{maximum}")
    if not total_seen:
        violations.append("品質評価スコア table is missing a 合計 row")
    return violations


def main() -> int:
    args = parse_args()
    task_path = resolve_task_path(args)
    lines = task_path.read_text(encoding="utf-8").splitlines()
    violations = []
    violations.extend(open_task_violations(lines, args.allow_open_containing))
    violations.extend(score_violations(lines))
    if violations:
        print(f"Release task ledger is not ready: {task_path}", file=sys.stderr)
        for violation in violations:
            print(f"- {violation}", file=sys.stderr)
        return 1
    print(f"Release task ledger passed: {task_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
