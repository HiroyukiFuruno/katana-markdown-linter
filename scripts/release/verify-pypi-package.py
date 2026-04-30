#!/usr/bin/env python3
from __future__ import annotations

import argparse
import shutil
import subprocess
import sys
import tarfile
import tempfile
import tomllib
import venv
import zipfile
from pathlib import Path


class PyPIPackageVerifier:
    def __init__(self, package_root: Path) -> None:
        self.package_root = package_root.resolve()
        self.pyproject_path = self.package_root / "pyproject.toml"
        self.readme_path = self.package_root / "README.md"
        self.target_root = Path("target/pypi-package-check").resolve()
        self.venv_root = self.target_root / "venv"
        self.dist_root = self.target_root / "dist"
        self.pyproject = self.load_pyproject()
        self.project = self.pyproject["project"]

    def run(self) -> None:
        try:
            self.verify_metadata()
            self.verify_readme()
            self.build_distributions()
            self.verify_distributions()
        finally:
            self.clean_source_build_artifacts()
        print(
            "PyPI package check passed for "
            f"{self.project['name']}@{self.project['version']}"
        )

    def load_pyproject(self) -> dict[str, object]:
        with self.pyproject_path.open("rb") as pyproject_file:
            return tomllib.load(pyproject_file)

    def verify_metadata(self) -> None:
        for field in ["name", "version", "description", "readme", "requires-python", "license"]:
            self.require_non_empty_string(self.project, field)

        if self.project["readme"] != "README.md":
            raise AssertionError("pyproject.toml: readme must point to README.md")

        dependencies = self.project.get("dependencies", [])
        if dependencies:
            raise AssertionError("pyproject.toml: runtime dependencies must stay empty")

        for field in ["keywords", "classifiers"]:
            values = self.project.get(field)
            if not isinstance(values, list) or not values:
                raise AssertionError(f"pyproject.toml: {field} must be a non-empty list")

        urls = self.project.get("urls")
        if not isinstance(urls, dict):
            raise AssertionError("pyproject.toml: project.urls is required")
        for field in ["Homepage", "Repository", "Issues", "Changelog"]:
            self.require_non_empty_string(urls, field)

    def verify_readme(self) -> None:
        readme = self.readme_path.read_text(encoding="utf-8")
        required_fragments = [
            "downloads the matching `kml` binary archive from GitHub Releases",
            "SHA-256 checksum",
            "## Install",
            "## Supported Platforms",
            "## Wrapper Contract",
            "uvx --from katana-markdown-linter==",
        ]
        for fragment in required_fragments:
            if fragment not in readme:
                raise AssertionError(f"wrappers/python/README.md is missing `{fragment}`")

    def build_distributions(self) -> None:
        python = self.ensure_build_environment()
        if self.dist_root.exists():
            shutil.rmtree(self.dist_root)
        self.dist_root.mkdir(parents=True)
        subprocess.run(
            [
                str(python),
                "-m",
                "build",
                "--sdist",
                "--wheel",
                "--outdir",
                str(self.dist_root),
                str(self.package_root),
            ],
            check=True,
        )

    def ensure_build_environment(self) -> Path:
        python = self.venv_python()
        if not python.exists():
            self.venv_root.parent.mkdir(parents=True, exist_ok=True)
            venv.EnvBuilder(with_pip=True).create(self.venv_root)
            python = self.venv_python()

        subprocess.run(
            [str(python), "-m", "pip", "install", "--upgrade", "build"],
            check=True,
        )
        return python

    def verify_distributions(self) -> None:
        sdists = sorted(self.dist_root.glob("*.tar.gz"))
        wheels = sorted(self.dist_root.glob("*.whl"))
        if len(sdists) != 1 or len(wheels) != 1:
            raise AssertionError("PyPI build must produce exactly one sdist and one wheel")

        sdist_names = self.read_sdist_names(sdists[0])
        wheel_names = self.read_wheel_names(wheels[0])
        for required in ["README.md", "pyproject.toml"]:
            if not any(name.endswith(f"/{required}") for name in sdist_names):
                raise AssertionError(f"sdist is missing {required}")

        metadata_name = next(
            (name for name in wheel_names if name.endswith(".dist-info/METADATA")),
            None,
        )
        if metadata_name is None:
            raise AssertionError("wheel is missing METADATA")
        self.verify_wheel_metadata(wheels[0], metadata_name)

    def verify_wheel_metadata(self, wheel_path: Path, metadata_name: str) -> None:
        with zipfile.ZipFile(wheel_path) as wheel:
            metadata = wheel.read(metadata_name).decode("utf-8")
        required_fragments = [
            "Description-Content-Type: text/markdown",
            "Project-URL: Homepage, https://github.com/HiroyukiFuruno/katana-markdown-linter",
            "downloads the matching `kml` binary archive from GitHub Releases",
            "## Supported Platforms",
        ]
        for fragment in required_fragments:
            if fragment not in metadata:
                raise AssertionError(f"wheel METADATA is missing `{fragment}`")

    def clean_source_build_artifacts(self) -> None:
        for path in [
            self.package_root / "build",
            self.package_root / "src" / "katana_markdown_linter.egg-info",
        ]:
            if path.exists():
                shutil.rmtree(path)

    def read_sdist_names(self, sdist_path: Path) -> set[str]:
        with tarfile.open(sdist_path, "r:gz") as sdist:
            return set(sdist.getnames())

    def read_wheel_names(self, wheel_path: Path) -> set[str]:
        with zipfile.ZipFile(wheel_path) as wheel:
            return set(wheel.namelist())

    def venv_python(self) -> Path:
        posix_python = self.venv_root / "bin" / "python"
        if posix_python.exists():
            return posix_python
        return self.venv_root / "Scripts" / "python.exe"

    def require_non_empty_string(self, metadata: object, field: str) -> None:
        if not isinstance(metadata, dict):
            raise AssertionError("metadata must be a table")
        value = metadata.get(field)
        if not isinstance(value, str) or not value.strip():
            raise AssertionError(f"pyproject.toml: {field} must be a non-empty string")


class ArgumentParser:
    def parse(self) -> argparse.Namespace:
        parser = argparse.ArgumentParser()
        parser.add_argument("package_root", nargs="?", default="wrappers/python")
        return parser.parse_args()


def main() -> int:
    args = ArgumentParser().parse()
    PyPIPackageVerifier(Path(args.package_root)).run()
    return 0


if __name__ == "__main__":
    sys.exit(main())
