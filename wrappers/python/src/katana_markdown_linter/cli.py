import subprocess
import sys

from katana_markdown_linter.installer import KmlInstaller


class KmlCli:
    def __init__(self, binary_role: str, arguments: list[str]) -> None:
        self.binary_role = binary_role
        self.arguments = arguments

    def run(self) -> int:
        binary_path = KmlInstaller(self.binary_role).ensure_binary()
        return subprocess.run([str(binary_path), *self.arguments], check=False).returncode


def main() -> None:
    raise SystemExit(KmlCli("kml", sys.argv[1:]).run())


def main_mcp() -> None:
    raise SystemExit(KmlCli("kml-mcp", sys.argv[1:]).run())


def main_mcp_remote() -> None:
    raise SystemExit(KmlCli("kml-mcp-remote", sys.argv[1:]).run())
