#!/usr/bin/env bash
set -euo pipefail

VERSION_BARE="${1:?version is required}"
REPO="${2:-${GH_REPO:-HiroyukiFuruno/katana-markdown-linter}}"
PACKAGE="${3:-katana-markdown-linter}"
TAG="v${VERSION_BARE}"
VERIFY_OUTPUT_DIR="${VERIFY_OUTPUT_DIR:-target/release-verify/${TAG}}"
HOMEBREW_TAP_REPO="${HOMEBREW_TAP_REPO:-HiroyukiFuruno/homebrew-katana}"
HOMEBREW_TAP_BRANCH="${HOMEBREW_TAP_BRANCH:-master}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TMP_ROOT=""

source "${SCRIPT_DIR}/verify-release-common.sh"
source "${SCRIPT_DIR}/verify-release-assets.sh"
source "${SCRIPT_DIR}/verify-release-registries.sh"
source "${SCRIPT_DIR}/verify-release-homebrew.sh"

require_command gh
require_command cargo
require_command python3
trap cleanup EXIT
TMP_ROOT="$(mktemp -d)"

verify_consistency_with_state
verify_github_release
verify_binary_assets
verify_mcp_binary_assets
smoke_current_platform_binary
smoke_current_platform_mcp_binaries
verify_crates_io
verify_npm_registry
verify_pypi_registry
smoke_npm_wrapper
smoke_npm_mcp_wrapper
smoke_pypi_wrapper
smoke_pypi_mcp_wrapper
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
