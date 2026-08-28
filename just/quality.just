# ============================================================
# Quality
# ============================================================

# Apply code formatting (rustfmt)
fmt:
    cargo fmt --all

# Check format differences (for CI)
fmt-check:
    cargo fmt --all -- --check

# Verify config schema consistency
schema-check:
    scripts/release/verify-schema.sh

# Run Clippy (forces zero warnings)
lint:
    cargo clippy -j {{JOBS}} --workspace --all-targets --all-features --locked -- -D warnings

# Run Clippy and apply automatic fixes
lint-fix:
    cargo clippy --workspace --all-targets --all-features --fix --allow-dirty --allow-staged -- -D warnings

# Run AST-based custom linters
ast-lint:
    cargo test -j {{JOBS}} --test ast_linter -- --nocapture

# Run upstream markdownlint default-branch drift gate (requires KML_UPSTREAM_MARKDOWNLINT_DOC_DIR)
upstream-drift:
    cargo test upstream_default_branch_drift_has_no_unknown_items --all-features --locked -- --ignored

# Run deterministic upstream markdownlint golden comparison
upstream-golden:
    cargo test --test upstream_golden_comparison --locked

# Run live upstream markdownlint oracle against golden corpus
upstream-golden-live:
    sh scripts/upstream/markdownlint-oracle.sh tests/fixtures/upstream-golden-corpus

# Regenerate docs/rule-coverage-dashboard.md from fixture metadata
rule-dashboard:
    cargo run --quiet --example rule_coverage_dashboard --locked -- tests/fixtures/rule-fixture-matrix.json tests/fixtures/upstream-golden-baseline.json tests/fixtures/upstream-golden-known-deltas.json docs/rule-coverage-dashboard.md

# Verify issue-driven dependency update/branch contract behavior.
issue-driven-hook-contract-check:
    bash scripts/ci/issue-driven-hook-contracts.sh

# Run unit tests
test:
    cargo test --workspace

# Report uncovered lines without failing the build (requires cargo-llvm-cov)
coverage:
    JOBS={{JOBS}} scripts/ci/coverage.sh

# Fail when uncovered lines exceed scripts/ci/coverage-baseline.txt
coverage-blocking:
    COVERAGE_MODE=blocking JOBS={{JOBS}} scripts/ci/coverage.sh

# Fast impacted verification (local default)
check: fmt-check lint ast-lint test dogfood issue-driven-hook-contract-check
    @echo "All checks passed"

# Compile public Rust embedding examples
examples:
    cargo build --examples --locked

# Capture internal code quality evidence for src, tests, and build.rs
internal-quality-check:
    python3 scripts/ci/internal-quality.py --report target/internal-quality-report.json --src src --extra-paths tests build.rs
