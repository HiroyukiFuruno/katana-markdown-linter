#!/usr/bin/env bash
set -euo pipefail

INPUT_VERSION="${1:-}"
# Strip potential trailing carriage return (common in Windows CI environments)
INPUT_VERSION="${INPUT_VERSION%$'\r'}"

CARGO_VERSION="$(awk -F '"' '/^version = / { print $2 }' Cargo.toml | head -n 1)"
# Strip potential trailing carriage return (common in Windows CI environments)
CARGO_VERSION="${CARGO_VERSION%$'\r'}"

if [[ -z "$INPUT_VERSION" ]]; then
  INPUT_VERSION="$CARGO_VERSION"
fi

VERSION_BARE="${INPUT_VERSION#v}"
VERSION="v${VERSION_BARE}"

if [[ ! "$VERSION_BARE" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "Invalid release version: ${INPUT_VERSION}" >&2
  exit 1
fi

if [[ "$VERSION_BARE" != "$CARGO_VERSION" ]]; then
  echo "Cargo.toml version (${CARGO_VERSION}) does not match release version (${VERSION_BARE})" >&2
  exit 1
fi

python3 scripts/release/verify-version-sync.py --version "$CARGO_VERSION"

echo "Release version: ${VERSION}"

if [[ -n "${GITHUB_OUTPUT:-}" ]]; then
  {
    echo "version=${VERSION}"
    echo "version_bare=${VERSION_BARE}"
    echo "cargo_version=${CARGO_VERSION}"
  } >> "$GITHUB_OUTPUT"
fi
