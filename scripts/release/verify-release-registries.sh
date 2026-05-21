#!/usr/bin/env bash

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

smoke_npm_mcp_wrapper() {
  require_command npx
  python3 scripts/ci/mcp-stdio-smoke.py \
    --command npx --yes "${PACKAGE}@${VERSION_BARE}" kml-mcp
  python3 scripts/ci/mcp-remote-smoke.py \
    --command npx --yes "${PACKAGE}@${VERSION_BARE}" kml-mcp-remote
  if command -v bunx >/dev/null 2>&1; then
    python3 scripts/ci/mcp-stdio-smoke.py \
      --command bunx --package "${PACKAGE}@${VERSION_BARE}" kml-mcp
  else
    echo "bunx is not installed; skipping post-release bunx MCP smoke."
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

smoke_pypi_mcp_wrapper() {
  require_command uvx
  python3 scripts/ci/mcp-stdio-smoke.py \
    --command uvx --from "${PACKAGE}==${VERSION_BARE}" kml-mcp
  python3 scripts/ci/mcp-remote-smoke.py \
    --command uvx --from "${PACKAGE}==${VERSION_BARE}" kml-mcp-remote
}

verify_vscode_extension() {
  if [[ "${PUBLISH_VSCODE_EXTENSION:-false}" == "true" ]]; then
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

verify_consistency_with_state() {
  state_path="target/release-verify-state.json"
  if [[ ! -f "${state_path}" ]]; then
    echo "Verification state file missing: ${state_path}" >&2
    exit 1
  fi

  expected_version="$(python3 -c "import json; print(json.load(open('${state_path}'))['version'])")"
  if [[ "${TAG}" != "${expected_version}" ]]; then
    echo "Verification drift detected: expected version ${expected_version} from state, but verifying ${TAG}." >&2
    exit 1
  fi

  release_decision="$(python3 -c "import json; print(json.load(open('${state_path}'))['release_decision'])")"
  if [[ "${release_decision}" != "allow_release" ]]; then
    echo "Verification state indicates blockers: ${release_decision}" >&2
    python3 -c "import json; print('\n'.join(json.load(open('${state_path}'))['publish_blockers']))" >&2
    exit 1
  fi
  echo "Consistency check passed with ${state_path}"
}
