#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import subprocess
import sys
import tempfile
import zipfile
from pathlib import Path


class McpbSmoke:
    def __init__(self, package: Path) -> None:
        self.package = package

    def run(self) -> None:
        with tempfile.TemporaryDirectory(prefix="kml-mcpb-") as directory:
            root = Path(directory)
            with zipfile.ZipFile(self.package) as archive:
                archive.extractall(root)
            manifest = self.load_manifest(root / "manifest.json")
            binary = self.assert_manifest(manifest, root)
            binary.chmod(0o755)
            subprocess.run(
                [sys.executable, "scripts/ci/mcp-stdio-smoke.py", "--bin", str(binary)],
                check=True,
            )

    @staticmethod
    def load_manifest(path: Path) -> dict[str, object]:
        if not path.is_file():
            raise AssertionError("MCPB package is missing manifest.json")
        payload = json.loads(path.read_text(encoding="utf-8"))
        if not isinstance(payload, dict):
            raise AssertionError("manifest.json must contain a JSON object")
        return payload

    @staticmethod
    def assert_manifest(manifest: dict[str, object], root: Path) -> Path:
        expected = {
            "manifest_version": "0.3",
            "name": "katana-markdown-linter",
            "version": "0.14.0",
        }
        for key, value in expected.items():
            if manifest.get(key) != value:
                raise AssertionError(f"manifest {key} must be {value!r}")

        server = manifest.get("server")
        if not isinstance(server, dict):
            raise AssertionError("manifest server must be an object")
        if server.get("type") != "binary":
            raise AssertionError("manifest server.type must be binary")
        entry_point = server.get("entry_point")
        if entry_point != "server/kml-mcp":
            raise AssertionError("manifest server.entry_point must be server/kml-mcp")

        mcp_config = server.get("mcp_config")
        if not isinstance(mcp_config, dict):
            raise AssertionError("manifest server.mcp_config must be an object")
        if mcp_config.get("command") != "${__dirname}/server/kml-mcp":
            raise AssertionError("manifest command must use the bundled binary")
        if mcp_config.get("args") != ["--workspace-root", "${user_config.workspace_root}"]:
            raise AssertionError("manifest args must require a configured workspace root")

        binary = root / entry_point
        if not binary.is_file():
            raise AssertionError(f"bundled kml-mcp binary not found: {binary}")
        return binary


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Smoke test a kml MCPB package.")
    parser.add_argument("--mcpb", required=True, type=Path)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if not args.mcpb.is_file():
        print(f"MCPB package not found: {args.mcpb}", file=sys.stderr)
        return 1
    McpbSmoke(args.mcpb).run()
    print("MCPB smoke passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
