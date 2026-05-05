#!/usr/bin/env bash
set -euo pipefail

# This script simulates various partial publish states and verifies that verify-release-published.sh fails.
VERSION_BARE="${1:-$(awk -F '"' '/^version = / { print $2; exit }' Cargo.toml)}"
TAG="v${VERSION_BARE}"

TMP_MOCK_ROOT="$(mktemp -d)"
export PATH="${TMP_MOCK_ROOT}/bin:$PATH"
mkdir -p "${TMP_MOCK_ROOT}/bin"

cleanup() {
  rm -rf "${TMP_MOCK_ROOT}"
}
trap cleanup EXIT

cat << MOCK > "${TMP_MOCK_ROOT}/bin/gh"
#!/usr/bin/env bash
# Simple gh mock that handles --json and --jq for release view
if [[ "\$1" == "release" && "\$2" == "view" ]]; then
  if [[ "\${SIMULATE_MISSING_RELEASE:-false}" == "true" ]]; then
    echo "release not found" >&2
    exit 1
  fi
  is_prerelease="false"
  if [[ "\${SIMULATE_PRERELEASE:-false}" == "true" ]]; then
    is_prerelease="true"
  fi

  # Respond based on requested fields
  json_args=""
  for arg in "\$@"; do
    if [[ "\$arg" == "--json" ]]; then json_args="true"; fi
  done

  if [[ -n "\$json_args" ]]; then
    # Very simple field extractor for mock
    field=""
    for i in "\${!@}"; do
      if [[ "\${!i}" == "--jq" ]]; then
        next_idx=\$((i+1))
        field="\${!next_idx}"
      fi
    done

    case "\$field" in
      ".tagName") echo "$TAG" ;;
      ".name") echo "$TAG" ;;
      ".targetCommitish") echo "$(git rev-parse HEAD)" ;;
      ".isDraft") echo "false" ;;
      ".isPrerelease") echo "\$is_prerelease" ;;
      ".url") echo "https://github.com/mock" ;;
      ".assets[].name") echo "kml-$TAG-x86_64-unknown-linux-gnu.tar.gz" ;;
      *) echo '{"tagName":"$TAG", "name":"$TAG", "isDraft":false, "isPrerelease":'\$is_prerelease'}' ;;
    esac
    exit 0
  fi
fi
exit 0
MOCK
chmod +x "${TMP_MOCK_ROOT}/bin/gh"

cat << MOCK > "${TMP_MOCK_ROOT}/bin/cargo"
#!/usr/bin/env bash
if [[ "\$1" == "info" ]]; then
  if [[ "\${SIMULATE_MISSING_CRATE:-false}" == "true" ]]; then
    exit 1
  fi
  echo "version: $VERSION_BARE"
fi
exit 0
MOCK
chmod +x "${TMP_MOCK_ROOT}/bin/cargo"

touch "${TMP_MOCK_ROOT}/bin/npm" "${TMP_MOCK_ROOT}/bin/python3" "${TMP_MOCK_ROOT}/bin/uvx" "${TMP_MOCK_ROOT}/bin/npx"
chmod +x "${TMP_MOCK_ROOT}/bin/npm" "${TMP_MOCK_ROOT}/bin/python3" "${TMP_MOCK_ROOT}/bin/uvx" "${TMP_MOCK_ROOT}/bin/npx"

# Ensure we have a valid state file for consistency check or skip it
mkdir -p target
cat << EOF > target/release-verify-state.json
{
  "version": "$TAG",
  "editor_artifacts": {
    "vscode": {"state": "deferred"},
    "zed": {"state": "deferred"}
  },
  "publish_blockers": [],
  "release_decision": "allow_release"
}
EOF

echo "Testing missing GitHub Release for $TAG..."
SIMULATE_MISSING_RELEASE=true ./scripts/release/verify-release-published.sh "$VERSION_BARE" > /dev/null 2>&1 && { echo "FAILED: Expected failure for missing release"; exit 1; } || echo "PASSED"

echo "Testing missing crate for $VERSION_BARE..."
SIMULATE_MISSING_CRATE=true ./scripts/release/verify-release-published.sh "$VERSION_BARE" > /dev/null 2>&1 && { echo "FAILED: Expected failure for missing crate"; exit 1; } || echo "PASSED"

echo "Testing prerelease for $TAG..."
SIMULATE_PRERELEASE=true ./scripts/release/verify-release-published.sh "$VERSION_BARE" > /dev/null 2>&1 && { echo "FAILED: Expected failure for prerelease"; exit 1; } || echo "PASSED"
