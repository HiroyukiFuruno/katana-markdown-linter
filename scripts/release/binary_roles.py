#!/usr/bin/env python3
from __future__ import annotations

import platform
from dataclasses import dataclass


@dataclass(frozen=True)
class BinaryRole:
    name: str
    executable: str
    archive_prefix: str
    features: tuple[str, ...]
    smoke: str

    def archive_name(self, version: str, target: str) -> str:
        suffix = "zip" if is_windows_target(target) else "tar.gz"
        return f"{self.archive_prefix}-{version}-{target}.{suffix}"

    def binary_name(self, target: str) -> str:
        suffix = ".exe" if is_windows_target(target) else ""
        return f"{self.executable}{suffix}"

    def build_command(self, target: str) -> list[str]:
        command = [
            "cargo",
            "build",
            "--release",
            "--bin",
            self.executable,
            "--target",
            target,
            "--locked",
        ]
        if self.features:
            command.extend(["--features", ",".join(self.features)])
        return command

    def install_note(self, version: str) -> str:
        if self.name == "cli":
            return (
                f"# kml {version}\n\n"
                "Add this directory to PATH or move the `kml` executable into a "
                "directory that is already on PATH.\n"
            )
        return (
            f"# {self.executable} {version}\n\n"
            f"This archive contains the `{self.executable}` server binary used by "
            "the npm and PyPI thin wrappers.\n"
        )


class BinaryRoles:
    roles = {
        "cli": BinaryRole("cli", "kml", "kml", (), "cli"),
        "kml": BinaryRole("cli", "kml", "kml", (), "cli"),
        "kml-mcp": BinaryRole("mcp", "kml-mcp", "kml-mcp", ("mcp",), "mcp-stdio"),
        "mcp": BinaryRole("mcp", "kml-mcp", "kml-mcp", ("mcp",), "mcp-stdio"),
        "kml-mcp-remote": BinaryRole(
            "mcp-remote",
            "kml-mcp-remote",
            "kml-mcp-remote",
            ("mcp-remote",),
            "mcp-remote",
        ),
        "mcp-remote": BinaryRole(
            "mcp-remote",
            "kml-mcp-remote",
            "kml-mcp-remote",
            ("mcp-remote",),
            "mcp-remote",
        ),
    }

    @classmethod
    def resolve(cls, requested: str) -> BinaryRole:
        if requested not in cls.roles:
            supported = ", ".join(sorted(cls.roles))
            raise SystemExit(f"Unsupported executable role: {requested}. Supported: {supported}")
        return cls.roles[requested]

    @classmethod
    def canonical_names(cls) -> list[str]:
        return ["kml", "kml-mcp", "kml-mcp-remote"]


def detect_target() -> str:
    system = platform.system()
    machine = platform.machine().lower()
    if system == "Linux" and machine in {"x86_64", "amd64"}:
        return "x86_64-unknown-linux-gnu"
    if system == "Darwin" and machine in {"x86_64", "amd64"}:
        return "x86_64-apple-darwin"
    if system == "Darwin" and machine in {"arm64", "aarch64"}:
        return "aarch64-apple-darwin"
    if system == "Windows" and machine in {"amd64", "x86_64"}:
        return "x86_64-pc-windows-msvc"
    raise SystemExit(f"Unsupported host platform for binary packaging: {system} {machine}")


def is_windows_target(target: str) -> bool:
    return "windows" in target or target.endswith("msvc")
