#!/usr/bin/env python3
"""Verify that the requested release version follows the published release line."""

from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
from dataclasses import dataclass


@dataclass(frozen=True, order=True)
class StableVersion:
    major: int
    minor: int
    patch: int

    @classmethod
    def parse(cls, value: str) -> "StableVersion":
        match = re.fullmatch(r"v?(\d+)\.(\d+)\.(\d+)", value.strip())
        if match is None:
            raise ValueError(f"expected a stable version like v1.2.3, got {value!r}")
        return cls(*(int(group) for group in match.groups()))

    def tag(self) -> str:
        return f"v{self.major}.{self.minor}.{self.patch}"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--target-version", required=True, help="Release version such as v0.17.7")
    parser.add_argument("--latest-version", help="Override latest stable version for tests")
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


def latest_stable_version(target: StableVersion, args: argparse.Namespace) -> StableVersion | None:
    if args.latest_version:
        return StableVersion.parse(args.latest_version)

    versions = []
    for tag in git_tags(args.remote):
        try:
            version = StableVersion.parse(tag)
        except ValueError:
            continue
        if version != target:
            versions.append(version)
    return max(versions) if versions else None


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
    except (ValueError, subprocess.CalledProcessError) as error:
        print(f"Release target sanity check failed: {error}", file=sys.stderr)
        return 1
    return verify(target, latest)


if __name__ == "__main__":
    raise SystemExit(main())
