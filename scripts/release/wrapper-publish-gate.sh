#!/usr/bin/env bash
set -euo pipefail

NPM_ENABLED="${PUBLISH_NPM_WRAPPER:-false}"
PYPI_ENABLED="${PUBLISH_PYPI_WRAPPER:-false}"

if [[ "$NPM_ENABLED" == "true" ]]; then
  if [[ "${GITHUB_ACTIONS:-false}" != "true" ]]; then
    echo "npm wrapper publish requires GitHub Actions trusted publishing." >&2
    exit 1
  fi
  echo "npm wrapper publish enabled through trusted publishing."
else
  echo "npm wrapper publish deferred."
fi

if [[ "$PYPI_ENABLED" == "true" ]]; then
  if [[ "${GITHUB_ACTIONS:-false}" != "true" ]]; then
    echo "PyPI wrapper publish requires GitHub Actions trusted publishing." >&2
    exit 1
  fi
  echo "PyPI wrapper publish enabled through trusted publishing."
else
  echo "PyPI wrapper publish deferred."
fi
