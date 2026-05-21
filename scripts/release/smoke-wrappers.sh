#!/usr/bin/env bash
set -euo pipefail

VERSION_INPUT="${1:?version is required}"
VERSION_BARE="${VERSION_INPUT#v}"
VERSION="v${VERSION_BARE}"
ARCHIVE_DIR="${2:-target/binary}"
SMOKE_DIR="target/wrapper-smoke"
PYTHON_PACKAGE="katana_markdown_linter"

scripts/release/verify-version.sh "$VERSION"
rm -rf "$SMOKE_DIR"
mkdir -p "$SMOKE_DIR"

export KML_WRAPPER_VERSION="$VERSION"
export KML_WRAPPER_ARCHIVE_DIR="$PWD/$ARCHIVE_DIR"

assert_version() {
  label="$1"
  output="$2"
  if [[ "$output" != "$VERSION_BARE" ]]; then
    echo "${label} version mismatch: expected ${VERSION_BARE}, got ${output}" >&2
    exit 1
  fi
}

write_fixture() {
  fixture="$SMOKE_DIR/README.md"
  printf '# Smoke\n\nText.\n' > "$fixture"
  printf '%s\n' "$fixture"
}

smoke_npm_cli() {
  export KML_WRAPPER_INSTALL_DIR="$PWD/$SMOKE_DIR/npm"
  npm_output="$(node wrappers/npm/bin/kml.js --version)"
  assert_version "npm wrapper" "$npm_output"
  node wrappers/npm/bin/kml.js check "$(write_fixture)" --locale en --output json >/dev/null
}

smoke_python_cli() {
  export KML_WRAPPER_INSTALL_DIR="$PWD/$SMOKE_DIR/python"
  python_output="$(PYTHONPATH="$PWD/wrappers/python/src" python3 -m "$PYTHON_PACKAGE" --version)"
  assert_version "Python wrapper" "$python_output"
  PYTHONPATH="$PWD/wrappers/python/src" python3 -m "$PYTHON_PACKAGE" check "$(write_fixture)" --locale en --output json >/dev/null
}

smoke_direct_mcp_wrappers() {
  export KML_WRAPPER_INSTALL_DIR="$PWD/$SMOKE_DIR/npm"
  python3 scripts/ci/mcp-stdio-smoke.py --command node wrappers/npm/bin/kml-mcp.js
  python3 scripts/ci/mcp-remote-smoke.py --command node wrappers/npm/bin/kml-mcp-remote.js

  export KML_WRAPPER_INSTALL_DIR="$PWD/$SMOKE_DIR/python"
  PYTHONPATH="$PWD/wrappers/python/src" python3 scripts/ci/mcp-stdio-smoke.py \
    --command python3 -m katana_markdown_linter.mcp_stdio
  PYTHONPATH="$PWD/wrappers/python/src" python3 scripts/ci/mcp-remote-smoke.py \
    --command python3 -m katana_markdown_linter.mcp_remote
}

smoke_exact_package_commands() {
  export KML_WRAPPER_INSTALL_DIR="$PWD/$SMOKE_DIR/exact-npx"
  python3 scripts/ci/mcp-stdio-smoke.py \
    --command npx --yes "$PWD/wrappers/npm" kml-mcp

  if command -v bunx >/dev/null 2>&1; then
    export KML_WRAPPER_INSTALL_DIR="$PWD/$SMOKE_DIR/exact-bunx"
    python3 scripts/ci/mcp-stdio-smoke.py \
      --command bunx --package "$PWD/wrappers/npm" kml-mcp
  else
    echo "bunx is not installed; skipping bunx wrapper smoke."
  fi

  export KML_WRAPPER_INSTALL_DIR="$PWD/$SMOKE_DIR/exact-uvx"
  python3 scripts/ci/mcp-stdio-smoke.py \
    --command uvx --from "$PWD/wrappers/python" kml-mcp
}

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

smoke_legacy_stale_cache_is_ignored() {
  npm_stale_dir="$PWD/$SMOKE_DIR/npm-stale"
  write_stale_binary "$npm_stale_dir/bin/kml"
  export KML_WRAPPER_INSTALL_DIR="$npm_stale_dir"
  npm_stale_output="$(node wrappers/npm/bin/kml.js --version)"
  assert_stale_cache_ignored "npm wrapper" "$npm_stale_output"

  python_stale_dir="$PWD/$SMOKE_DIR/python-stale"
  write_stale_binary "$python_stale_dir/bin/kml"
  export KML_WRAPPER_INSTALL_DIR="$python_stale_dir"
  python_stale_output="$(PYTHONPATH="$PWD/wrappers/python/src" python3 -m "$PYTHON_PACKAGE" --version)"
  assert_stale_cache_ignored "Python wrapper" "$python_stale_output"
}

smoke_npm_cli
smoke_python_cli
smoke_direct_mcp_wrappers
smoke_exact_package_commands
smoke_legacy_stale_cache_is_ignored

echo "Wrapper smoke passed for ${VERSION}"
