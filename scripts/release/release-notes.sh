#!/usr/bin/env bash
set -euo pipefail

VERSION_BARE="${1:?version is required}"
VERSION="v${VERSION_BARE}"

if [[ ! -f CHANGELOG.md ]]; then
  echo "Release ${VERSION}"
  echo
  echo "No CHANGELOG.md found."
  exit 0
fi

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
