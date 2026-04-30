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

write_stale_binary() {
  stale_binary="$1"
  mkdir -p "$(dirname "$stale_binary")"
  printf '#!/usr/bin/env sh\nprintf "0.0.0\\n"\n' > "$stale_binary"
  chmod +x "$stale_binary"
}

assert_stale_cache_ignored() {
  launcher="$1"
  output="$2"
  if [[ "$output" != "$VERSION_BARE" ]]; then
    echo "${launcher} reused an unversioned stale cache: expected ${VERSION_BARE}, got ${output}" >&2
    exit 1
  fi
}

npm_stale_dir="$PWD/$SMOKE_DIR/npm-stale"
write_stale_binary "$npm_stale_dir/bin/kml"
npm_stale_output="$(
  KML_WRAPPER_VERSION="$VERSION" \
  KML_WRAPPER_ARCHIVE_DIR="$PWD/$ARCHIVE_DIR" \
  KML_WRAPPER_INSTALL_DIR="$npm_stale_dir" \
  node wrappers/npm/bin/kml.js --version
)"
assert_stale_cache_ignored "npm wrapper" "$npm_stale_output"

python_stale_dir="$PWD/$SMOKE_DIR/python-stale"
write_stale_binary "$python_stale_dir/bin/kml"
python_stale_output="$(
  PYTHONPATH="$PWD/wrappers/python/src" \
  KML_WRAPPER_VERSION="$VERSION" \
  KML_WRAPPER_ARCHIVE_DIR="$PWD/$ARCHIVE_DIR" \
  KML_WRAPPER_INSTALL_DIR="$python_stale_dir" \
  python3 -m katana_markdown_linter --version
)"
assert_stale_cache_ignored "Python wrapper" "$python_stale_output"

echo "Wrapper smoke passed for ${VERSION}"
