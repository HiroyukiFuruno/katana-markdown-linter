#!/usr/bin/env bash

verify_homebrew_formula() {
  formula_dist_dir="${TMP_ROOT}/homebrew-assets"
  formula_path="${VERIFY_OUTPUT_DIR}/homebrew/kml.rb"
  versioned_formula_path="${VERIFY_OUTPUT_DIR}/homebrew/kml@${VERSION_BARE}.rb"
  mkdir -p "${formula_dist_dir}" "$(dirname "${formula_path}")"
  for target in \
    x86_64-unknown-linux-gnu \
    x86_64-apple-darwin \
    aarch64-apple-darwin; do
    archive="kml-${TAG}-${target}.tar.gz"
    gh release download "${TAG}" \
      --repo "${REPO}" \
      --dir "${formula_dist_dir}" \
      --pattern "${archive}" \
      --pattern "${archive}.sha256"
  done
  python3 scripts/release/homebrew_formula.py generate \
    --version "${TAG}" \
    --formula-name kml \
    --repo "${REPO}" \
    --dist-dir "${formula_dist_dir}" \
    --output "${formula_path}" >/dev/null
  python3 scripts/release/homebrew_formula.py check \
    --version "${TAG}" \
    --formula-name kml \
    --repo "${REPO}" \
    --dist-dir "${formula_dist_dir}" \
    --output "${formula_path}" >/dev/null
  python3 scripts/release/homebrew_formula.py generate \
    --version "${TAG}" \
    --formula-name "kml@${VERSION_BARE}" \
    --repo "${REPO}" \
    --dist-dir "${formula_dist_dir}" \
    --output "${versioned_formula_path}" >/dev/null
  python3 scripts/release/homebrew_formula.py check \
    --version "${TAG}" \
    --formula-name "kml@${VERSION_BARE}" \
    --repo "${REPO}" \
    --dist-dir "${formula_dist_dir}" \
    --output "${versioned_formula_path}" >/dev/null
  verify_homebrew_tap_formula "Formula/kml.rb" "${formula_path}"
  verify_homebrew_tap_formula "Formula/kml@${VERSION_BARE}.rb" "${versioned_formula_path}"
}

verify_homebrew_tap_formula() {
  tap_path="$1"
  expected_path="$2"
  actual_path="${TMP_ROOT}/$(basename "${tap_path}")"
  raw_url="https://raw.githubusercontent.com/${HOMEBREW_TAP_REPO}/${HOMEBREW_TAP_BRANCH}/${tap_path}"
  python3 - "${raw_url}" "${actual_path}" <<'PY'
import sys
import urllib.request

url, output = sys.argv[1:3]
with urllib.request.urlopen(url, timeout=30) as response:
    content = response.read()
with open(output, "wb") as file:
    file.write(content)
PY
  if ! cmp -s "${expected_path}" "${actual_path}"; then
    echo "Homebrew tap formula mismatch: ${tap_path} does not match ${TAG}." >&2
    echo "tap:      ${raw_url}" >&2
    echo "expected: ${expected_path}" >&2
    exit 1
  fi
}
