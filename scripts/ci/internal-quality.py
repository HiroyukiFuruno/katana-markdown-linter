#!/usr/bin/env python3
"""Generate lightweight internal quality evidence for rust src files."""

from __future__ import annotations

import argparse
import datetime
import json
import pathlib
import re
from dataclasses import dataclass


@dataclass
class FileMetric:
    path: str
    total_lines: int
    non_empty_lines: int
    comment_lines: int
    blank_lines: int
    code_lines: int
    functions: int
    impl_blocks: int
    structs: int


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--src", default="src", help="Source root directory")
    parser.add_argument("--report", required=True, help="Path to write JSON report")
    return parser.parse_args()


def collect_rust_files(root: pathlib.Path) -> list[pathlib.Path]:
    files = sorted(root.rglob("*.rs"))
    if not files:
        raise SystemExit(f"no Rust files found under {root}")
    return files


def metric_for_file(path: pathlib.Path) -> FileMetric:
    lines = path.read_text(encoding="utf-8").splitlines()
    total = len(lines)
    blank = sum(1 for line in lines if not line.strip())
    comment = sum(1 for line in lines if line.lstrip().startswith("//"))
    non_empty = total - blank
    code = non_empty - comment

    functions = len(re.findall(r"\bfn\s+", "\n".join(lines)))
    impl_blocks = len(re.findall(r"\bimpl\b", "\n".join(lines)))
    structs = len(re.findall(r"\b(struct|enum|trait)\b", "\n".join(lines)))

    return FileMetric(
        path=str(path),
        total_lines=total,
        non_empty_lines=non_empty,
        comment_lines=comment,
        blank_lines=blank,
        code_lines=code,
        functions=functions,
        impl_blocks=impl_blocks,
        structs=structs,
    )


def split_candidates(metrics: list[FileMetric]) -> list[dict[str, object]]:
    candidates = []
    for metric in metrics:
        if metric.total_lines >= 220 or metric.functions >= 25:
            split_candidates = [
                "cli" if "cli" in metric.path else "general",
                "rule" if "rules" in metric.path else "core",
            ]
            score = metric.code_lines + metric.functions * 5 + metric.impl_blocks * 3
            candidates.append(
                {
                    "path": metric.path,
                    "reason": "high_volume_or_many_functions",
                    "size_score": score,
                    "signals": sorted(set(split_candidates)),
                }
            )
    return sorted(candidates, key=lambda item: item["size_score"], reverse=True)


def build_report(root: pathlib.Path) -> dict[str, object]:
    metrics = [metric_for_file(path) for path in collect_rust_files(root)]
    by_non_empty = sorted(metrics, key=lambda item: item.non_empty_lines, reverse=True)
    by_hot_path = sorted(metrics, key=lambda item: item.functions + item.impl_blocks, reverse=True)

    return {
        "schema_version": 1,
        "generated_at": datetime.datetime.now(datetime.UTC).replace(microsecond=0).isoformat(),
        "root": str(root),
        "totals": {
            "source_files": len(metrics),
            "total_lines": sum(item.total_lines for item in metrics),
            "total_code_lines": sum(item.code_lines for item in metrics),
        },
        "largest_files_by_loc": [
            {
                "path": item.path,
                "non_empty_lines": item.non_empty_lines,
                "code_lines": item.code_lines,
                "functions": item.functions,
                "impl_blocks": item.impl_blocks,
            }
            for item in by_non_empty[:20]
        ],
        "hot_path_candidates": [
            {
                "path": item.path,
                "function_density": item.functions,
                "impl_blocks": item.impl_blocks,
                "struct_like_items": item.structs,
            }
            for item in by_hot_path[:20]
        ],
        "split_candidates": split_candidates(metrics),
        "files": [
            {
                "path": item.path,
                "total_lines": item.total_lines,
                "non_empty_lines": item.non_empty_lines,
                "comment_lines": item.comment_lines,
                "blank_lines": item.blank_lines,
                "code_lines": item.code_lines,
                "functions": item.functions,
                "impl_blocks": item.impl_blocks,
                "structs": item.structs,
            }
            for item in by_non_empty
        ],
    }


def main() -> int:
    args = parse_args()
    root = pathlib.Path(args.src)
    if not root.exists():
        raise SystemExit(f"source directory not found: {root}")

    report = build_report(root)
    target = pathlib.Path(args.report)
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(f"Internal quality report: {target}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
