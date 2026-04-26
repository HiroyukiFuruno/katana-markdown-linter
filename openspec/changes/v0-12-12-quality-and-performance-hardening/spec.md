# Specification: v0.12.12 Quality & Performance Hardening

## Overview
This specification outlines the requirements for `v0.12.12`, focusing on internal code quality refactoring, hotpath performance optimizations, and external quality confidence hardening.

## Scope
1.  **Refactoring (`src/cli.rs`)**:
    *   Separate the monolithic `src/cli.rs` into smaller, responsibility-focused modules.
    *   Address duplication and structure issues highlighted in `target/internal-quality-report.json`.
2.  **Performance Optimization**:
    *   Reduce unnecessary allocations (`.clone()`, `.to_string()`, `.collect::<Vec<_>>()`).
    *   Improve evaluation hotpaths.
    *   Update baseline after verified improvements.
3.  **Public Confidence**:
    *   Analyze and resolve/manage the outstanding `md-broken-link` false positives in the KatanA external corpus.
    *   Produce a verified `public-confidence-score.json` with a score of 100/100.

## Functional Requirements
*   No changes to public command-line arguments.
*   No changes to configuration file schemas.
*   No regressions in existing AST and formatting precision.
*   All tests, including `cli_convergence_contract`, must continue to pass exactly as they did in `v0.12.11`.

## Acceptance Criteria
*   **Internal Quality**: `internal-quality-report.json` shows a significant reduction in `size_score` for `src/cli.rs` and other targeted files.
*   **Performance**: `make perf-check-strict` passes with a `--max-ratio 1.4` (or better) and improvements are documented.
*   **Release Readiness**: The final `make release-check VERSION=v0.12.12` must pass without any manual interventions.