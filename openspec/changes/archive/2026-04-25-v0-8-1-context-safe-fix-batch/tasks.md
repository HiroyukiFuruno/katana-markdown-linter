## Definition of Ready

- [x] `safe-fix-coverage-continuous-expansion` is released and archived as `v0.8.0`.
- [x] `DocumentContext` exposes source-preserving fenced code block ranges and fence kind.
- [x] `MD048.style` official values are known: `consistent`, `backtick`, `tilde`.
- [x] Default fix mode remains safe-only.
- [x] Unsafe fix mode and formatter semantics are out of scope for this change.

## 1. MD048 Context Check

- [x] 1.1 Replace line-scan mixed-style detection with `DocumentContext::code_blocks()` evaluation.
- [x] 1.2 Honor configured `MD048.style = backtick`.
- [x] 1.3 Honor configured `MD048.style = tilde`.
- [x] 1.4 Honor default/configured `MD048.style = consistent` using the first fenced block style.
- [x] 1.5 Report diagnostics on offending fenced block ranges, not line 1.

## 2. MD048 Safe Fix

- [x] 2.1 Mark `MD048` official metadata as fixable for the implemented safe subset.
- [x] 2.2 Add `MD048` to the safe fix allow-list.
- [x] 2.3 Rewrite opening and closing fence markers while preserving marker length, indentation, info string, inner content, and trailing newline.
- [x] 2.4 Skip fix when the fenced block is unclosed.
- [x] 2.5 Skip fix when the target marker appears as an inner fence collision.
- [x] 2.6 Keep diagnostics visible when a fix is skipped by safety guards.

## 3. Coverage And Documentation

- [x] 3.1 Add unit tests for `consistent`, `backtick`, and `tilde` style behavior.
- [x] 3.2 Add a fixture matrix fix case for `MD048`.
- [x] 3.3 Add an edge fixture for the fence-collision guard.
- [x] 3.4 Add a golden corpus/baseline entry for `MD048`.
- [x] 3.5 Update rule coverage dashboard and fixture matrix summaries.
- [x] 3.6 Update README rule map and safe fix summary.
- [x] 3.7 Update rule fix feasibility notes.
- [x] 3.8 Add `v0.8.1` changelog entry.
- [x] 3.9 Bump crate version to `0.8.1`.

## Verification

- [x] `cargo fmt --all -- --check` succeeds.
- [x] `cargo test --workspace --locked` succeeds.
- [x] `cargo test --test rule_fixture_harness --locked` succeeds.
- [x] `cargo test --test upstream_golden_comparison --locked` succeeds.
- [x] `make dogfood` succeeds.
- [x] `git diff --check` succeeds.
- [x] `make release-check VERSION=v0.8.1` succeeds.

## Definition of Done

- [x] `MD048` check matches official `style` semantics for the implemented fenced-block cases.
- [x] `MD048` safe fix applies only when block-local marker conversion is collision-free.
- [x] Collision or unclosed-block cases remain diagnostic-only rather than silently rewriting.
- [x] README, dashboard, fixture matrix, changelog, and version metadata match the implementation.
- [x] `v0.8.1` is released and verified.
- [x] This change is archived after release.
