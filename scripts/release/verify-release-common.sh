#!/usr/bin/env bash

cleanup() {
  [[ -z "${TMP_ROOT}" ]] || rm -rf "${TMP_ROOT}"
}

require_command() {
  local command_name="$1"
  command -v "${command_name}" >/dev/null 2>&1 && return
  echo "${command_name} is required to verify ${TAG} publication." >&2
  exit 1
}

assert_equal() {
  local label="$1"
  local expected="$2"
  local actual="$3"
  [[ "${actual}" == "${expected}" ]] && return
  echo "${label} mismatch: expected ${expected}, got ${actual:-missing}" >&2
  exit 1
}

release_asset_exists() {
  local asset_name="$1"
  [[ $'\n'"${asset_names}"$'\n' == *$'\n'"${asset_name}"$'\n'* ]]
}

current_smoke_target() {
  case "$(uname -s)/$(uname -m)" in
    Linux/x86_64) echo "x86_64-unknown-linux-gnu" ;;
    Darwin/x86_64) echo "x86_64-apple-darwin" ;;
    Darwin/arm64) echo "aarch64-apple-darwin" ;;
    *) echo "" ;;
  esac
}

release_asset_archive_name() {
  local executable="$1"
  local target="$2"
  local extension="tar.gz"
  if [[ "${target}" == x86_64-pc-windows-msvc ]]; then
    extension="zip"
  fi
  echo "${executable}-${TAG}-${target}.${extension}"
}
