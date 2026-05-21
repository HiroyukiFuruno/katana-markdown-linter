#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import os
import shutil
import stat
import subprocess
import sys
import tarfile
import tempfile
import zipfile
from pathlib import Path

from binary_roles import BinaryRole, BinaryRoles, detect_target, is_windows_target


class BinaryArtifactTool:
    def __init__(self, args: argparse.Namespace) -> None:
        self.args = args
        self.role = BinaryRoles.resolve(args.executable)
        self.version = self._normalize_version(args.version)
        self.version_bare = self.version.removeprefix("v")
        self.version_base = self._version_base(self.version_bare)
        self.target = args.target or detect_target()
        self.dist_dir = Path(args.dist_dir)
        self.archive_name = self.role.archive_name(self.version, self.target)
        self.archive_path = self.dist_dir / self.archive_name
        self.checksum_path = self.dist_dir / f"{self.archive_name}.sha256"

    def package(self) -> None:
        if not self.args.skip_build:
            subprocess.run(self.role.build_command(self.target), check=True)
        binary_path = self._built_binary_path()
        if not binary_path.is_file():
            raise SystemExit(f"Expected binary was not built: {binary_path}")

        staging_root = self._staging_root()
        if staging_root.exists():
            shutil.rmtree(staging_root)
        staging_root.mkdir(parents=True)
        shutil.copy2(binary_path, staging_root / self.role.binary_name(self.target))
        if not is_windows_target(self.target):
            (staging_root / self.role.executable).chmod(0o755)
        shutil.copy2("LICENSE", staging_root / "LICENSE")
        (staging_root / "README.install.md").write_text(
            self.role.install_note(self.version), encoding="utf-8"
        )

        self.dist_dir.mkdir(parents=True, exist_ok=True)
        self.archive_path.unlink(missing_ok=True)
        self.checksum_path.unlink(missing_ok=True)
        if is_windows_target(self.target):
            self._write_zip(staging_root)
        else:
            self._write_tar(staging_root)
        self._write_checksum()
        self._write_github_output(
            {
                "archive_path": str(self.archive_path),
                "checksum_path": str(self.checksum_path),
                "archive_name": self.archive_name,
            }
        )
        print(f"Packaged {self.archive_path}")
        print(f"Checksum {self.checksum_path}")
    def smoke(self) -> None:
        archive_path = Path(self.args.archive or self.archive_path)
        checksum_path = Path(self.args.checksum or f"{archive_path}.sha256")
        self._verify_checksum(archive_path, checksum_path)
        with tempfile.TemporaryDirectory(prefix=f"{self.role.executable}-binary-smoke-") as temp_dir:
            extract_dir = Path(temp_dir) / "extract"
            extract_dir.mkdir()
            self._extract_archive(archive_path, extract_dir)
            binary = self._find_binary(extract_dir)
            self._smoke_binary(binary, Path(temp_dir))
        print(f"Binary smoke passed for {archive_path.name}")
    def _smoke_binary(self, binary: Path, temp_dir: Path) -> None:
        if self.role.smoke == "mcp-stdio":
            subprocess.run(
                ["python3", "scripts/ci/mcp-stdio-smoke.py", "--bin", str(binary)],
                check=True,
            )
            return
        if self.role.smoke == "mcp-remote":
            subprocess.run(
                ["python3", "scripts/ci/mcp-remote-smoke.py", "--bin", str(binary)],
                check=True,
            )
            return
        self._smoke_cli(binary, temp_dir)
    def _smoke_cli(self, binary: Path, temp_dir: Path) -> None:
        version_run = subprocess.run(
            [str(binary), "--version"],
            check=True,
            capture_output=True,
            text=True,
        )
        version_output = version_run.stdout.strip()
        if version_output != self.version_base:
            raise SystemExit(
                f"kml --version mismatch: expected {self.version_base}, got {version_output}"
            )
        fixture = temp_dir / "README.md"
        fixture.write_text("# Smoke\n\nText.\n", encoding="utf-8")
        subprocess.run(
            [str(binary), "check", str(fixture), "--locale", "en", "--output", "json"],
            check=True,
            stdout=subprocess.DEVNULL,
        )
    def _built_binary_path(self) -> Path:
        return Path("target") / self.target / "release" / self.role.binary_name(self.target)
    def _extract_archive(self, archive_path: Path, extract_dir: Path) -> None:
        if archive_path.suffix == ".zip":
            with zipfile.ZipFile(archive_path) as archive:
                archive.extractall(extract_dir)
            return
        with tarfile.open(archive_path, "r:gz") as archive:
            archive.extractall(extract_dir, filter="data")

    def _find_binary(self, extract_dir: Path) -> Path:
        binary_name = self.role.binary_name(self.target)
        matches = list(extract_dir.rglob(binary_name))
        if not matches:
            raise SystemExit(f"Extracted archive does not contain {binary_name}: {extract_dir}")
        binary = matches[0]
        binary.chmod(binary.stat().st_mode | stat.S_IXUSR)
        return binary

    def _normalize_version(self, version: str) -> str:
        if not version:
            raise SystemExit("version is required")
        return version if version.startswith("v") else f"v{version}"

    def _staging_root(self) -> Path:
        root_name = self.archive_name.removesuffix(".tar.gz").removesuffix(".zip")
        return self.dist_dir / "staging" / root_name

    def _verify_checksum(self, archive_path: Path, checksum_path: Path) -> None:
        expected = checksum_path.read_text(encoding="utf-8").split()[0]
        actual = hashlib.sha256(archive_path.read_bytes()).hexdigest()
        if actual != expected:
            raise SystemExit(f"Checksum mismatch for {archive_path.name}")

    def _write_checksum(self) -> None:
        digest = hashlib.sha256(self.archive_path.read_bytes()).hexdigest()
        self.checksum_path.write_text(f"{digest}  {self.archive_name}\n", encoding="utf-8")

    def _write_github_output(self, values: dict[str, str]) -> None:
        output_path = os.environ.get("GITHUB_OUTPUT")
        if not output_path:
            return
        with Path(output_path).open("a", encoding="utf-8") as output:
            for key, value in values.items():
                output.write(f"{key}={value}\n")

    def _write_tar(self, staging_root: Path) -> None:
        with tarfile.open(self.archive_path, "w:gz") as archive:
            archive.add(staging_root, arcname=staging_root.name)

    def _write_zip(self, staging_root: Path) -> None:
        with zipfile.ZipFile(self.archive_path, "w", zipfile.ZIP_DEFLATED) as archive:
            for path in staging_root.rglob("*"):
                archive.write(path, path.relative_to(staging_root.parent))

    @staticmethod
    def _version_base(version_bare: str) -> str:
        return version_bare.split("-", maxsplit=1)[0].split("+", maxsplit=1)[0]


class ArgumentParser:
    def parse(self) -> argparse.Namespace:
        parser = argparse.ArgumentParser()
        parser.add_argument("command", choices=("package", "smoke"))
        parser.add_argument("--version", required=True)
        parser.add_argument("--target")
        parser.add_argument("--dist-dir", default="target/binary")
        parser.add_argument("--archive")
        parser.add_argument("--checksum")
        parser.add_argument("--executable", default="kml", choices=BinaryRoles.canonical_names())
        parser.add_argument("--skip-build", action="store_true")
        return parser.parse_args()


def main() -> None:
    args = ArgumentParser().parse()
    tool = BinaryArtifactTool(args)
    if args.command == "package":
        tool.package()
    else:
        tool.smoke()


if __name__ == "__main__":
    try:
        main()
    except subprocess.CalledProcessError as error:
        sys.exit(error.returncode)
