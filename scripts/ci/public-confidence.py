#!/usr/bin/env python3
"""Record public confidence evidence for representative Markdown corpora."""

from __future__ import annotations

import argparse
import json
import pathlib

from public_confidence_corpus import CorpusInventory, KATANA_PATTERNS
from public_confidence_runner import PublicConfidenceRunner


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--report", required=True)
    parser.add_argument("--corpus")
    parser.add_argument("--katana-checkout")
    parser.add_argument("--config", required=True)
    parser.add_argument("command", nargs=argparse.REMAINDER)
    args = parser.parse_args()
    if not args.command or args.command[0] != "--":
        parser.error("pass the kml command after --")
    args.command = args.command[1:]
    if not args.command:
        parser.error("missing kml command")
    if bool(args.corpus) == bool(args.katana_checkout):
        parser.error("pass exactly one of --corpus or --katana-checkout")
    return args


def main() -> int:
    args = parse_args()
    mode = "katana" if args.katana_checkout else "curated"
    root = pathlib.Path(args.katana_checkout or args.corpus).resolve()
    patterns = KATANA_PATTERNS if args.katana_checkout else ("**/*.md",)
    inventory = CorpusInventory(root, patterns)
    runner = PublicConfidenceRunner(inventory, mode, pathlib.Path(args.config), args.command)
    evidence = runner.run()

    report = pathlib.Path(args.report)
    report.parent.mkdir(parents=True, exist_ok=True)
    report.write_text(json.dumps(evidence, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"Public confidence report: {report}")
    return 1 if evidence["release_blocking_issues"] else 0


if __name__ == "__main__":
    raise SystemExit(main())
