#!/usr/bin/env python3
"""Compare document-level fix output with reviewed answer fixtures."""

from __future__ import annotations

import argparse
import json
import pathlib
import sys

from document_answer_fix_runner import DocumentAnswerRunner, SCHEMA_VERSION
from document_answer_manifest import Manifest


class DocumentAnswerCli:
    def parse_args(self) -> argparse.Namespace:
        parser = argparse.ArgumentParser(description=__doc__)
        parser.add_argument("--manifest", required=True)
        parser.add_argument("--config", required=True)
        parser.add_argument("--report", required=True)
        parser.add_argument("--minimum-public", type=int, default=200)
        parser.add_argument("--minimum-original", type=int, default=50)
        parser.add_argument("command", nargs=argparse.REMAINDER)
        args = parser.parse_args()
        if not args.command or args.command[0] != "--":
            parser.error("pass the kml command after --")
        args.command = args.command[1:]
        if not args.command:
            parser.error("missing kml command")
        return args

    def run(self) -> int:
        args = self.parse_args()
        manifest = Manifest(pathlib.Path(args.manifest))
        errors = manifest.validate(args.minimum_public, args.minimum_original)
        if errors:
            self.write_report(pathlib.Path(args.report), {
                "schema_version": SCHEMA_VERSION,
                "summary": {"metadata_errors": len(errors)},
                "release_blocking_issues": errors,
            })
            return 1
        runner = DocumentAnswerRunner(manifest, pathlib.Path(args.config), args.command)
        report = runner.run()
        self.write_report(pathlib.Path(args.report), report)
        return 1 if report["release_blocking_issues"] else 0

    def write_report(self, path: pathlib.Path, payload: dict[str, object]) -> None:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        print(f"Document answer fix report: {path}")


if __name__ == "__main__":
    try:
        raise SystemExit(DocumentAnswerCli().run())
    except BrokenPipeError:
        sys.exit(1)
