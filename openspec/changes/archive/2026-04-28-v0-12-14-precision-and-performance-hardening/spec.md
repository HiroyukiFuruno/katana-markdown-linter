# Specification: v0.12.14 Precision & Performance Hardening

## Overview

This specification outlines the requirements for `v0.12.14`, focusing on hot path
deduplication and regex / index reuse in shared rule infrastructure. No user-facing
behavior changes are introduced; the release is performance-only with strict
precision-zero-regression as a release-blocking gate.

## Scope

1. **Inline Scanner Consolidation (`DocumentContext`)**:
    * Introduce a single `InlineIndex` that builds inline code spans, inline HTML
      elements, inline links, and reference definitions in one pass over the
      document lines.
    * Existing public methods (`inline_code_spans`, `inline_html_elements`,
      `inline_links`, `reference_definitions`) must delegate to the shared index
      with their external signatures unchanged.
2. **MD051 ignored_pattern Caching**:
    * Compile the `ignored_pattern` regex once per `Md051` instance instead of on
      each `evaluate_context` call.
    * Behavior on empty or invalid patterns must match the current implementation.
3. **MD046 code_line_flags Lookup**:
    * Replace `O(n)` `code_blocks.iter().any(...)` lookups with the existing
      `DocumentContext::code_line_flags()` `O(1)` index.
    * Style-distinguishing logic (fenced vs indented) remains via the existing
      block iteration and is not inlined.
4. **Roadmap Update**:
    * Register `v0.12.14` in `active-roadmap.md`.
    * Park the precision-side fix+ candidates (MD052, MD046, MD043, MD056, MD034,
      MD051 fragment refinement) as future entries.
    * Defer the MD013 wrap fix into a future unsafe-fix change.

## Functional Requirements

* No changes to public command-line arguments.
* No changes to configuration file schemas.
* No changes to rule diagnostic identifiers, severities, or messages.
* No changes to fix output for any rule.
* All tests, including `cli_convergence_contract`, `cli_path_context_contract`,
  and `public_confidence_contract`, must continue to pass with byte-identical
  output where applicable.

## Acceptance Criteria

* **Behavior Equivalence**: For every corpus exercised by `tests/ast_linter.rs`
  and `tests/public_confidence_contract.rs`, diagnostic counts, ids, ranges, and
  fix payloads remain bitwise unchanged.
* **Performance**: `make perf-check-strict` passes with `ratio ≤ 1.40x`. When
  improvements are confirmed, `make perf-refresh-baseline` may be run to lower
  the committed baseline.
* **Internal Quality**: `internal-quality-report.json` shows no regression in
  `split_candidates` and at least neutral movement on hot path file scores.
* **Public Confidence**: `make public-confidence` reports `unclassified_count: 0`
  and `release_blocking_issues: []`. `public-confidence-score.json` is recorded
  with score 100/100.
* **Release Readiness**: `make release-check VERSION=v0.12.14` passes with no
  manual intervention.
