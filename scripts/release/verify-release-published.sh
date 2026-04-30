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
release_title="$(gh release view "${TAG}" --repo "${REPO}" --json name --jq '.name')"
release_target="$(gh release view "${TAG}" --repo "${REPO}" --json targetCommitish --jq '.targetCommitish')"
release_draft="$(gh release view "${TAG}" --repo "${REPO}" --json isDraft --jq '.isDraft')"
release_url="$(gh release view "${TAG}" --repo "${REPO}" --json url --jq '.url')"

if [[ "${release_tag}" != "${TAG}" ]]; then
  echo "GitHub Release tag mismatch: expected ${TAG}, got ${release_tag}" >&2
  exit 1
fi

if [[ "${release_title}" != "${TAG}" ]]; then
  echo "GitHub Release title mismatch: expected ${TAG}, got ${release_title}" >&2
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

asset_names="$(gh release view "${TAG}" --repo "${REPO}" --json assets --jq '.assets[].name')"
for target in \
  x86_64-unknown-linux-gnu \
  x86_64-apple-darwin \
  aarch64-apple-darwin \
  x86_64-pc-windows-msvc; do
  extension="tar.gz"
  if [[ "${target}" == x86_64-pc-windows-msvc ]]; then
    extension="zip"
  fi
  archive="kml-${TAG}-${target}.${extension}"
  if ! grep -Fxq "${archive}" <<< "${asset_names}"; then
    echo "GitHub Release is missing binary archive: ${archive}" >&2
    exit 1
  fi
  if ! grep -Fxq "${archive}.sha256" <<< "${asset_names}"; then
    echo "GitHub Release is missing binary checksum: ${archive}.sha256" >&2
    exit 1
  fi
done

case "$(uname -s)/$(uname -m)" in
  Linux/x86_64) smoke_target="x86_64-unknown-linux-gnu" ;;
  Darwin/x86_64) smoke_target="x86_64-apple-darwin" ;;
  Darwin/arm64) smoke_target="aarch64-apple-darwin" ;;
  *) smoke_target="" ;;
esac

if [[ -n "${smoke_target}" ]]; then
  smoke_extension="tar.gz"
  smoke_archive="kml-${TAG}-${smoke_target}.${smoke_extension}"
  smoke_dir="$(mktemp -d)"
  trap 'rm -rf "${smoke_dir}"' EXIT
  gh release download "${TAG}" --repo "${REPO}" --dir "${smoke_dir}" --pattern "${smoke_archive}" --pattern "${smoke_archive}.sha256"
  python3 scripts/release/binary_artifacts.py smoke \
    --version "${TAG}" \
    --target "${smoke_target}" \
    --archive "${smoke_dir}/${smoke_archive}" \
    --checksum "${smoke_dir}/${smoke_archive}.sha256"
fi

cargo_info="$(cargo info "${PACKAGE}@${VERSION_BARE}" --registry crates-io)"
crate_version="$(printf '%s\n' "${cargo_info}" | sed -n 's/^version: //p')"
if [[ "${crate_version}" != "${VERSION_BARE}" ]]; then
  echo "crates.io version mismatch: expected ${VERSION_BARE}, got ${crate_version:-missing}" >&2
  exit 1
fi

echo "tag_target=${local_target}"
echo "github_release_title=${release_title}"
echo "github_release_target=${release_target}"
echo "github_release_url=${release_url}"
echo "crates_io_version=${crate_version}"
