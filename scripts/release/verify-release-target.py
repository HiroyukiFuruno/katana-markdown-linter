#!/usr/bin/env python3
"""Verify that the requested release version follows the published release line."""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from urllib import error, request


@dataclass(frozen=True, order=True)
class StableVersion:
    major: int
    minor: int
    patch: int

    @classmethod
    def parse(cls, value: str) -> "StableVersion":
        # Allow versions with suffixes (e.g. v0.18.0-id) by splitting at hyphen or plus
        base_version = value.strip().split("-")[0].split("+")[0]
        match = re.fullmatch(r"v?(\d+)\.(\d+)\.(\d+)", base_version)
        if match is None:
            raise ValueError(f"expected a stable version like v1.2.3, got {value!r}")
        return cls(*(int(group) for group in match.groups()))

    def tag(self) -> str:
        return f"v{self.major}.{self.minor}.{self.patch}"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--target-version", required=True, help="Release version such as v0.17.7")
    parser.add_argument("--latest-version", help="Override latest stable version for tests")
    parser.add_argument(
        "--repo",
        default="HiroyukiFuruno/katana-markdown-linter",
        help="GitHub repository used to resolve published stable releases",
    )
    parser.add_argument(
        "--github-releases-json",
        help="Read a GitHub Releases API JSON fixture instead of calling GitHub",
    )
    parser.add_argument("--remote", default="origin", help="Git remote used when fetching tags")
    return parser.parse_args()


def git_tags(remote: str) -> list[str]:
    subprocess.run(
        ["git", "fetch", "--quiet", "--tags", remote],
        check=False,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    result = subprocess.run(
        ["git", "tag", "--list", "v[0-9]*.[0-9]*.[0-9]*"],
        check=True,
        text=True,
        stdout=subprocess.PIPE,
    )
    return [line.strip() for line in result.stdout.splitlines() if line.strip()]


def github_request_headers() -> dict[str, str]:
    headers = {
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
        "User-Agent": "katana-markdown-linter-release-target-check",
    }
    token = os.environ.get("GITHUB_TOKEN") or os.environ.get("GH_TOKEN")
    if token:
        headers["Authorization"] = f"Bearer {token}"
    return headers


def github_release_payload(args: argparse.Namespace) -> list[object]:
    if args.github_releases_json:
        payload = json.loads(Path(args.github_releases_json).read_text(encoding="utf-8"))
        if not isinstance(payload, list):
            raise ValueError("GitHub Releases JSON fixture must be an array")
        return payload

    releases: list[object] = []
    for page in range(1, 11):
        url = f"https://api.github.com/repos/{args.repo}/releases?per_page=100&page={page}"
        api_request = request.Request(url, headers=github_request_headers())
        try:
            with request.urlopen(api_request, timeout=20) as response:
                page_payload = json.loads(response.read().decode("utf-8"))
        except error.URLError as release_error:
            raise RuntimeError(
                f"could not read latest stable GitHub Release from {args.repo}: {release_error}"
            ) from release_error
        if not isinstance(page_payload, list):
            raise ValueError("GitHub Releases API response must be an array")
        releases.extend(page_payload)
        if len(page_payload) < 100:
            break
    return releases


def github_stable_versions(target: StableVersion, args: argparse.Namespace) -> list[StableVersion]:
    versions: list[StableVersion] = []
    for release in github_release_payload(args):
        if not isinstance(release, dict):
            continue
        if release.get("draft") or release.get("prerelease"):
            continue
        tag_name = release.get("tag_name")
        if not isinstance(tag_name, str):
            continue
        try:
            version = StableVersion.parse(tag_name)
        except ValueError:
            continue
        if version != target:
            versions.append(version)
    return versions


def latest_tag_version(target: StableVersion, remote: str) -> StableVersion | None:
    versions = []
    for tag in git_tags(remote):
        try:
            version = StableVersion.parse(tag)
        except ValueError:
            continue
        if version != target:
            versions.append(version)
    return max(versions) if versions else None


def latest_stable_version(target: StableVersion, args: argparse.Namespace) -> StableVersion | None:
    if args.latest_version:
        return StableVersion.parse(args.latest_version)

    github_versions = github_stable_versions(target, args)
    if github_versions:
        return max(github_versions)
    return latest_tag_version(target, args.remote)


def fail(message: str) -> int:
    print(f"Release target sanity check failed: {message}", file=sys.stderr)
    print(
        "If this is an intentional corrective release, stop and get explicit user confirmation. "
        "Then rerun with KML_RELEASE_ALLOW_VERSION_LINE_OVERRIDE=1 and document the reason.",
        file=sys.stderr,
    )
    return 1


def verify(target: StableVersion, latest: StableVersion | None) -> int:
    if os.environ.get("KML_RELEASE_ALLOW_VERSION_LINE_OVERRIDE") == "1":
        print("Release target sanity check override is enabled.")
        return 0
    if latest is None:
        print(f"No previous stable release tag found; accepting {target.tag()}.")
        return 0
    if target <= latest:
        return fail(f"{target.tag()} is not newer than latest stable release {latest.tag()}.")
    if target.major == latest.major and target.minor == latest.minor:
        expected = StableVersion(latest.major, latest.minor, latest.patch + 1)
        if target == expected:
            print(f"Release target sanity check passed: {latest.tag()} -> {target.tag()}.")
            return 0
        return fail(
            f"patch releases must be consecutive: expected {expected.tag()} after {latest.tag()}, "
            f"got {target.tag()}."
        )
    if target.major == latest.major and target.minor == latest.minor + 1:
        expected = StableVersion(latest.major, latest.minor + 1, 0)
        if target == expected:
            print(f"Release target sanity check passed: {latest.tag()} -> {target.tag()}.")
            return 0
        return fail(
            f"a new minor line must start at {expected.tag()} after {latest.tag()}, "
            f"got {target.tag()}."
        )
    if target.major == latest.major + 1:
        expected = StableVersion(latest.major + 1, 0, 0)
        if target == expected:
            print(f"Release target sanity check passed: {latest.tag()} -> {target.tag()}.")
            return 0
        return fail(
            f"a new major line must start at {expected.tag()} after {latest.tag()}, "
            f"got {target.tag()}."
        )
    return fail(f"{target.tag()} skips over latest stable release {latest.tag()}.")


def main() -> int:
    args = parse_args()
    try:
        target = StableVersion.parse(args.target_version)
        latest = latest_stable_version(target, args)
    except (RuntimeError, ValueError, subprocess.CalledProcessError) as error:
        print(f"Release target sanity check failed: {error}", file=sys.stderr)
        return 1
    return verify(target, latest)


if __name__ == "__main__":
    raise SystemExit(main())
