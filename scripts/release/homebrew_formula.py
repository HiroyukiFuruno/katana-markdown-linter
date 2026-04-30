#!/usr/bin/env python3
import argparse
from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class FormulaAsset:
    target: str
    archive_name: str
    checksum: str

    def url(self, repo: str, version: str) -> str:
        return f"https://github.com/{repo}/releases/download/{version}/{self.archive_name}"


class HomebrewFormulaTool:
    def __init__(self, args: argparse.Namespace) -> None:
        self.version = self._normalize_version(args.version)
        self.repo = args.repo
        self.dist_dir = Path(args.dist_dir)
        self.output = Path(args.output)

    def generate(self) -> None:
        assets = self._load_assets()
        formula = self._render_formula(assets)
        self.output.parent.mkdir(parents=True, exist_ok=True)
        self.output.write_text(formula, encoding="utf-8")
        print(f"Generated {self.output}")

    def check(self) -> None:
        formula = self.output.read_text(encoding="utf-8")
        required = ["class Kml < Formula", "bin.install \"kml\"", "kml --version"]
        missing = [item for item in required if item not in formula]
        missing.extend(self._missing_asset_content(formula))
        if missing:
            raise SystemExit(f"Formula is missing required content: {', '.join(missing)}")
        print(f"Formula check passed for {self.output}")

    def _load_assets(self) -> list[FormulaAsset]:
        assets: list[FormulaAsset] = []
        prefix = f"kml-{self.version}-"
        for checksum_path in sorted(self.dist_dir.glob(f"{prefix}*.sha256")):
            archive_name = checksum_path.name.removesuffix(".sha256")
            if archive_name.endswith(".zip"):
                continue
            target = archive_name.removeprefix(prefix).removesuffix(".tar.gz")
            if target not in {
                "x86_64-unknown-linux-gnu",
                "x86_64-apple-darwin",
                "aarch64-apple-darwin",
            }:
                continue
            archive_path = self.dist_dir / archive_name
            if not archive_path.is_file():
                raise SystemExit(f"Formula asset is missing archive: {archive_path}")
            checksum = checksum_path.read_text(encoding="utf-8").split()[0]
            assets.append(FormulaAsset(target, archive_name, checksum))
        if not assets:
            raise SystemExit("No Homebrew-compatible binary assets were found")
        return assets

    def _missing_asset_content(self, formula: str) -> list[str]:
        missing: list[str] = []
        for asset in self._load_assets():
            for item in [asset.url(self.repo, self.version), asset.checksum]:
                if item not in formula:
                    missing.append(item)
        return missing

    def _render_formula(self, assets: list[FormulaAsset]) -> str:
        by_target = {asset.target: asset for asset in assets}
        lines = [
            "class Kml < Formula",
            "  desc \"Markdownlint-compatible Markdown linter library and CLI\"",
            "  homepage \"https://github.com/HiroyukiFuruno/katana-markdown-linter\"",
            "  license \"MIT\"",
            "",
        ]
        mac_assets = [
            by_target.get("aarch64-apple-darwin"),
            by_target.get("x86_64-apple-darwin"),
        ]
        mac_assets = [asset for asset in mac_assets if asset is not None]
        if mac_assets:
            lines.extend(self._render_macos(mac_assets))
        linux_asset = by_target.get("x86_64-unknown-linux-gnu")
        if linux_asset:
            lines.extend(self._render_linux(linux_asset))
        lines.extend(
            [
                "  def install",
                "    bin.install \"kml\"",
                "  end",
                "",
                "  test do",
                "    assert_match version.to_s, shell_output(\"#{bin}/kml --version\")",
                "  end",
                "end",
                "",
            ]
        )
        return "\n".join(lines)

    def _render_linux(self, asset: FormulaAsset) -> list[str]:
        return [
            "  on_linux do",
            "    on_intel do",
            f"      url \"{asset.url(self.repo, self.version)}\"",
            f"      sha256 \"{asset.checksum}\"",
            "    end",
            "  end",
            "",
        ]

    def _render_macos(self, assets: list[FormulaAsset]) -> list[str]:
        lines = ["  on_macos do"]
        for asset in assets:
            cpu_block = "on_arm" if asset.target.startswith("aarch64") else "on_intel"
            lines.extend(
                [
                    f"    {cpu_block} do",
                    f"      url \"{asset.url(self.repo, self.version)}\"",
                    f"      sha256 \"{asset.checksum}\"",
                    "    end",
                ]
            )
        lines.extend(["  end", ""])
        return lines

    def _normalize_version(self, version: str) -> str:
        return version if version.startswith("v") else f"v{version}"


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("command", choices=("generate", "check"))
    parser.add_argument("--version", required=True)
    parser.add_argument("--repo", default="HiroyukiFuruno/katana-markdown-linter")
    parser.add_argument("--dist-dir", default="target/binary")
    parser.add_argument("--output", default="target/homebrew/kml.rb")
    args = parser.parse_args()
    tool = HomebrewFormulaTool(args)
    if args.command == "generate":
        tool.generate()
    else:
        tool.check()


if __name__ == "__main__":
    main()
