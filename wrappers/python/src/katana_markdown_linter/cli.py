import subprocess
import sys

from katana_markdown_linter.installer import KmlInstaller


class KmlCli:
    def __init__(self, arguments: list[str]) -> None:
        self.arguments = arguments

    def run(self) -> int:
        binary_path = KmlInstaller().ensure_binary()
        return subprocess.run([str(binary_path), *self.arguments], check=False).returncode


def main() -> None:
    raise SystemExit(KmlCli(sys.argv[1:]).run())
