# Specification: v0.12.13 Quality & Performance Hardening

## Overview

This specification outlines the requirements for `v0.12.13`, focusing on the
decomposition of the remaining monolithic workflow module and elimination of
residual hotpath allocations.

## Scope

1. **Refactoring (`src/cli/workflow.rs`)**:
    * Split the 1013-line workflow module into responsibility-focused sub-modules:
      `common`, `check`, `fmt`, and `config_cmd`.
    * Each sub-module must have a size_score ≤ 400 after splitting.
    * The public API exposed via `src/cli/mod.rs` must remain unchanged.
1. **Performance Optimization**:
    * Remove the unnecessary intermediate `Vec` allocation in
      `md059.rs::normalize_link_text` (`split_whitespace().collect::<Vec<_>>().join()`).
    * Verify no performance regression with `make perf-check-strict`.
    * Update baseline with `make perf-refresh-baseline` only when improvements
      are confirmed.
1. **Public Confidence**:
    * Produce a verified `public-confidence-score.json` with a score of 100/100.
    * `make public-confidence` must report `unclassified_count: 0` and
      `release_blocking_issues: []`.

## Functional Requirements

* No changes to public command-line arguments.
* No changes to configuration file schemas.
* No regressions in existing AST and formatting precision.
* All tests, including `cli_convergence_contract` and `cli_path_context_contract`,
  must continue to pass exactly as they did in `v0.12.12`.

## Acceptance Criteria

* **Internal Quality**: `internal-quality-report.json` shows `src/cli/workflow.rs`
  removed from `split_candidates`, replaced by sub-modules each scoring ≤ 400.
* **Performance**: `make perf-check-strict` passes with `--max-ratio 1.4`;
  `md059.rs` change eliminates the intermediate `Vec` allocation.
* **Release Readiness**: The final `make release-check VERSION=v0.12.13` must
  pass without any manual interventions.
