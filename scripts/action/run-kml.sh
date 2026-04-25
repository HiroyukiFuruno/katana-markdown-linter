#!/usr/bin/env bash
set -euo pipefail

command_name="${KML_ACTION_COMMAND:-check}"
paths="${KML_ACTION_PATHS:-.}"
config="${KML_ACTION_CONFIG:-}"
locale="${KML_ACTION_LOCALE:-en}"
output="${KML_ACTION_OUTPUT:-text}"
extra_args="${KML_ACTION_EXTRA_ARGS:-}"

case "${command_name}" in
  check | fix | fmt | rule | config | version | init-config) ;;
  *)
    echo "unsupported kml command: ${command_name}" >&2
    exit 2
    ;;
esac

args=("${command_name}")

if [[ -n "${config}" ]]; then
  args+=(--config "${config}")
fi

if [[ -n "${locale}" ]]; then
  args+=(--locale "${locale}")
fi

if [[ -n "${output}" && "${output}" != "text" ]]; then
  args+=(--output "${output}")
fi

append_multiline_args() {
  local value="$1"
  local line

  while IFS= read -r line || [[ -n "${line}" ]]; do
    if [[ -n "${line}" ]]; then
      args+=("${line}")
    fi
  done <<<"${value}"
}

append_multiline_args "${extra_args}"

case "${command_name}" in
  check | fix | fmt)
    append_multiline_args "${paths}"
    ;;
esac

kml "${args[@]}"
