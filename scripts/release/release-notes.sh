#!/usr/bin/env bash
set -euo pipefail

VERSION_BARE="${1:?version is required}"
VERSION="v${VERSION_BARE}"

if [[ ! -f CHANGELOG.md ]]; then
  echo "CHANGELOG.md is required to build release notes for ${VERSION}." >&2
  exit 1
fi

notes="$(
  awk -v version="$VERSION" -v bare="$VERSION_BARE" '
  /^## / {
    if (capture) {
      exit
    }
    heading = $0
    if (heading ~ "## " version "$" || heading ~ "## " bare "$") {
      capture = 1
      next
    }
  }
  capture {
    print
  }
' CHANGELOG.md
)"

if [[ -z "$(tr -d '[:space:]' <<< "${notes}")" ]]; then
  echo "CHANGELOG.md is missing a non-empty section for ${VERSION}." >&2
  exit 1
fi

printf '%s\n' "${notes}"
