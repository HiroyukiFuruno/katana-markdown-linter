#!/usr/bin/env python3
"""Compare the current performance report with the committed baseline."""

from __future__ import annotations

import argparse
import json
import pathlib
import sys
from typing import Any

SCHEMA_VERSION = 2
REQUIRED_CASES = {
    "api_lint_large_document",
    "api_lint_clean_large_document",
    "api_fix_large_document",
    "api_format_large_document",
    "api_lint_many_small_documents",
    "api_lint_link_heavy_document",
    "api_lint_inline_code_heavy_document",
    "api_lint_reference_heavy_document",
    "api_lint_table_heavy_document",
    "api_fix_parser_heavy_document",
    "api_format_parser_heavy_document",
    "context_inline_token_index_large_document",
    "cli_check_many_small_files",
    "cli_fix_many_small_files",
    "cli_fmt_many_small_files",
    "config_validate_representative",
    "api_rule_catalog",
}
REQUIRED_CASE_FIELDS = {
    "name",
    "iterations",
    "samples",
    "work_units",
    "work_unit_name",
    "total_ms",
    "mean_ms",
    "median_ms",
    "min_ms",
    "max_ms",
    "stddev_ms",
    "sample_ms",
    "observed_items",
}
NUMERIC_FIELDS = {
    "total_ms",
    "mean_ms",
    "median_ms",
    "min_ms",
    "max_ms",
    "stddev_ms",
}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline", required=True)
    parser.add_argument("--report", required=True)
    parser.add_argument("--update", action="store_true")
    return parser.parse_args()


def load(path: pathlib.Path) -> dict[str, Any]:
    payload = json.loads(path.read_text(encoding="utf-8"))
    validate_report(payload, path)
    return payload


def validate_report(payload: dict[str, Any], path: pathlib.Path) -> None:
    if payload.get("schema_version") != SCHEMA_VERSION:
        raise SystemExit(
            f"unsupported perf report schema in {path}: {payload.get('schema_version')}"
        )
    cases = cases_by_name(payload)
    missing_cases = sorted(REQUIRED_CASES - set(cases))
    if missing_cases:
        fail_list(f"performance report {path} is missing required cases", missing_cases)
    for name, case in sorted(cases.items()):
        missing_fields = sorted(REQUIRED_CASE_FIELDS - set(case))
        if missing_fields:
            fail_list(f"performance case {name} in {path} is missing fields", missing_fields)
        for field in NUMERIC_FIELDS:
            if not isinstance(case[field], (int, float)):
                raise SystemExit(f"performance case {name} field {field} must be numeric")
        sample_ms = case["sample_ms"]
        samples = case["samples"]
        if not isinstance(samples, int) or samples <= 0:
            raise SystemExit(f"performance case {name} samples must be a positive integer")
        if not isinstance(sample_ms, list) or len(sample_ms) != samples:
            raise SystemExit(
                f"performance case {name} sample_ms length must match samples"
            )
        if not all(isinstance(sample, (int, float)) for sample in sample_ms):
            raise SystemExit(f"performance case {name} sample_ms values must be numeric")


def cases_by_name(payload: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {str(case["name"]): case for case in payload.get("cases", [])}


def fail_list(title: str, items: list[str]) -> None:
    print(title, file=sys.stderr)
    for item in items:
        print(f"- {item}", file=sys.stderr)
    raise SystemExit(1)


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
        fail_list("Performance report is missing baseline cases", missing)

    print("Performance comparison:")
    for name in sorted(baseline_cases):
        baseline_median = float(baseline_cases[name]["median_ms"])
        current_median = float(current_cases[name]["median_ms"])
        ratio = current_median / baseline_median if baseline_median > 0 else 0.0
        print(
            f"- {name}: current_median={current_median:.3f}ms "
            f"baseline_median={baseline_median:.3f}ms ratio={ratio:.2f}x"
        )
    print(
        "Performance check is report-first; missing cases or schema errors fail, "
        "timing regressions are informational."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
