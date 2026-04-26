from __future__ import annotations

import hashlib
import pathlib
import shutil
from dataclasses import dataclass

KATANA_PATTERNS = ("docs/**/*.md", "assets/**/*.md")


@dataclass(frozen=True)
class CorpusFile:
    path: pathlib.Path
    relative_path: str


class CorpusInventory:
    def __init__(self, root: pathlib.Path, patterns: tuple[str, ...]) -> None:
        self.root = root
        self.files = self._collect_files(patterns)
        if not self.files:
            raise SystemExit(f"no Markdown files found under {root}")

    def _collect_files(self, patterns: tuple[str, ...]) -> list[CorpusFile]:
        files: dict[str, CorpusFile] = {}
        for pattern in patterns:
            for path in sorted(self.root.glob(pattern)):
                if self._is_markdown(path):
                    relative_path = path.relative_to(self.root).as_posix()
                    files[relative_path] = CorpusFile(path, relative_path)
        return [files[key] for key in sorted(files)]

    def hashes(self) -> dict[str, str]:
        return {
            item.relative_path: hashlib.sha256(item.path.read_bytes()).hexdigest()
            for item in self.files
        }

    def report(self) -> dict[str, object]:
        return {
            "root": str(self.root),
            "file_count": len(self.files),
            "total_bytes": sum(item.path.stat().st_size for item in self.files),
            "syntax": self._syntax_counts(),
            "files": [item.relative_path for item in self.files],
        }

    def copy_to(self, target: pathlib.Path) -> list[pathlib.Path]:
        copied: list[pathlib.Path] = []
        for item in self.files:
            destination = target.joinpath(item.relative_path)
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(item.path, destination)
            copied.append(destination)
        return copied

    def _syntax_counts(self) -> dict[str, int]:
        counts = {
            "links": 0,
            "images": 0,
            "inline_html": 0,
            "fenced_code": 0,
            "tables": 0,
            "reference_definitions": 0,
            "mixed_japanese_english": 0,
        }
        for item in self.files:
            text = item.path.read_text(encoding="utf-8")
            counts["links"] += text.count("](")
            counts["images"] += text.count("![")
            counts["inline_html"] += text.count("<span") + text.count("<a ")
            counts["fenced_code"] += text.count("```") + text.count("~~~")
            counts["tables"] += text.count("| ---")
            counts["reference_definitions"] += text.count("]:")
            if self._contains_japanese_and_ascii(text):
                counts["mixed_japanese_english"] += 1
        return counts

    def _contains_japanese_and_ascii(self, text: str) -> bool:
        has_japanese = any("\u3040" <= char <= "\u9fff" for char in text)
        has_ascii = any(char.isascii() and char.isalpha() for char in text)
        return has_japanese and has_ascii

    def _is_markdown(self, path: pathlib.Path) -> bool:
        return path.is_file() and path.suffix.lower() in (".md", ".markdown")
