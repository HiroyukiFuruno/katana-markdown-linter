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

if [[ "$VSCODE_ENABLED" == "true" ]]; then
  require_trusted_publishing_context "VS Code"
  echo "VS Code extension publish enabled."
else
  echo "VS Code extension publish deferred."
fi

if [[ "$ZED_ENABLED" == "true" ]]; then
  require_trusted_publishing_context "Zed"
  echo "Zed extension publish enabled."
else
  echo "Zed extension publish deferred."
fi
