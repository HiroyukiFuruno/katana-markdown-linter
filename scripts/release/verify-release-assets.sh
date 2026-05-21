#!/usr/bin/env bash

verify_github_release() {
  local_target="$(git rev-parse "${TAG}^{}")"
  release_tag="$(gh release view "${TAG}" --repo "${REPO}" --json tagName --jq '.tagName')"
  release_title="$(gh release view "${TAG}" --repo "${REPO}" --json name --jq '.name')"
  release_target="$(gh release view "${TAG}" --repo "${REPO}" --json targetCommitish --jq '.targetCommitish')"
  release_draft="$(gh release view "${TAG}" --repo "${REPO}" --json isDraft --jq '.isDraft')"
  release_prerelease="$(gh release view "${TAG}" --repo "${REPO}" --json isPrerelease --jq '.isPrerelease')"
  release_url="$(gh release view "${TAG}" --repo "${REPO}" --json url --jq '.url')"

  assert_equal "GitHub Release tag" "${TAG}" "${release_tag}"
  assert_equal "GitHub Release title" "${TAG}" "${release_title}"
  assert_equal "GitHub Release draft state" "false" "${release_draft}"
  assert_equal "GitHub Release prerelease state" "false" "${release_prerelease}"

  if [[ "${release_target}" != "${local_target}" ]]; then
    echo "${TAG} GitHub Release target differs from the local tag target." >&2
    echo "release: ${release_target}" >&2
    echo "local:   ${local_target}" >&2
    exit 1
  fi

  asset_names="$(gh release view "${TAG}" --repo "${REPO}" --json assets --jq '.assets[].name')"
}

verify_binary_assets() {
  verify_executable_assets "kml" "binary"
}

verify_mcp_binary_assets() {
  verify_executable_assets "kml-mcp" "MCP binary"
  verify_executable_assets "kml-mcp-remote" "MCP binary"
}

verify_executable_assets() {
  local executable="$1"
  local label="$2"
  local target archive
  for target in \
    x86_64-unknown-linux-gnu \
    x86_64-apple-darwin \
    aarch64-apple-darwin \
    x86_64-pc-windows-msvc; do
    archive="$(release_asset_archive_name "${executable}" "${target}")"
    if ! release_asset_exists "${archive}"; then
      echo "GitHub Release is missing ${label} archive: ${archive}" >&2
      exit 1
    fi
    if ! release_asset_exists "${archive}.sha256"; then
      echo "GitHub Release is missing ${label} checksum: ${archive}.sha256" >&2
      exit 1
    fi
  done
}

smoke_current_platform_binary() {
  local smoke_target
  smoke_target="$(current_smoke_target)"
  [[ -n "${smoke_target}" ]] || return
  smoke_release_binary "kml" "${smoke_target}" "${TMP_ROOT}/binary-smoke"
}

smoke_current_platform_mcp_binaries() {
  local smoke_target executable
  smoke_target="$(current_smoke_target)"
  [[ -n "${smoke_target}" ]] || return
  for executable in kml-mcp kml-mcp-remote; do
    smoke_release_binary "${executable}" "${smoke_target}" "${TMP_ROOT}/mcp-binary-smoke"
  done
}

smoke_release_binary() {
  local executable="$1"
  local smoke_target="$2"
  local smoke_dir="$3"
  local smoke_archive
  mkdir -p "${smoke_dir}"
  smoke_archive="$(release_asset_archive_name "${executable}" "${smoke_target}")"
  gh release download "${TAG}" \
    --repo "${REPO}" \
    --dir "${smoke_dir}" \
    --pattern "${smoke_archive}" \
    --pattern "${smoke_archive}.sha256"
  python3 scripts/release/binary_artifacts.py smoke \
    --version "${TAG}" \
    --target "${smoke_target}" \
    --executable "${executable}" \
    --archive "${smoke_dir}/${smoke_archive}" \
    --checksum "${smoke_dir}/${smoke_archive}.sha256"
}
