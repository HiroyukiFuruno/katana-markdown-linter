#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path


class McpServerJsonRenderer:
    def __init__(self, args: argparse.Namespace) -> None:
        self.args = args

    def run(self) -> None:
        source = self.load_json(self.args.source)
        version_bare = self.args.version.removeprefix("v")
        version = f"v{version_bare}"
        package_path = self.args.mcpb_package
        package_sha = self.sha256(package_path)
        package_url = (
            f"https://github.com/{self.args.repository}/releases/download/"
            f"{version}/{package_path.name}"
        )

        source["version"] = version_bare
        package = self.mcpb_package(source)
        package["identifier"] = package_url
        package["version"] = version_bare
        package["fileSha256"] = package_sha

        self.args.output.parent.mkdir(parents=True, exist_ok=True)
        self.args.output.write_text(
            json.dumps(source, indent=2, ensure_ascii=False) + "\n",
            encoding="utf-8",
        )

        if self.args.github_output:
            with self.args.github_output.open("a", encoding="utf-8") as output:
                output.write(f"server_json_path={self.args.output}\n")

        print(f"Rendered {self.args.output}")

    @staticmethod
    def load_json(path: Path) -> dict[str, object]:
        payload = json.loads(path.read_text(encoding="utf-8"))
        if not isinstance(payload, dict):
            raise TypeError(f"{path} must contain a JSON object")
        return payload

    @staticmethod
    def sha256(path: Path) -> str:
        digest = hashlib.sha256()
        with path.open("rb") as stream:
            for chunk in iter(lambda: stream.read(1024 * 1024), b""):
                digest.update(chunk)
        return digest.hexdigest()

    @staticmethod
    def mcpb_package(source: dict[str, object]) -> dict[str, object]:
        packages = source.get("packages")
        if not isinstance(packages, list):
            raise TypeError("server.json packages must be an array")
        for package in packages:
            if isinstance(package, dict) and package.get("registryType") == "mcpb":
                return package
        raise ValueError("server.json must contain an mcpb package")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Render release-ready MCP Registry metadata.")
    parser.add_argument("--version", required=True, help="Release version, such as v0.14.0")
    parser.add_argument("--mcpb-package", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--source", default=Path("server.json"), type=Path)
    parser.add_argument(
        "--repository",
        default="HiroyukiFuruno/katana-markdown-linter",
        help="GitHub owner/repository used for the release artifact URL.",
    )
    parser.add_argument("--github-output", type=Path, default=None)
    return parser.parse_args()


def main() -> int:
    McpServerJsonRenderer(parse_args()).run()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
