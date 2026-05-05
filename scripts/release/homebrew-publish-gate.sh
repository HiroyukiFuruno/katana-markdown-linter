#!/usr/bin/env bash
set -euo pipefail

HOMEBREW_ENABLED="${PUBLISH_HOMEBREW:-false}"

require_trusted_publishing_context() {
  registry="$1"
  if [[ "${GITHUB_ACTIONS:-false}" != "true" ]]; then
    echo "${registry} publish requires GitHub Actions. Local release checks never publish to taps." >&2
    exit 1
  fi
  # For Homebrew, we usually need a GitHub token with repo scope,
  # but we still want to ensure we are in a CI environment if we intent to publish.
  if [[ -z "${HOMEBREW_KATANA_GIT_TOKEN:-}" ]]; then
    echo "${registry} publish requires HOMEBREW_KATANA_GIT_TOKEN." >&2
    exit 1
  fi
}

if [[ "$HOMEBREW_ENABLED" == "true" ]]; then
  require_trusted_publishing_context "Homebrew"
  echo "Homebrew tap update enabled."
else
  echo "Homebrew tap update deferred."
fi
