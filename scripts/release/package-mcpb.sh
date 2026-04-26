#!/bin/bash
set -e

VERSION="${1}"
if [ -z "${VERSION}" ]; then
  echo "Usage: $0 <version>"
  exit 1
fi

VERSION_BARE="${VERSION#v}"
DIST_DIR="target/mcpb"
BUNDLE_NAME="katana-markdown-linter-${VERSION_BARE}.mcpb"
STAGING_DIR="${DIST_DIR}/staging"

echo "Packaging MCPB bundle for v${VERSION_BARE}..."

mkdir -p "${STAGING_DIR}"
cp mcpb/manifest.json "${STAGING_DIR}/"

# Determine binary source (assume it was built by Makefile)
# We support multiple platforms if they exist in target/
# For now, we package the current host binary
BINARY_SRC="target/debug/kml-mcp"
if [ -f "target/release/kml-mcp" ]; then
  BINARY_SRC="target/release/kml-mcp"
fi

if [ ! -f "${BINARY_SRC}" ]; then
  echo "Error: ${BINARY_SRC} not found. Build it first."
  exit 1
fi

cp "${BINARY_SRC}" "${STAGING_DIR}/"

# Save absolute path to DIST_DIR
ABS_DIST_DIR="$(pwd)/${DIST_DIR}"

cd "${STAGING_DIR}"
zip -r "${ABS_DIST_DIR}/${BUNDLE_NAME}" .
cd -

echo "Created ${DIST_DIR}/${BUNDLE_NAME}"
sha256sum "${DIST_DIR}/${BUNDLE_NAME}" > "${DIST_DIR}/${BUNDLE_NAME}.sha256"
