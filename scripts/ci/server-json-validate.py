#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path


class ServerJsonValidator:
    NAME = re.compile(r"^[a-zA-Z0-9.-]+/[a-zA-Z0-9._-]+$")
    SHA256 = re.compile(r"^[a-f0-9]{64}$")
    VERSION = re.compile(r"^\d+\.\d+\.\d+(?:[-+][0-9A-Za-z.-]+)?$")

    def __init__(self, path: Path, expected_version: str) -> None:
        self.path = path
        self.expected_version = expected_version.removeprefix("v")
        self.violations: list[str] = []

    def run(self) -> int:
        payload = json.loads(self.path.read_text(encoding="utf-8"))
        if not isinstance(payload, dict):
            self.violations.append("server.json must contain a JSON object")
            return self.report()
        self.validate_root(payload)
        self.validate_packages(payload)
        return self.report()

    def validate_root(self, payload: dict[str, object]) -> None:
        self.require(payload.get("$schema") == "https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json", "$schema must point to the 2025-12-11 MCP Registry schema")
        name = self.string(payload.get("name"), "name")
        self.require(bool(self.NAME.fullmatch(name)), "name must match MCP Registry reverse-DNS format")
        self.require(name == "io.github.HiroyukiFuruno/kml", "name must match the GitHub-owned server id")

        description = self.string(payload.get("description"), "description")
        self.require(0 < len(description) <= 100, "description must be 1-100 characters")
        version = self.string(payload.get("version"), "version")
        self.require(version == self.expected_version, "version must match the release version")
        self.require(bool(self.VERSION.fullmatch(version)), "version must be a specific semantic version")
        self.require("remotes" not in payload, "server.json must not claim remote MCP transport")

        repository = payload.get("repository")
        if not isinstance(repository, dict):
            self.violations.append("repository must be an object")
            return
        self.require(repository.get("source") == "github", "repository.source must be github")
        self.require(repository.get("url") == "https://github.com/HiroyukiFuruno/katana-markdown-linter", "repository.url must point to the source repository")

    def validate_packages(self, payload: dict[str, object]) -> None:
        packages = payload.get("packages")
        if not isinstance(packages, list) or not packages:
            self.violations.append("packages must be a non-empty array")
            return
        mcpb_packages = [
            package for package in packages if isinstance(package, dict) and package.get("registryType") == "mcpb"
        ]
        self.require(len(mcpb_packages) == 1, "exactly one mcpb package must be present")
        if not mcpb_packages:
            return
        package = mcpb_packages[0]
        identifier = self.string(package.get("identifier"), "packages[0].identifier")
        self.require(identifier.startswith("https://github.com/HiroyukiFuruno/katana-markdown-linter/releases/download/"), "mcpb identifier must point to the GitHub Release artifact")
        self.require("mcp" in identifier.lower(), "mcpb identifier must contain mcp")
        self.require(identifier.endswith(".mcpb"), "mcpb identifier must end with .mcpb")
        self.require(package.get("version") == self.expected_version, "mcpb package version must match the release version")
        file_sha256 = self.string(package.get("fileSha256"), "packages[0].fileSha256")
        self.require(bool(self.SHA256.fullmatch(file_sha256)), "mcpb fileSha256 must be a lowercase SHA-256 hex digest")
        self.require(file_sha256 != "0" * 64, "mcpb fileSha256 must be rendered from the release artifact")
        transport = package.get("transport")
        self.require(isinstance(transport, dict) and transport.get("type") == "stdio", "mcpb transport must be stdio")

    def string(self, value: object, label: str) -> str:
        if isinstance(value, str):
            return value
        self.violations.append(f"{label} must be a string")
        return ""

    def require(self, condition: bool, message: str) -> None:
        if not condition:
            self.violations.append(message)

    def report(self) -> int:
        if not self.violations:
            print(f"MCP server.json validation passed: {self.path}")
            return 0
        print(f"MCP server.json validation failed: {self.path}", file=sys.stderr)
        for violation in self.violations:
            print(f"- {violation}", file=sys.stderr)
        return 1


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Validate rendered MCP Registry server.json metadata.")
    parser.add_argument("--server-json", required=True, type=Path)
    parser.add_argument("--version", required=True)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    return ServerJsonValidator(args.server_json, args.version).run()


if __name__ == "__main__":
    raise SystemExit(main())
