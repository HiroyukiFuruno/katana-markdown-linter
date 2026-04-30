#!/usr/bin/env python3
import argparse
import hashlib
import os
import platform
import shutil
import stat
import subprocess
import sys
import tarfile
import tempfile
import zipfile
from pathlib import Path


class BinaryArtifactTool:
    def __init__(self, args: argparse.Namespace) -> None:
        self.args = args
        self.version = self._normalize_version(args.version)
        self.version_bare = self.version.removeprefix("v")
        self.target = args.target or self._detect_target()
        self.dist_dir = Path(args.dist_dir)
        self.archive_name = self._archive_name()
        self.archive_path = self.dist_dir / self.archive_name
        self.checksum_path = self.dist_dir / f"{self.archive_name}.sha256"

    def package(self) -> None:
        if not self.args.skip_build:
            subprocess.run(
                ["cargo", "build", "--release", "--bin", "kml", "--target", self.target, "--locked"],
                check=True,
            )
        binary_path = self._built_binary_path()
        if not binary_path.is_file():
            raise SystemExit(f"Expected binary was not built: {binary_path}")
        staging_root = self.dist_dir / "staging" / self.archive_name.removesuffix(".tar.gz").removesuffix(".zip")
        if staging_root.exists():
            shutil.rmtree(staging_root)
        staging_root.mkdir(parents=True)
        shutil.copy2(binary_path, staging_root / self._binary_name())
        if not self._is_windows_target():
            (staging_root / "kml").chmod(0o755)
        shutil.copy2("LICENSE", staging_root / "LICENSE")
        (staging_root / "README.install.md").write_text(
            self._install_note(), encoding="utf-8"
        )
        self.dist_dir.mkdir(parents=True, exist_ok=True)
        self.archive_path.unlink(missing_ok=True)
        self.checksum_path.unlink(missing_ok=True)
        if self._is_windows_target():
            self._write_zip(staging_root)
        else:
            self._write_tar(staging_root)
        self._write_checksum()
        self._write_github_output({
            "archive_path": str(self.archive_path),
            "checksum_path": str(self.checksum_path),
            "archive_name": self.archive_name,
        })
        print(f"Packaged {self.archive_path}")
        print(f"Checksum {self.checksum_path}")

    def smoke(self) -> None:
        archive_path = Path(self.args.archive or self.archive_path)
        checksum_path = Path(self.args.checksum or f"{archive_path}.sha256")
        self._verify_checksum(archive_path, checksum_path)
        with tempfile.TemporaryDirectory(prefix="kml-binary-smoke-") as temp_dir:
            extract_dir = Path(temp_dir) / "extract"
            extract_dir.mkdir()
            self._extract_archive(archive_path, extract_dir)
            binary = self._find_binary(extract_dir)
            version_run = subprocess.run([str(binary), "--version"], check=True, capture_output=True, text=True)
            version_output = version_run.stdout.strip()
            if version_output != self.version_bare:
                raise SystemExit(f"kml --version mismatch: expected {self.version_bare}, got {version_output}")
            fixture = Path(temp_dir) / "README.md"
            fixture.write_text("# Smoke\n\nText.\n", encoding="utf-8")
            subprocess.run(
                [str(binary), "check", str(fixture), "--locale", "en", "--output", "json"],
                check=True,
                stdout=subprocess.DEVNULL,
            )
        print(f"Binary smoke passed for {archive_path.name}")

    def _archive_name(self) -> str:
        suffix = "zip" if self._is_windows_target() else "tar.gz"
        return f"kml-{self.version}-{self.target}.{suffix}"

    def _binary_name(self) -> str:
        return "kml.exe" if self._is_windows_target() else "kml"

    def _built_binary_path(self) -> Path:
        return Path("target") / self.target / "release" / self._binary_name()

    def _detect_target(self) -> str:
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
        raise SystemExit(
            f"Unsupported host platform for binary packaging: {system} {machine}"
        )

    def _extract_archive(self, archive_path: Path, extract_dir: Path) -> None:
        if archive_path.suffix == ".zip":
            with zipfile.ZipFile(archive_path) as archive:
                archive.extractall(extract_dir)
            return
        with tarfile.open(archive_path, "r:gz") as archive:
            archive.extractall(extract_dir, filter="data")

    def _find_binary(self, extract_dir: Path) -> Path:
        for name in ("kml", "kml.exe"):
            matches = list(extract_dir.rglob(name))
            if matches:
                binary = matches[0]
                binary.chmod(binary.stat().st_mode | stat.S_IXUSR)
                return binary
        raise SystemExit(f"Extracted archive does not contain kml: {extract_dir}")

    def _install_note(self) -> str:
        return (
            f"# kml {self.version}\n\n"
            "Add this directory to PATH or move the `kml` executable into a "
            "directory that is already on PATH.\n"
        )

    def _is_windows_target(self) -> bool:
        return "windows" in self.target or self.target.endswith("msvc")

    def _normalize_version(self, version: str) -> str:
        if not version:
            raise SystemExit("version is required")
        return version if version.startswith("v") else f"v{version}"

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


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("command", choices=("package", "smoke"))
    parser.add_argument("--version", required=True)
    parser.add_argument("--target")
    parser.add_argument("--dist-dir", default="target/binary")
    parser.add_argument("--archive")
    parser.add_argument("--checksum")
    parser.add_argument("--skip-build", action="store_true")
    args = parser.parse_args()
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
