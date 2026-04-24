#!/usr/bin/env sh
set -eu

if [ "$#" -eq 0 ]; then
  set -- tests/fixtures/upstream-golden-corpus
fi

: "${KML_MARKDOWNLINT_ORACLE_CMD:=npx -y markdownlint-cli2@latest}"

# Intentional word splitting lets callers pass a full command with arguments.
# shellcheck disable=SC2086
$KML_MARKDOWNLINT_ORACLE_CMD "$@"
