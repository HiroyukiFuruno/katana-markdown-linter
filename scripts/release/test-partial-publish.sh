#!/usr/bin/env bash
set -euo pipefail

# This script simulates various partial publish states and verifies that verify-release-published.sh fails.
# Use the first argument as the version, or default to the version in Cargo.toml.
VERSION_BARE="${1:-$(awk -F '"' '/^version = / { print $2; exit }' Cargo.toml)}"
TAG="v${VERSION_BARE}"

# Mocking environment for verification test
export PATH="$PWD/mock-bin:$PATH"
mkdir -p mock-bin

cat << MOCK > mock-bin/gh
#!/usr/bin/env bash
if [[ "\$1" == "release" && "\$2" == "view" ]]; then
  if [[ "\${SIMULATE_MISSING_RELEASE:-false}" == "true" ]]; then
    echo "release not found" >&2
    exit 1
  fi
  is_prerelease="false"
  if [[ "\${SIMULATE_PRERELEASE:-false}" == "true" ]]; then
    is_prerelease="true"
  fi
  echo '{"tagName":"$TAG", "name":"$TAG", "targetCommitish":"'$(git rev-parse HEAD)'", "isDraft":false, "isPrerelease":'\$is_prerelease', "url":"https://github.com/mock", "assets":[]}'
fi
exit 0
MOCK
chmod +x mock-bin/gh

cat << MOCK > mock-bin/cargo
#!/usr/bin/env bash
if [[ "\$1" == "info" ]]; then
  if [[ "\${SIMULATE_MISSING_CRATE:-false}" == "true" ]]; then
    exit 1
  fi
  echo "version: $VERSION_BARE"
fi
exit 0
MOCK
chmod +x mock-bin/cargo

touch mock-bin/npm mock-bin/python3 mock-bin/uvx mock-bin/npx
chmod +x mock-bin/npm mock-bin/python3 mock-bin/uvx mock-bin/npx

echo "Testing missing GitHub Release for $TAG..."
SIMULATE_MISSING_RELEASE=true ./scripts/release/verify-release-published.sh "$VERSION_BARE" > /dev/null 2>&1 && { echo "FAILED: Expected failure for missing release"; exit 1; } || echo "PASSED"

echo "Testing missing crate for $VERSION_BARE..."
SIMULATE_MISSING_CRATE=true ./scripts/release/verify-release-published.sh "$VERSION_BARE" > /dev/null 2>&1 && { echo "FAILED: Expected failure for missing crate"; exit 1; } || echo "PASSED"

echo "Testing prerelease for $TAG..."
SIMULATE_PRERELEASE=true ./scripts/release/verify-release-published.sh "$VERSION_BARE" > /dev/null 2>&1 && { echo "FAILED: Expected failure for prerelease"; exit 1; } || echo "PASSED"

rm -rf mock-bin
