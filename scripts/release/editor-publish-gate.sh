#!/usr/bin/env bash
set -euo pipefail

VSCODE_ENABLED="${PUBLISH_VSCODE_EXTENSION:-false}"
ZED_ENABLED="${PUBLISH_ZED_EXTENSION:-false}"

require_trusted_publishing_context() {
  registry="$1"
  if [[ "${GITHUB_ACTIONS:-false}" != "true" ]]; then
    echo "${registry} extension publish requires GitHub Actions. Local release checks never publish to marketplaces." >&2
    exit 1
  fi
  if [[ -z "${ACTIONS_ID_TOKEN_REQUEST_TOKEN:-}" || -z "${ACTIONS_ID_TOKEN_REQUEST_URL:-}" ]]; then
    echo "${registry} extension publish requires id-token: write so the marketplace can exchange a GitHub OIDC token." >&2
    exit 1
  fi
}

verify_vscode_metadata() {
  local package_json="editors/vscode/package.json"
  if [[ ! -f "$package_json" ]]; then
    echo "VS Code package.json not found at $package_json" >&2
    exit 1
  fi

  local publisher=$(grep '"publisher":' "$package_json" | cut -d'"' -f4)
  local name=$(grep '"name":' "$package_json" | cut -d'"' -f4)

  if [[ "$publisher" != "HiroyukiFuruno" ]]; then
    echo "Invalid VS Code publisher: $publisher (expected: HiroyukiFuruno)" >&2
    exit 1
  fi

  if [[ "$name" != "vscode-katana-markdown-linter" ]]; then
    echo "Invalid VS Code extension name: $name (expected: vscode-katana-markdown-linter)" >&2
    exit 1
  fi
}

verify_zed_metadata() {
  local extension_toml="editors/zed/extension.toml"
  if [[ ! -f "$extension_toml" ]]; then
    echo "Zed extension.toml not found at $extension_toml" >&2
    exit 1
  fi

  local id=$(grep '^id =' "$extension_toml" | cut -d'"' -f2)
  local authors=$(grep '^authors =' "$extension_toml")

  if [[ "$id" != "katana-markdown-linter" ]]; then
    echo "Invalid Zed extension id: $id (expected: katana-markdown-linter)" >&2
    exit 1
  fi

  if [[ "$authors" != *"Hiroyuki Furuno"* ]]; then
    echo "Invalid Zed extension authors: $authors (expected to contain: Hiroyuki Furuno)" >&2
    exit 1
  fi
}

if [[ "$VSCODE_ENABLED" == "true" ]]; then
  require_trusted_publishing_context "VS Code"
  verify_vscode_metadata
  echo "VS Code extension publish enabled."
else
  echo "VS Code extension publish deferred."
fi

if [[ "$ZED_ENABLED" == "true" ]]; then
  require_trusted_publishing_context "Zed"
  verify_zed_metadata
  echo "Zed extension publish enabled."
else
  echo "Zed extension publish deferred."
fi
