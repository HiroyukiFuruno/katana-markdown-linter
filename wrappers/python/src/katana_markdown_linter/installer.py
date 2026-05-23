import hashlib
import os
import platform
import shutil
import stat
import tarfile
import tempfile
import urllib.request
import zipfile
from importlib.metadata import PackageNotFoundError, version
from pathlib import Path


class BinaryRole:
    def __init__(self, executable: str, archive_prefix: str) -> None:
        self.executable = executable
        self.archive_prefix = archive_prefix


class BinaryRoles:
    roles = {
        "cli": BinaryRole("kml", "kml"),
        "kml": BinaryRole("kml", "kml"),
        "kml-mcp": BinaryRole("kml-mcp", "kml-mcp"),
        "mcp": BinaryRole("kml-mcp", "kml-mcp"),
        "kml-mcp-remote": BinaryRole("kml-mcp-remote", "kml-mcp-remote"),
        "mcp-remote": BinaryRole("kml-mcp-remote", "kml-mcp-remote"),
    }

    @classmethod
    def resolve(cls, binary_role: str) -> BinaryRole:
        if binary_role not in cls.roles:
            raise SystemExit(f"Unsupported kml wrapper binary role: {binary_role}")
        return cls.roles[binary_role]


class KmlInstaller:
    def __init__(self, binary_role: str = "cli") -> None:
        self.role = BinaryRoles.resolve(binary_role)
        self.version = self._resolve_version()
        self.target = self._resolve_target()
        self.archive_name = self._archive_name()
        self.install_root = Path(
            os.environ.get("KML_WRAPPER_INSTALL_DIR", Path.home() / ".cache" / "kml-wrapper")
        )

    def ensure_binary(self) -> Path:
        binary_path = (
            self.install_root
            / self.version
            / self.target
            / self.role.executable
            / "bin"
            / self._binary_name()
        )
        if binary_path.is_file():
            return binary_path
        with tempfile.TemporaryDirectory(prefix="kml-python-wrapper-") as temp_dir:
            work_root = Path(temp_dir)
            archive_path = self._prepare_archive(work_root)
            self._verify_checksum(archive_path)
            extract_dir = work_root / "extract"
            extract_dir.mkdir()
            self._extract_archive(archive_path, extract_dir)
            extracted_binary = self._find_binary(extract_dir)
            binary_path.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(extracted_binary, binary_path)
            binary_path.chmod(binary_path.stat().st_mode | stat.S_IXUSR)
        return binary_path

    def _archive_name(self) -> str:
        suffix = "zip" if self.target.endswith("msvc") else "tar.gz"
        return f"{self.role.archive_prefix}-{self.version}-{self.target}.{suffix}"

    def _archive_url(self, name: str) -> str:
        repo = "HiroyukiFuruno/katana-markdown-linter"
        return f"https://github.com/{repo}/releases/download/{self.version}/{name}"

    def _binary_name(self) -> str:
        suffix = ".exe" if self.target.endswith("msvc") else ""
        return f"{self.role.executable}{suffix}"

    def _download(self, url: str, output_path: Path) -> None:
        urllib.request.urlretrieve(url, output_path)

    def _extract_archive(self, archive_path: Path, extract_dir: Path) -> None:
        if archive_path.suffix == ".zip":
            with zipfile.ZipFile(archive_path) as archive:
                archive.extractall(extract_dir)
            return
        with tarfile.open(archive_path, "r:gz") as archive:
            archive.extractall(extract_dir, filter="data")

    def _find_binary(self, extract_dir: Path) -> Path:
        for path in extract_dir.rglob(self._binary_name()):
            return path
        raise SystemExit(f"Archive does not contain {self._binary_name()}")

    def _prepare_archive(self, work_root: Path) -> Path:
        archive_dir = os.environ.get("KML_WRAPPER_ARCHIVE_DIR")
        if archive_dir:
            return Path(archive_dir) / self.archive_name
        archive_path = work_root / self.archive_name
        self._download(self._archive_url(self.archive_name), archive_path)
        self._download(self._archive_url(f"{self.archive_name}.sha256"), Path(f"{archive_path}.sha256"))
        return archive_path

    def _resolve_target(self) -> str:
        key = f"{platform.system()}/{platform.machine().lower()}"
        targets = {
            "Darwin/arm64": "aarch64-apple-darwin",
            "Darwin/aarch64": "aarch64-apple-darwin",
            "Darwin/x86_64": "x86_64-apple-darwin",
            "Linux/x86_64": "x86_64-unknown-linux-gnu",
            "Linux/amd64": "x86_64-unknown-linux-gnu",
            "Windows/amd64": "x86_64-pc-windows-msvc",
            "Windows/x86_64": "x86_64-pc-windows-msvc",
        }
        if key not in targets:
            raise SystemExit(f"Unsupported platform for kml wrapper: {key}")
        return targets[key]

    def _resolve_version(self) -> str:
        explicit_version = os.environ.get("KML_WRAPPER_VERSION")
        if explicit_version:
            return explicit_version if explicit_version.startswith("v") else f"v{explicit_version}"
        try:
            package_version = version("katana-markdown-linter")
        except PackageNotFoundError:
            package_version = "0.19.4"
        return f"v{package_version}"

    def _verify_checksum(self, archive_path: Path) -> None:
        checksum_path = Path(f"{archive_path}.sha256")
        expected = checksum_path.read_text(encoding="utf-8").split()[0]
        actual = hashlib.sha256(archive_path.read_bytes()).hexdigest()
        if actual != expected:
            raise SystemExit(f"Checksum mismatch for {self.archive_name}")
