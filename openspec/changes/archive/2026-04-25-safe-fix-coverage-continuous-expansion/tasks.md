# Tasks

## Definition Of Ready

- [x] `v0-7-0-performance-hotpath` is complete or not blocking rule implementation.
- [x] README Rule Map shows active, deleted, safe, and unsafe status clearly.
- [x] Rule fixture matrix is the source of truth for safe fix coverage.
- [x] Formatter productization remains out of scope.

## 1. Feasibility Review

- [x] 1.1 Classify each diagnostic-only rule as `safe-now`, `safe-after-context`, `unsafe-candidate`, or `manual-required`.
- [x] 1.2 Write the classification into the coverage dashboard or a referenced document.
- [x] 1.3 Select the first implementation batch with the lowest correctness risk.

## 2. Safe Fix Implementation

- [x] 2.1 Add safe fixes for the selected first batch.
- [x] 2.2 Add edge fixtures that prove ignored code blocks, front matter, tables, and reference syntax stay safe.
- [x] 2.3 Re-check fixed output to ensure fixes are idempotent.
- [x] 2.4 Update README Rule Map status.

## 3. Check Precision

- [x] 3.1 Fix any false positives found while adding safe fixes.
- [x] 3.2 Add regression fixtures for each false positive.
- [x] 3.3 Keep known upstream deltas explicit.

## Verification

- [x] `cargo fmt --all -- --check` passes.
- [x] `cargo test --workspace --locked` passes.
- [x] `cargo test --test rule_fixture_harness --locked` passes.
- [x] `cargo test --test upstream_golden_comparison --locked` passes.
- [x] `make dogfood` passes.
- [x] `git diff --check` passes.
- [x] `make release-check VERSION=v0.8.0` passes.

## Definition Of Done

- [x] At least one safe-fix batch is implemented with fixture and golden coverage.
- [x] Each non-implemented candidate has a documented reason.
- [x] README and dashboard show the new safe-fix state.
- [x] No formatter or unsafe semantics are introduced.
