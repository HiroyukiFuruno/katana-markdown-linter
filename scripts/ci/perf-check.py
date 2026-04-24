#!/usr/bin/env python3
"""Compare the current performance report with the committed baseline."""

from __future__ import annotations

import argparse
import json
import pathlib
import sys
from typing import Any

SCHEMA_VERSION = 1


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline", required=True)
    parser.add_argument("--report", required=True)
    parser.add_argument("--update", action="store_true")
    return parser.parse_args()


def load(path: pathlib.Path) -> dict[str, Any]:
    payload = json.loads(path.read_text(encoding="utf-8"))
    if payload.get("schema_version") != SCHEMA_VERSION:
        raise SystemExit(
            f"unsupported perf report schema in {path}: {payload.get('schema_version')}"
        )
    return payload


def cases_by_name(payload: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {str(case["name"]): case for case in payload.get("cases", [])}


def main() -> int:
    args = parse_args()
    baseline_path = pathlib.Path(args.baseline)
    report_path = pathlib.Path(args.report)
    report = load(report_path)

    if args.update:
        baseline_path.parent.mkdir(parents=True, exist_ok=True)
        baseline_path.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
        print(f"Performance baseline refreshed: {baseline_path}")
        return 0

    baseline = load(baseline_path)
    current_cases = cases_by_name(report)
    baseline_cases = cases_by_name(baseline)
    missing = sorted(set(baseline_cases) - set(current_cases))
    if missing:
        print("Performance report is missing baseline cases:", file=sys.stderr)
        for name in missing:
            print(f"- {name}", file=sys.stderr)
        return 1

    print("Performance comparison:")
    for name in sorted(baseline_cases):
        baseline_avg = float(baseline_cases[name]["average_ms"])
        current_avg = float(current_cases[name]["average_ms"])
        ratio = current_avg / baseline_avg if baseline_avg > 0 else 0.0
        print(
            f"- {name}: current={current_avg:.3f}ms "
            f"baseline={baseline_avg:.3f}ms ratio={ratio:.2f}x"
        )
    print(
        "Performance check is report-first; missing cases fail, "
        "timing regressions are informational."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
