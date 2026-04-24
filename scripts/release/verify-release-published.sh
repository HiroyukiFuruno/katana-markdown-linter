#!/usr/bin/env bash
set -euo pipefail

VERSION_BARE="${1:?version is required}"
REPO="${2:-${GH_REPO:-HiroyukiFuruno/katana-markdown-linter}}"
PACKAGE="${3:-katana-markdown-linter}"
TAG="v${VERSION_BARE}"

if ! command -v gh >/dev/null 2>&1; then
  echo "gh CLI is required to verify GitHub Release state." >&2
  exit 1
fi

local_target="$(git rev-parse "${TAG}^{}")"
release_tag="$(gh release view "${TAG}" --repo "${REPO}" --json tagName --jq '.tagName')"
release_target="$(gh release view "${TAG}" --repo "${REPO}" --json targetCommitish --jq '.targetCommitish')"
release_draft="$(gh release view "${TAG}" --repo "${REPO}" --json isDraft --jq '.isDraft')"
release_url="$(gh release view "${TAG}" --repo "${REPO}" --json url --jq '.url')"

if [[ "${release_tag}" != "${TAG}" ]]; then
  echo "GitHub Release tag mismatch: expected ${TAG}, got ${release_tag}" >&2
  exit 1
fi

if [[ "${release_draft}" != "false" ]]; then
  echo "${TAG} GitHub Release is still a draft." >&2
  exit 1
fi

if [[ "${release_target}" != "${local_target}" ]]; then
  echo "${TAG} GitHub Release target differs from the local tag target." >&2
  echo "release: ${release_target}" >&2
  echo "local:   ${local_target}" >&2
  exit 1
fi

cargo_info="$(cargo info "${PACKAGE}@${VERSION_BARE}" --registry crates-io)"
crate_version="$(awk -F ': ' '/^version:/ { print $2; exit }' <<< "${cargo_info}")"
if [[ "${crate_version}" != "${VERSION_BARE}" ]]; then
  echo "crates.io version mismatch: expected ${VERSION_BARE}, got ${crate_version:-missing}" >&2
  exit 1
fi

echo "tag_target=${local_target}"
echo "github_release_target=${release_target}"
echo "github_release_url=${release_url}"
echo "crates_io_version=${crate_version}"
