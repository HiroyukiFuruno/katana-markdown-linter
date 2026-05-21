#!/usr/bin/env python3
from __future__ import annotations

import subprocess
import tarfile
import venv
import zipfile
from pathlib import Path


class PyPIPackageBuildEnvironment:
    def __init__(self, venv_root: Path) -> None:
        self.venv_root = venv_root

    def ensure(self) -> Path:
        python = self.python()
        if not python.exists():
            self.venv_root.parent.mkdir(parents=True, exist_ok=True)
            venv.EnvBuilder(with_pip=True).create(self.venv_root)
            python = self.python()

        subprocess.run([str(python), "-m", "pip", "install", "--upgrade", "build"], check=True)
        return python

    def python(self) -> Path:
        posix_python = self.venv_root / "bin" / "python"
        if posix_python.exists():
            return posix_python
        return self.venv_root / "Scripts" / "python.exe"


class PyPIPackageDistribution:
    def __init__(self, dist_root: Path) -> None:
        self.dist_root = dist_root

    def require_pair(self) -> tuple[Path, Path]:
        sdists = sorted(self.dist_root.glob("*.tar.gz"))
        wheels = sorted(self.dist_root.glob("*.whl"))
        if len(sdists) != 1 or len(wheels) != 1:
            raise AssertionError("PyPI build must produce exactly one sdist and one wheel")
        return sdists[0], wheels[0]

    def read_sdist_names(self, sdist_path: Path) -> set[str]:
        with tarfile.open(sdist_path, "r:gz") as sdist:
            return set(sdist.getnames())

    def read_wheel_names(self, wheel_path: Path) -> set[str]:
        with zipfile.ZipFile(wheel_path) as wheel:
            return set(wheel.namelist())

    def read_wheel_text(self, wheel_path: Path, name: str) -> str:
        with zipfile.ZipFile(wheel_path) as wheel:
            return wheel.read(name).decode("utf-8")
