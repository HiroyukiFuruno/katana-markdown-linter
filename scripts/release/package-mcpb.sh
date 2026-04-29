#!/usr/bin/env bash
set -euo pipefail

VERSION_INPUT="${1:-}"

if [[ -z "$VERSION_INPUT" ]]; then
  echo "Usage: $0 <version>" >&2
  exit 2
fi

VERSION_BARE="${VERSION_INPUT#v}"
BUNDLE_NAME="katana-markdown-linter-${VERSION_BARE}.mcpb"
DIST_DIR="target/mcpb"
STAGING_DIR="${DIST_DIR}/staging/${VERSION_BARE}"
BINARY_SRC="target/release/kml-mcp"

scripts/release/verify-version.sh "$VERSION_INPUT"

if [[ ! -x "$BINARY_SRC" ]]; then
  echo "Expected MCP binary was not built: ${BINARY_SRC}" >&2
  exit 1
fi

rm -rf "$STAGING_DIR"
mkdir -p "$STAGING_DIR/server"
cp mcpb/manifest.json "$STAGING_DIR/manifest.json"
cp "$BINARY_SRC" "$STAGING_DIR/server/kml-mcp"
chmod 0755 "$STAGING_DIR/server/kml-mcp"

PACKAGE_PATH="${DIST_DIR}/${BUNDLE_NAME}"
CHECKSUM_PATH="${PACKAGE_PATH}.sha256"
rm -f "$PACKAGE_PATH" "$CHECKSUM_PATH"

(
  cd "$STAGING_DIR"
  zip -X -q -r "../../${BUNDLE_NAME}" .
)

if command -v sha256sum >/dev/null 2>&1; then
  sha256sum "$PACKAGE_PATH" > "$CHECKSUM_PATH"
else
  shasum -a 256 "$PACKAGE_PATH" > "$CHECKSUM_PATH"
fi

if [[ -n "${GITHUB_OUTPUT:-}" ]]; then
  {
    echo "mcpb_path=${PACKAGE_PATH}"
    echo "mcpb_checksum_path=${CHECKSUM_PATH}"
  } >> "$GITHUB_OUTPUT"
fi

echo "Packaged ${PACKAGE_PATH}"
echo "Checksum ${CHECKSUM_PATH}"
