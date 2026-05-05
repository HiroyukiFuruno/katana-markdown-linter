#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import sys
import tomllib
from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class VersionCheck:
    path: Path
    label: str
    actual: str
    expected: str

    def violation(self) -> str | None:
        if self.actual == self.expected:
            return None
        return f"{self.path}:{self.label} is {self.actual}, expected {self.expected}"


class VersionSyncVerifier:
    def __init__(self, root: Path, expected_version: str | None) -> None:
        self.root = root
        self.cargo_version = self._read_toml(Path("Cargo.toml"))["package"]["version"]
        self.expected_version = self._normalize_version(expected_version or self.cargo_version)

    def verify(self) -> list[str]:
        violations = [
            violation
            for violation in (check.violation() for check in self._version_checks())
            if violation is not None
        ]
        violations.extend(self._server_identifier_violations())
        violations.extend(self._changelog_violations())
        return violations

    def _version_checks(self) -> list[VersionCheck]:
        cargo_lock = self._read_toml(Path("Cargo.lock"))
        mcpb_manifest = self._read_json(Path("mcpb/manifest.json"))
        server_json = self._read_json(Path("server.json"))
        npm_package = self._read_json(Path("wrappers/npm/package.json"))
        pypi_project = self._read_toml(Path("wrappers/python/pyproject.toml"))
        vscode_package = self._read_json(Path("editors/vscode/package.json"))
        vscode_lock = self._read_json(Path("editors/vscode/package-lock.json"))
        zed_extension = self._read_toml(Path("editors/zed/extension.toml"))
        zed_cargo = self._read_toml(Path("editors/zed/Cargo.toml"))
        zed_lock = self._read_toml(Path("editors/zed/Cargo.lock"))
        mcpb_package = self._mcpb_package(server_json)
        workspace_lock = self._cargo_lock_package(cargo_lock)
        zed_lock_package = self._cargo_lock_package(zed_lock, "katana-markdown-linter-zed")

        return [
            VersionCheck(Path("Cargo.toml"), "package.version", self.cargo_version, self.expected_version),
            VersionCheck(Path("Cargo.lock"), "katana-markdown-linter.version", workspace_lock["version"], self.expected_version),
            VersionCheck(Path("mcpb/manifest.json"), "version", mcpb_manifest["version"], self.expected_version),
            VersionCheck(Path("server.json"), "version", server_json["version"], self.expected_version),
            VersionCheck(Path("server.json"), "packages[mcpb].version", mcpb_package["version"], self.expected_version),
            VersionCheck(Path("wrappers/npm/package.json"), "version", npm_package["version"], self.expected_version),
            VersionCheck(Path("wrappers/python/pyproject.toml"), "project.version", pypi_project["project"]["version"], self.expected_version),
            VersionCheck(Path("editors/vscode/package.json"), "version", vscode_package["version"], self.expected_version),
            VersionCheck(Path("editors/vscode/package-lock.json"), "version", vscode_lock["version"], self.expected_version),
            VersionCheck(Path("editors/vscode/package-lock.json"), "packages.root.version", vscode_lock["packages"][""]["version"], self.expected_version),
            VersionCheck(Path("editors/zed/extension.toml"), "version", zed_extension["version"], self.expected_version),
            VersionCheck(Path("editors/zed/Cargo.toml"), "package.version", zed_cargo["package"]["version"], self.expected_version),
            VersionCheck(Path("editors/zed/Cargo.lock"), "katana-markdown-linter-zed.version", zed_lock_package["version"], self.expected_version),
        ]

    def _server_identifier_violations(self) -> list[str]:
        server_json = self._read_json(Path("server.json"))
        package = self._mcpb_package(server_json)
        expected = (
            "https://github.com/HiroyukiFuruno/katana-markdown-linter/releases/download/"
            f"v{self.expected_version}/katana-markdown-linter-{self.expected_version}.mcpb"
        )
        if package["identifier"] == expected:
            return []
        return [f"server.json:packages[mcpb].identifier is {package['identifier']}, expected {expected}"]

    def _changelog_violations(self) -> list[str]:
        changelog = self._read_text(Path("CHANGELOG.md"))
        expected_heading = f"## v{self.expected_version}"
        if expected_heading in changelog:
            return []
        return [f"CHANGELOG.md is missing {expected_heading}"]

    def _cargo_lock_package(self, cargo_lock: dict, package_name: str = "katana-markdown-linter") -> dict:
        for package in cargo_lock["package"]:
            if package["name"] == package_name:
                return package
        raise ValueError(f"Cargo.lock does not contain {package_name}")

    def _mcpb_package(self, server_json: dict) -> dict:
        for package in server_json["packages"]:
            if package["registryType"] == "mcpb":
                return package
        raise ValueError("server.json does not contain an mcpb package")

    def _read_toml(self, path: Path) -> dict:
        return tomllib.loads(self._read_text(path))

    def _read_json(self, path: Path) -> dict:
        return json.loads(self._read_text(path))

    def _read_text(self, path: Path) -> str:
        return (self.root / path).read_text(encoding="utf-8")

    def _normalize_version(self, version: str) -> str:
        return version.removeprefix("v")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Verify release-facing package versions stay synchronized.")
    parser.add_argument("--root", default=Path("."), type=Path)
    parser.add_argument("--version")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    verifier = VersionSyncVerifier(args.root, args.version)
    violations = verifier.verify()
    if violations:
        print("Release version metadata is not synchronized:", file=sys.stderr)
        for violation in violations:
            print(f"- {violation}", file=sys.stderr)
        return 1
    print(f"Release version metadata is synchronized: {verifier.expected_version}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
