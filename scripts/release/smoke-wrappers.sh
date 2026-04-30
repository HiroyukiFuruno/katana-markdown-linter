#!/usr/bin/env bash
set -euo pipefail

VERSION_INPUT="${1:?version is required}"
VERSION_BARE="${VERSION_INPUT#v}"
VERSION="v${VERSION_BARE}"
ARCHIVE_DIR="${2:-target/binary}"
SMOKE_DIR="target/wrapper-smoke"

scripts/release/verify-version.sh "$VERSION"
rm -rf "$SMOKE_DIR"
mkdir -p "$SMOKE_DIR"

npm_output="$(
  KML_WRAPPER_VERSION="$VERSION" \
  KML_WRAPPER_ARCHIVE_DIR="$PWD/$ARCHIVE_DIR" \
  KML_WRAPPER_INSTALL_DIR="$PWD/$SMOKE_DIR/npm" \
  node wrappers/npm/bin/kml.js --version
)"
if [[ "$npm_output" != "$VERSION_BARE" ]]; then
  echo "npm wrapper version mismatch: expected ${VERSION_BARE}, got ${npm_output}" >&2
  exit 1
fi

python_output="$(
  PYTHONPATH="$PWD/wrappers/python/src" \
  KML_WRAPPER_VERSION="$VERSION" \
  KML_WRAPPER_ARCHIVE_DIR="$PWD/$ARCHIVE_DIR" \
  KML_WRAPPER_INSTALL_DIR="$PWD/$SMOKE_DIR/python" \
  python3 -m katana_markdown_linter --version
)"
if [[ "$python_output" != "$VERSION_BARE" ]]; then
  echo "Python wrapper version mismatch: expected ${VERSION_BARE}, got ${python_output}" >&2
  exit 1
fi

echo "Wrapper smoke passed for ${VERSION}"
