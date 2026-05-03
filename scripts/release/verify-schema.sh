#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCHEMA_FILE="${REPO_ROOT}/schema/markdownlint.schema.json"
TEMP_SCHEMA="$(mktemp)"

trap 'rm -f "${TEMP_SCHEMA}"' EXIT

echo "Generating current schema..."
cargo run --quiet --bin kml -- config schema --output json > "${TEMP_SCHEMA}"

if ! diff -u "${SCHEMA_FILE}" "${TEMP_SCHEMA}"; then
  echo "Error: Committed schema '${SCHEMA_FILE}' does not match 'kml config schema' output." >&2
  echo "If this change is intentional, update the committed schema file:" >&2
  echo "  cargo run --bin kml -- config schema --output json > schema/markdownlint.schema.json" >&2
  exit 1
fi

echo "Schema consistency check passed."
