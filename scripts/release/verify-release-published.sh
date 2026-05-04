#!/usr/bin/env bash
set -euo pipefail

VERSION_BARE="${1:?version is required}"
REPO="${2:-${GH_REPO:-HiroyukiFuruno/katana-markdown-linter}}"
PACKAGE="${3:-katana-markdown-linter}"
TAG="v${VERSION_BARE}"
VERIFY_OUTPUT_DIR="${VERIFY_OUTPUT_DIR:-target/release-verify/${TAG}}"
HOMEBREW_TAP_REPO="${HOMEBREW_TAP_REPO:-HiroyukiFuruno/homebrew-katana}"
HOMEBREW_TAP_BRANCH="${HOMEBREW_TAP_BRANCH:-master}"
TMP_ROOT=""

cleanup() {
  [[ -z "${TMP_ROOT}" ]] || rm -rf "${TMP_ROOT}"
}

require_command() {
  command_name="$1"
  command -v "${command_name}" >/dev/null 2>&1 && return
  echo "${command_name} is required to verify ${TAG} publication." >&2
  exit 1
}

assert_equal() {
  label="$1"
  expected="$2"
  actual="$3"
  [[ "${actual}" == "${expected}" ]] && return
  echo "${label} mismatch: expected ${expected}, got ${actual:-missing}" >&2
  exit 1
}

release_asset_exists() {
  asset_name="$1"
  [[ $'\n'"${asset_names}"$'\n' == *$'\n'"${asset_name}"$'\n'* ]]
}

verify_github_release() {
  local_target="$(git rev-parse "${TAG}^{}")"
  release_tag="$(gh release view "${TAG}" --repo "${REPO}" --json tagName --jq '.tagName')"
  release_title="$(gh release view "${TAG}" --repo "${REPO}" --json name --jq '.name')"
  release_target="$(gh release view "${TAG}" --repo "${REPO}" --json targetCommitish --jq '.targetCommitish')"
  release_draft="$(gh release view "${TAG}" --repo "${REPO}" --json isDraft --jq '.isDraft')"
  release_url="$(gh release view "${TAG}" --repo "${REPO}" --json url --jq '.url')"

  assert_equal "GitHub Release tag" "${TAG}" "${release_tag}"
  assert_equal "GitHub Release title" "${TAG}" "${release_title}"
  assert_equal "GitHub Release draft state" "false" "${release_draft}"

  if [[ "${release_target}" != "${local_target}" ]]; then
    echo "${TAG} GitHub Release target differs from the local tag target." >&2
    echo "release: ${release_target}" >&2
    echo "local:   ${local_target}" >&2
    exit 1
  fi

  asset_names="$(gh release view "${TAG}" --repo "${REPO}" --json assets --jq '.assets[].name')"
}

verify_binary_assets() {
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
    if ! release_asset_exists "${archive}"; then
      echo "GitHub Release is missing binary archive: ${archive}" >&2
      exit 1
    fi
    if ! release_asset_exists "${archive}.sha256"; then
      echo "GitHub Release is missing binary checksum: ${archive}.sha256" >&2
      exit 1
    fi
  done
}

smoke_current_platform_binary() {
  case "$(uname -s)/$(uname -m)" in
    Linux/x86_64) smoke_target="x86_64-unknown-linux-gnu" ;;
    Darwin/x86_64) smoke_target="x86_64-apple-darwin" ;;
    Darwin/arm64) smoke_target="aarch64-apple-darwin" ;;
    *) smoke_target="" ;;
  esac

  [[ -n "${smoke_target}" ]] || return

  smoke_dir="${TMP_ROOT}/binary-smoke"
  mkdir -p "${smoke_dir}"
  smoke_archive="kml-${TAG}-${smoke_target}.tar.gz"
  gh release download "${TAG}" \
    --repo "${REPO}" \
    --dir "${smoke_dir}" \
    --pattern "${smoke_archive}" \
    --pattern "${smoke_archive}.sha256"
  python3 scripts/release/binary_artifacts.py smoke \
    --version "${TAG}" \
    --target "${smoke_target}" \
    --archive "${smoke_dir}/${smoke_archive}" \
    --checksum "${smoke_dir}/${smoke_archive}.sha256"
}

verify_crates_io() {
  cargo_info="$(cargo info "${PACKAGE}@${VERSION_BARE}" --registry crates-io)"
  crate_version="$(printf '%s\n' "${cargo_info}" | sed -n 's/^version: //p')"
  if [[ "${crate_version}" != "${VERSION_BARE}" ]]; then
    echo "crates.io version mismatch: expected ${VERSION_BARE}, got ${crate_version:-missing}" >&2
    exit 1
  fi
}

verify_npm_registry() {
  require_command npm
  npm_registry_version="$(npm view "${PACKAGE}@${VERSION_BARE}" version 2>/dev/null || true)"
  if [[ "${npm_registry_version}" != "${VERSION_BARE}" ]]; then
    echo "npm registry version mismatch: expected ${VERSION_BARE}, got ${npm_registry_version:-missing}" >&2
    exit 1
  fi
}

verify_pypi_registry() {
  pypi_registry_version="$(
    python3 -c 'import json, sys, urllib.request; package, version = sys.argv[1:3]; url = f"https://pypi.org/pypi/{package}/{version}/json"; print(json.load(urllib.request.urlopen(url, timeout=30))["info"]["version"])' "${PACKAGE}" "${VERSION_BARE}" 2>/dev/null || true
  )"
  if [[ "${pypi_registry_version}" != "${VERSION_BARE}" ]]; then
    echo "PyPI version mismatch: expected ${VERSION_BARE}, got ${pypi_registry_version:-missing}" >&2
    exit 1
  fi
}

smoke_npm_wrapper() {
  require_command npx
  npm_wrapper_version="$(npx --yes "${PACKAGE}@${VERSION_BARE}" --version | tail -n 1)"
  if [[ "${npm_wrapper_version}" != "${VERSION_BARE}" ]]; then
    echo "npm wrapper version mismatch: expected ${VERSION_BARE}, got ${npm_wrapper_version:-missing}" >&2
    exit 1
  fi
}

smoke_pypi_wrapper() {
  require_command uvx
  pypi_wrapper_version="$(uvx --from "${PACKAGE}==${VERSION_BARE}" kml --version | tail -n 1)"
  if [[ "${pypi_wrapper_version}" != "${VERSION_BARE}" ]]; then
    echo "PyPI wrapper version mismatch: expected ${VERSION_BARE}, got ${pypi_wrapper_version:-missing}" >&2
    exit 1
  fi
}

verify_vscode_extension() {
  if [[ "${PUBLISH_VSCODE_EXTENSION:-false}" == "true" ]]; then
    # Check VS Code Marketplace availability
    if curl -sSfL "https://marketplace.visualstudio.com/items?itemName=HiroyukiFuruno.vscode-katana-markdown-linter" > /dev/null 2>&1; then
      echo "vscode_extension_status=published"
    else
      echo "VS Code extension publication check failed: extension not found in marketplace." >&2
      exit 1
    fi
  else
    echo "vscode_extension_status=deferred"
  fi
}

verify_zed_extension() {
  if [[ "${PUBLISH_ZED_EXTENSION:-false}" == "true" ]]; then
    # Check Zed Extension Registry availability
    # Note: Zed extension registry URL pattern is based on current zed ecosystem
    if curl -sSfL "https://extensions.zed.dev/extensions/katana-markdown-linter" > /dev/null 2>&1; then
       echo "zed_extension_status=published"
    else
      echo "Zed extension publication check failed: extension not found in registry." >&2
      exit 1
    fi
  else
    echo "zed_extension_status=deferred"
  fi
}

verify_homebrew_formula() {
  formula_dist_dir="${TMP_ROOT}/homebrew-assets"
  formula_path="${VERIFY_OUTPUT_DIR}/homebrew/kml.rb"
  versioned_formula_path="${VERIFY_OUTPUT_DIR}/homebrew/kml@${VERSION_BARE}.rb"
  mkdir -p "${formula_dist_dir}" "$(dirname "${formula_path}")"
  for target in \
    x86_64-unknown-linux-gnu \
    x86_64-apple-darwin \
    aarch64-apple-darwin; do
    archive="kml-${TAG}-${target}.tar.gz"
    gh release download "${TAG}" \
      --repo "${REPO}" \
      --dir "${formula_dist_dir}" \
      --pattern "${archive}" \
      --pattern "${archive}.sha256"
  done
  python3 scripts/release/homebrew_formula.py generate \
    --version "${TAG}" \
    --formula-name kml \
    --repo "${REPO}" \
    --dist-dir "${formula_dist_dir}" \
    --output "${formula_path}" >/dev/null
  python3 scripts/release/homebrew_formula.py check \
    --version "${TAG}" \
    --formula-name kml \
    --repo "${REPO}" \
    --dist-dir "${formula_dist_dir}" \
    --output "${formula_path}" >/dev/null
  python3 scripts/release/homebrew_formula.py generate \
    --version "${TAG}" \
    --formula-name "kml@${VERSION_BARE}" \
    --repo "${REPO}" \
    --dist-dir "${formula_dist_dir}" \
    --output "${versioned_formula_path}" >/dev/null
  python3 scripts/release/homebrew_formula.py check \
    --version "${TAG}" \
    --formula-name "kml@${VERSION_BARE}" \
    --repo "${REPO}" \
    --dist-dir "${formula_dist_dir}" \
    --output "${versioned_formula_path}" >/dev/null
  verify_homebrew_tap_formula "Formula/kml.rb" "${formula_path}"
  verify_homebrew_tap_formula "Formula/kml@${VERSION_BARE}.rb" "${versioned_formula_path}"
}

verify_homebrew_tap_formula() {
  tap_path="$1"
  expected_path="$2"
  actual_path="${TMP_ROOT}/$(basename "${tap_path}")"
  raw_url="https://raw.githubusercontent.com/${HOMEBREW_TAP_REPO}/${HOMEBREW_TAP_BRANCH}/${tap_path}"
  python3 - "${raw_url}" "${actual_path}" <<'PY'
import sys
import urllib.request

url, output = sys.argv[1:3]
with urllib.request.urlopen(url, timeout=30) as response:
    content = response.read()
with open(output, "wb") as file:
    file.write(content)
PY
  if ! cmp -s "${expected_path}" "${actual_path}"; then
    echo "Homebrew tap formula mismatch: ${tap_path} does not match ${TAG}." >&2
    echo "tap:      ${raw_url}" >&2
    echo "expected: ${expected_path}" >&2
    exit 1
  fi
}

require_command gh
require_command cargo
require_command python3
trap cleanup EXIT
TMP_ROOT="$(mktemp -d)"

verify_github_release
verify_binary_assets
smoke_current_platform_binary
verify_crates_io
verify_npm_registry
verify_pypi_registry
smoke_npm_wrapper
smoke_pypi_wrapper
verify_vscode_extension
verify_zed_extension
verify_homebrew_formula

echo "tag_target=${local_target}"
echo "github_release_title=${release_title}"
echo "github_release_target=${release_target}"
echo "github_release_url=${release_url}"
echo "crates_io_version=${crate_version}"
echo "npm_registry_version=${npm_registry_version}"
echo "pypi_registry_version=${pypi_registry_version}"
echo "npm_wrapper_version=${npm_wrapper_version}"
echo "pypi_wrapper_version=${pypi_wrapper_version}"
echo "homebrew_formula_path=${formula_path}"
echo "homebrew_versioned_formula_path=${versioned_formula_path}"
