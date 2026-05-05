#!/usr/bin/env python3
"""Verify that the requested release version follows the published release line."""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
import time
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


def github_api_request_json(url: str, args: argparse.Namespace) -> object:
    api_request = request.Request(url, headers=github_request_headers())
    for attempt in range(3):
        try:
            with request.urlopen(api_request, timeout=20) as response:
                payload = bytearray()
                while True:
                    chunk = response.read(8192)
                    if not chunk:
                        break
                    payload.extend(chunk)
                return json.loads(payload.decode("utf-8"))
        except (error.URLError, json.JSONDecodeError, UnicodeDecodeError, OSError) as release_error:
            if attempt < 2:
                time.sleep(2 ** attempt)
                continue
            raise RuntimeError(
                f"could not read latest stable GitHub Release from {args.repo}: {release_error}"
            ) from release_error


def github_release_payload(args: argparse.Namespace) -> list[object]:
    if args.github_releases_json:
        payload = json.loads(Path(args.github_releases_json).read_text(encoding="utf-8"))
        if isinstance(payload, dict):
            return [payload]
        if not isinstance(payload, list):
            raise ValueError("GitHub Releases JSON fixture must be an object or array")
        return payload

    latest_payload = github_api_request_json(
        f"https://api.github.com/repos/{args.repo}/releases/latest", args
    )
    if not isinstance(latest_payload, dict):
        raise ValueError("GitHub latest release payload must be an object")
    return [latest_payload]


def stable_version_from_tag(tag_name: object) -> StableVersion | None:
    if not isinstance(tag_name, str):
        return None
    try:
        return StableVersion.parse(tag_name)
    except ValueError:
        return None


def github_stable_version_from_payload(
    payload: object, target: StableVersion
) -> StableVersion | None:
    releases: list[StableVersion] = []
    if isinstance(payload, dict):
        payload = [payload]
    elif not isinstance(payload, list):
        raise ValueError("GitHub Releases payload must be an object or array")

    for release in payload:
        if not isinstance(release, dict):
            continue
        if release.get("draft") or release.get("prerelease"):
            continue
        version = stable_version_from_tag(release.get("tag_name"))
        if version is not None and version != target:
            releases.append(version)

    if not releases:
        return None
    return max(releases)


def latest_stable_version_from_tags(
    target: StableVersion, remote: str
) -> StableVersion | None:
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
    versions: list[StableVersion] = []
    for line in result.stdout.splitlines():
        try:
            version = StableVersion.parse(line)
        except ValueError:
            continue
        if version < target:
            versions.append(version)
    if not versions:
        return None
    return max(versions)


def latest_stable_version(target: StableVersion, args: argparse.Namespace) -> StableVersion | None:
    if args.latest_version:
        return StableVersion.parse(args.latest_version)

    try:
        payload = github_release_payload(args)
    except RuntimeError as error_from_api:
        fallback = latest_stable_version_from_tags(target, args.remote)
        if fallback is None:
            raise error_from_api
        print(
            f"Fallback to local tags for release-line resolution: {fallback.tag()} "
            "(GitHub release API unavailable).",
            file=sys.stderr,
        )
        return fallback
    latest = github_stable_version_from_payload(payload, target)
    if latest is None:
        if args.github_releases_json:
            return None
        fallback = latest_stable_version_from_tags(target, args.remote)
        if fallback is None:
            raise RuntimeError(
                "Could not resolve latest stable GitHub release; no stable payload was available"
            )
        print(
            f"Fallback to local tags for release-line resolution: {fallback.tag()} "
            "(no stable GitHub payload).",
            file=sys.stderr,
        )
        return fallback
    if not args.github_releases_json:
        if (
            not isinstance(payload, list)
            or not payload
            or not isinstance(payload[0], dict)
        ):
            raise RuntimeError("Unexpected GitHub release payload format from /releases/latest")
        if payload[0].get("draft") or payload[0].get("prerelease"):
            raise RuntimeError(
                "GitHub releases/latest is not a stable release; refuse to infer the release line"
            )
    return latest


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


def write_verification_state(target: StableVersion, blockers: list[str]) -> None:
    state_path = Path("target/release-verify-state.json")
    state_path.parent.mkdir(parents=True, exist_ok=True)

    # Simplified mock for editor artifacts state as they are checked by separate scripts
    # In a full implementation, we might collect these from the environment or specific check results.
    state = {
        "version": target.tag(),
        "editor_artifacts": {
            "vscode": {"state": os.environ.get("PUBLISH_VSCODE_EXTENSION", "deferred")},
            "zed": {"state": os.environ.get("PUBLISH_ZED_EXTENSION", "deferred")},
        },
        "publish_blockers": blockers,
        "release_decision": "allow_release" if not blockers else "stop_release",
    }
    state_path.write_text(json.dumps(state, indent=2), encoding="utf-8")
    print(f"Verification state written to {state_path}")


def main() -> int:
    args = parse_args()
    blockers = []
    try:
        target = StableVersion.parse(args.target_version)
        latest = latest_stable_version(target, args)
        result = verify(target, latest)
        if result != 0:
            blockers.append(f"Release target sanity check failed for {target.tag()}")
    except (RuntimeError, ValueError) as error:
        print(f"Release target sanity check failed: {error}", file=sys.stderr)
        target = StableVersion(0, 0, 0)  # Dummy for state write
        blockers.append(str(error))
        result = 1

    write_verification_state(target, blockers)
    return result


if __name__ == "__main__":
    raise SystemExit(main())
