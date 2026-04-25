# Tasks

## Definition Of Ready

- [ ] `v0-7-0-performance-hotpath` is complete or not blocking rule implementation.
- [ ] README Rule Map shows active, deleted, safe, and unsafe status clearly.
- [ ] Rule fixture matrix is the source of truth for safe fix coverage.
- [ ] Formatter productization remains out of scope.

## 1. Feasibility Review

- [ ] 1.1 Classify each diagnostic-only rule as `safe-now`, `safe-after-context`, `unsafe-candidate`, or `manual-required`.
- [ ] 1.2 Write the classification into the coverage dashboard or a referenced document.
- [ ] 1.3 Select the first implementation batch with the lowest correctness risk.

## 2. Safe Fix Implementation

- [ ] 2.1 Add safe fixes for the selected first batch.
- [ ] 2.2 Add edge fixtures that prove ignored code blocks, front matter, tables, and reference syntax stay safe.
- [ ] 2.3 Re-check fixed output to ensure fixes are idempotent.
- [ ] 2.4 Update README Rule Map status.

## 3. Check Precision

- [ ] 3.1 Fix any false positives found while adding safe fixes.
- [ ] 3.2 Add regression fixtures for each false positive.
- [ ] 3.3 Keep known upstream deltas explicit.

## Verification

- [ ] `cargo fmt --all -- --check` passes.
- [ ] `cargo test --workspace --locked` passes.
- [ ] `cargo test --test rule_fixture_harness --locked` passes.
- [ ] `cargo test --test upstream_golden_comparison --locked` passes.
- [ ] `make dogfood` passes.
- [ ] `git diff --check` passes.

## Definition Of Done

- [ ] At least one safe-fix batch is implemented with fixture and golden coverage.
- [ ] Each non-implemented candidate has a documented reason.
- [ ] README and dashboard show the new safe-fix state.
- [ ] No formatter or unsafe semantics are introduced.
