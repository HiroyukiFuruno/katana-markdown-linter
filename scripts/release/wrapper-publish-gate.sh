#!/usr/bin/env bash
set -euo pipefail

NPM_ENABLED="${PUBLISH_NPM_WRAPPER:-false}"
PYPI_ENABLED="${PUBLISH_PYPI_WRAPPER:-false}"

require_trusted_publishing_context() {
  registry="$1"
  if [[ "${GITHUB_ACTIONS:-false}" != "true" ]]; then
    echo "${registry} wrapper publish requires GitHub Actions trusted publishing. Local release checks never publish registry packages." >&2
    exit 1
  fi
  if [[ -z "${ACTIONS_ID_TOKEN_REQUEST_TOKEN:-}" || -z "${ACTIONS_ID_TOKEN_REQUEST_URL:-}" ]]; then
    echo "${registry} wrapper publish requires id-token: write so the registry can exchange a GitHub OIDC token." >&2
    exit 1
  fi
}

if [[ "$NPM_ENABLED" == "true" ]]; then
  require_trusted_publishing_context "npm"
  echo "npm wrapper publish enabled through trusted publishing."
else
  echo "npm wrapper publish deferred."
fi

if [[ "$PYPI_ENABLED" == "true" ]]; then
  require_trusted_publishing_context "PyPI"
  echo "PyPI wrapper publish enabled through trusted publishing."
else
  echo "PyPI wrapper publish deferred."
fi
