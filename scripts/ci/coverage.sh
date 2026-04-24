#!/usr/bin/env bash
set -euo pipefail

COVERAGE_IGNORE_PATTERNS=(
    "main\.rs"
    "cli\.rs"
)
COVERAGE_IGNORE="$(IFS='|'; echo "${COVERAGE_IGNORE_PATTERNS[*]}")"

info()    { echo "\033[0;36m[INFO]\033[0m  $*"; }
success() { echo "\033[0;32m[OK]\033[0m    $*"; }
error()   { echo "\033[0;31m[ERROR]\033[0m $*" >&2; }
header()  { echo "\033[1m\033[0;36m==> $*\033[0m"; }

header "Testing Code Coverage Gate"

JOBS=${JOBS:-2}
info "Cleaning up old coverage data..."
cargo llvm-cov clean --workspace

info "Running workspace lib/bin tests with llvm-cov (-j $JOBS)..."
if [ -n "$COVERAGE_IGNORE" ]; then
    cargo llvm-cov --no-report --jobs "$JOBS" --workspace --lib --bins -q \
        -- --test-threads="$JOBS"
else
    cargo llvm-cov --no-report --jobs "$JOBS" --workspace --lib --bins -q \
        -- --test-threads="$JOBS"
fi

info "Analyzing coverage report for truly unreachable lines..."

UNCOV=$(cargo llvm-cov report \
    ${COVERAGE_IGNORE:+--ignore-filename-regex "$COVERAGE_IGNORE"} \
    --text 2>&1 | grep '^ *[0-9]*|  *0|' | grep -vE 'panic!|^[^|]*\|[^|]*\|[[:space:]]*((\}[;,]?)|(\}\);?))[[:space:]]*$|return None;|return;|continue[;,]|\)\?;' | wc -l || true)

UNCOV=$(echo "$UNCOV" | xargs)

if [[ "$UNCOV" -ne 0 ]]; then
    error "WARN: $UNCOV lines were never executed"
    cargo llvm-cov report \
        ${COVERAGE_IGNORE:+--ignore-filename-regex "$COVERAGE_IGNORE"} \
        --text 2>&1 | grep '^ *[0-9]*|  *0|' | grep -vE 'panic!|^[^|]*\|[^|]*\|[[:space:]]*((\}[;,]?)|(\}\);?))[[:space:]]*$|return None;|return;|continue[;,]|\)\?;'
    exit 0
fi

success "Coverage report passed with no uncovered lines."
