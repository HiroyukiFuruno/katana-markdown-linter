#!/usr/bin/env python3
import argparse
import base64
import json
import os
import urllib.error
import urllib.request
from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class TapFile:
    path: str
    source: Path
    message: str


class HomebrewTapUpdater:
    def __init__(self, args: argparse.Namespace) -> None:
        self.version = self._normalize_version(args.version)
        self.version_bare = self.version.removeprefix("v")
        self.repo = args.repo
        self.branch = args.branch
        self.token = os.environ.get(args.token_env, "")
        self.current_formula = Path(args.current_formula)
        self.versioned_formula = Path(args.versioned_formula)
        self.formula_dir = args.formula_dir.rstrip("/")
        if not self.token:
            raise SystemExit(f"{args.token_env} is required to update {self.repo}.")

    def update(self) -> None:
        for tap_file in self._tap_files():
            self._put_file(tap_file)

    def _tap_files(self) -> list[TapFile]:
        versioned_name = f"kml@{self.version_bare}.rb"
        return [
            TapFile(
                path=f"{self.formula_dir}/kml.rb",
                source=self.current_formula,
                message=f"chore: update kml to {self.version_bare}",
            ),
            TapFile(
                path=f"{self.formula_dir}/{versioned_name}",
                source=self.versioned_formula,
                message=f"chore: add {versioned_name}",
            ),
        ]

    def _put_file(self, tap_file: TapFile) -> None:
        content = tap_file.source.read_text(encoding="utf-8")
        payload = {
            "branch": self.branch,
            "message": tap_file.message,
            "content": base64.b64encode(content.encode()).decode(),
        }
        current_sha = self._current_sha(tap_file.path)
        if current_sha:
            payload["sha"] = current_sha
        response = self._request("PUT", self._contents_url(tap_file.path), payload)
        html_url = response.get("content", {}).get("html_url", tap_file.path)
        print(f"Updated {html_url}")

    def _current_sha(self, path: str) -> str:
        try:
            response = self._request("GET", f"{self._contents_url(path)}?ref={self.branch}")
        except urllib.error.HTTPError as error:
            if error.code == 404:
                return ""
            raise
        return str(response.get("sha", ""))

    def _request(self, method: str, url: str, payload: dict | None = None) -> dict:
        data = None if payload is None else json.dumps(payload).encode()
        request = urllib.request.Request(url, data=data, method=method)
        request.add_header("Accept", "application/vnd.github+json")
        request.add_header("Authorization", f"Bearer {self.token}")
        request.add_header("User-Agent", "katana-markdown-linter-release")
        request.add_header("X-GitHub-Api-Version", "2022-11-28")
        with urllib.request.urlopen(request, timeout=30) as response:
            return json.load(response)

    def _contents_url(self, path: str) -> str:
        return f"https://api.github.com/repos/{self.repo}/contents/{path}"

    def _normalize_version(self, version: str) -> str:
        return version if version.startswith("v") else f"v{version}"


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--version", required=True)
    parser.add_argument("--repo", default="HiroyukiFuruno/homebrew-katana")
    parser.add_argument("--branch", default="master")
    parser.add_argument("--formula-dir", default="Formula")
    parser.add_argument("--current-formula", required=True)
    parser.add_argument("--versioned-formula", required=True)
    parser.add_argument("--token-env", default="HOMEBREW_KATANA_GIT_TOKEN")
    HomebrewTapUpdater(parser.parse_args()).update()


if __name__ == "__main__":
    main()
