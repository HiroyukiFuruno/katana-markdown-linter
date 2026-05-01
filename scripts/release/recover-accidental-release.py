#!/usr/bin/env python3
"""Plan or run non-destructive recovery for an accidental release version."""

from __future__ import annotations

import argparse
import json
import os
import re
import shlex
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
            raise ValueError(f"expected stable version like v1.2.3, got {value!r}")
        return cls(*(int(group) for group in match.groups()))

    def bare(self) -> str:
        return f"{self.major}.{self.minor}.{self.patch}"

    def tag(self) -> str:
        return f"v{self.bare()}"


@dataclass(frozen=True)
class RecoveryStep:
    title: str
    command: list[str]
    manual: bool = False


class AccidentalReleaseRecovery:
    def __init__(self, args: argparse.Namespace) -> None:
        self.args = args
        self.bad_version = StableVersion.parse(args.bad_version)
        self.latest_version = self._latest_prior_version()
        replacement = args.replacement_version or self.latest_version.tag()
        self.replacement_version = StableVersion.parse(replacement)
        if self.replacement_version >= self.bad_version:
            raise SystemExit("replacement version must be lower than the accidental version.")

    def run(self) -> int:
        sanity_error = self._target_sanity_error(self.bad_version)
        if sanity_error == "":
            print(
                f"{self.bad_version.tag()} is not flagged as a suspicious release target.",
                file=sys.stderr,
            )
            return 1
        steps = self._steps()
        self._print_plan(sanity_error, steps)
        if self.args.execute:
            self._execute(steps)
        return 0

    def _latest_prior_version(self) -> StableVersion:
        if self.args.latest_version:
            return StableVersion.parse(self.args.latest_version)
        versions = []
        subprocess.run(
            ["git", "fetch", "--quiet", "--tags", self.args.remote],
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
        for line in result.stdout.splitlines():
            try:
                version = StableVersion.parse(line)
            except ValueError:
                continue
            if version < self.bad_version:
                versions.append(version)
        if not versions:
            raise SystemExit("no prior stable release tag found.")
        return max(versions)

    def _target_sanity_error(self, version: StableVersion) -> str:
        command = [
            sys.executable,
            "scripts/release/verify-release-target.py",
            "--target-version",
            version.tag(),
            "--latest-version",
            self.latest_version.tag(),
        ]
        result = subprocess.run(command, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        if result.returncode == 0:
            return ""
        return result.stderr.strip()

    def _steps(self) -> list[RecoveryStep]:
        package = self.args.package
        bad = self.bad_version
        replacement = self.replacement_version
        message = f"Accidental release; use {replacement.tag()} instead."
        return [
            RecoveryStep("Yank crates.io version", ["cargo", "yank", "--vers", bad.bare(), package]),
            RecoveryStep("Deprecate npm version", ["npm", "deprecate", f"{package}@{bad.bare()}", message]),
            RecoveryStep(
                "Move npm latest tag",
                ["npm", "dist-tag", "add", f"{package}@{replacement.bare()}", "latest"],
            ),
            RecoveryStep(
                "Mark GitHub Release as accidental prerelease",
                [
                    "gh",
                    "release",
                    "edit",
                    bad.tag(),
                    "--repo",
                    self.args.repo,
                    "--prerelease",
                    "--title",
                    f"{bad.tag()} (accidental release)",
                    "--notes",
                    message,
                ],
            ),
            RecoveryStep("Mark replacement GitHub Release as latest", ["gh", "release", "edit", replacement.tag(), "--repo", self.args.repo, "--latest"]),
            RecoveryStep(
                "Yank PyPI release from project management page",
                [f"https://pypi.org/manage/project/{package}/releases/"],
                manual=True,
            ),
            RecoveryStep(
                "Rollback Homebrew tap latest formula and remove accidental versioned formula",
                [
                    f"https://github.com/{self.args.homebrew_tap}",
                    f"restore Formula/kml.rb to {replacement.tag()}",
                    f"remove Formula/kml@{bad.bare()}.rb",
                ],
                manual=True,
            ),
        ]

    def _print_plan(self, sanity_error: str, steps: list[RecoveryStep]) -> None:
        if self.args.output == "json":
            payload = {
                "bad_version": self.bad_version.tag(),
                "replacement_version": self.replacement_version.tag(),
                "latest_prior_version": self.latest_version.tag(),
                "reason": sanity_error,
                "steps": [step.__dict__ for step in steps],
            }
            print(json.dumps(payload, indent=2))
            return
        print(f"Accidental release recovery plan: {self.bad_version.tag()}")
        print(f"Replacement version: {self.replacement_version.tag()}")
        print(sanity_error)
        for index, step in enumerate(steps, start=1):
            prefix = "manual" if step.manual else "run"
            print(f"{index}. {step.title} [{prefix}]")
            print("   " + shlex.join(step.command))

    def _execute(self, steps: list[RecoveryStep]) -> None:
        expected = self.bad_version.tag()
        if os.environ.get("KML_RELEASE_RECOVERY_CONFIRM") != expected:
            raise SystemExit(f"KML_RELEASE_RECOVERY_CONFIRM={expected} is required for --execute.")
        for step in steps:
            if step.manual:
                print(f"Manual step remains: {step.title}")
            else:
                subprocess.run(step.command, check=True)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    for option in ("bad-version", "replacement-version", "latest-version"):
        parser.add_argument(f"--{option}", required=option == "bad-version")
    parser.add_argument("--package", default="katana-markdown-linter")
    parser.add_argument("--repo", default="HiroyukiFuruno/katana-markdown-linter")
    parser.add_argument("--homebrew-tap", default="HiroyukiFuruno/homebrew-katana")
    parser.add_argument("--remote", default="origin")
    parser.add_argument("--output", choices=["text", "json"], default="text")
    parser.add_argument("--execute", action="store_true")
    return AccidentalReleaseRecovery(parser.parse_args()).run()


if __name__ == "__main__":
    raise SystemExit(main())
