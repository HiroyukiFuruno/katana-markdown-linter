#!/usr/bin/env bash
set -euo pipefail

EVENT_NAME="${1:?event name is required}"
INPUT_VERSION="${2:-}"
PR_HEAD_REF="${3:-}"

# Strip potential trailing carriage return (common in Windows CI environments)
EVENT_NAME="${EVENT_NAME%$'\r'}"
INPUT_VERSION="${INPUT_VERSION%$'\r'}"
PR_HEAD_REF="${PR_HEAD_REF%$'\r'}"

if [[ "${EVENT_NAME}" == "pull_request" ]]; then
  if [[ ! "${PR_HEAD_REF}" =~ ^release/v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Release branch must be exactly release/vX.Y.Z: ${PR_HEAD_REF}" >&2
    exit 1
  fi

  scripts/release/verify-version.sh "${PR_HEAD_REF#release/}"
elif [[ -n "${INPUT_VERSION}" ]]; then
  scripts/release/verify-version.sh "${INPUT_VERSION}"
else
  scripts/release/verify-version.sh
fi
