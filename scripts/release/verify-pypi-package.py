#!/usr/bin/env python3
from __future__ import annotations

import argparse
import shutil
import subprocess
import sys
import tomllib
from pathlib import Path

from pypi_package_build import PyPIPackageBuildEnvironment, PyPIPackageDistribution


class PyPIPackageVerifier:
    def __init__(self, package_root: Path) -> None:
        self.package_root = package_root.resolve()
        self.pyproject_path = self.package_root / "pyproject.toml"
        self.readme_path = self.package_root / "README.md"
        self.target_root = Path("target/pypi-package-check").resolve()
        self.venv_root = self.target_root / "venv"
        self.dist_root = self.target_root / "dist"
        self.build_environment = PyPIPackageBuildEnvironment(self.venv_root)
        self.distribution = PyPIPackageDistribution(self.dist_root)
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

        scripts = self.project.get("scripts")
        if not isinstance(scripts, dict):
            raise AssertionError("pyproject.toml: project.scripts is required")
        expected_scripts = {
            "kml": "katana_markdown_linter.cli:main",
            "kml-mcp": "katana_markdown_linter.cli:main_mcp",
            "kml-mcp-remote": "katana_markdown_linter.cli:main_mcp_remote",
        }
        for name, target in expected_scripts.items():
            if scripts.get(name) != target:
                raise AssertionError(f"pyproject.toml: script {name} must be {target}")

    def verify_readme(self) -> None:
        readme = self.readme_path.read_text(encoding="utf-8")
        required_fragments = [
            "downloads the matching `kml`, `kml-mcp`, or `kml-mcp-remote`",
            "SHA-256 checksum",
            "## Install",
            "## MCP server entrypoints",
            "## Supported Platforms",
            "## Wrapper Contract",
            "uvx --from katana-markdown-linter==",
            "kml-mcp --workspace-root",
            "kml-mcp-remote",
        ]
        for fragment in required_fragments:
            if fragment not in readme:
                raise AssertionError(f"wrappers/python/README.md is missing `{fragment}`")

    def build_distributions(self) -> None:
        python = self.build_environment.ensure()
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

    def verify_distributions(self) -> None:
        sdist_path, wheel_path = self.distribution.require_pair()
        sdist_names = self.distribution.read_sdist_names(sdist_path)
        wheel_names = self.distribution.read_wheel_names(wheel_path)
        for required in ["README.md", "pyproject.toml"]:
            if not any(name.endswith(f"/{required}") for name in sdist_names):
                raise AssertionError(f"sdist is missing {required}")

        metadata_name = next(
            (name for name in wheel_names if name.endswith(".dist-info/METADATA")),
            None,
        )
        if metadata_name is None:
            raise AssertionError("wheel is missing METADATA")
        self.verify_wheel_metadata(wheel_path, metadata_name)
        entry_points_name = next(
            (name for name in wheel_names if name.endswith(".dist-info/entry_points.txt")),
            None,
        )
        if entry_points_name is None:
            raise AssertionError("wheel is missing entry_points.txt")
        self.verify_entry_points(wheel_path, entry_points_name)

    def verify_wheel_metadata(self, wheel_path: Path, metadata_name: str) -> None:
        metadata = self.distribution.read_wheel_text(wheel_path, metadata_name)
        required_fragments = [
            "Description-Content-Type: text/markdown",
            "Project-URL: Homepage, https://github.com/HiroyukiFuruno/katana-markdown-linter",
            "downloads the matching `kml`, `kml-mcp`, or `kml-mcp-remote`",
            "kml-mcp --workspace-root",
            "## Supported Platforms",
        ]
        for fragment in required_fragments:
            if fragment not in metadata:
                raise AssertionError(f"wheel METADATA is missing `{fragment}`")

    def verify_entry_points(self, wheel_path: Path, entry_points_name: str) -> None:
        entry_points = self.distribution.read_wheel_text(wheel_path, entry_points_name)
        required_fragments = [
            "kml = katana_markdown_linter.cli:main",
            "kml-mcp = katana_markdown_linter.cli:main_mcp",
            "kml-mcp-remote = katana_markdown_linter.cli:main_mcp_remote",
        ]
        for fragment in required_fragments:
            if fragment not in entry_points:
                raise AssertionError(f"wheel entry_points.txt is missing `{fragment}`")

    def clean_source_build_artifacts(self) -> None:
        for path in [
            self.package_root / "build",
            self.package_root / "src" / "katana_markdown_linter.egg-info",
        ]:
            if path.exists():
                shutil.rmtree(path)

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
