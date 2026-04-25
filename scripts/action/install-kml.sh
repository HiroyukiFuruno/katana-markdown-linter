#!/usr/bin/env bash
set -euo pipefail

install_source="${KML_ACTION_INSTALL_SOURCE:-crates-io}"
version="${KML_ACTION_VERSION:-}"
install_path="${KML_ACTION_PATH:-.}"
install_root="${KML_ACTION_INSTALL_ROOT:-${RUNNER_TEMP:-/tmp}/kml-action}"
bin_dir="${install_root}/bin"

mkdir -p "${install_root}"

case "${install_source}" in
  crates-io)
    command=(cargo install katana-markdown-linter --locked --force --root "${install_root}" --bin kml)
    if [[ -n "${version}" ]]; then
      command+=(--version "${version}")
    fi
    ;;
  path)
    command=(cargo install --path "${install_path}" --locked --force --root "${install_root}" --bin kml)
    ;;
  *)
    echo "unsupported install-source: ${install_source}" >&2
    exit 2
    ;;
esac

"${command[@]}"

if [[ ! -x "${bin_dir}/kml" ]]; then
  echo "kml binary was not installed under ${bin_dir}" >&2
  exit 2
fi

if [[ -n "${GITHUB_PATH:-}" ]]; then
  printf '%s\n' "${bin_dir}" >>"${GITHUB_PATH}"
else
  printf 'KML_ACTION_BIN_DIR=%s\n' "${bin_dir}"
fi
