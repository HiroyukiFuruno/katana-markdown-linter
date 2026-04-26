## Definition of Ready

- [x] `v0.4.0` is released and verified.
- [x] `safe-fix-strategy-expansion` is archived.
- [x] `unsafe-fix-mode-and-confirmation` remains separate from default safe fixes.
- [x] Official `MD060` documentation has been checked for `style` and `aligned_delimiter` behavior.
- [x] README rule map state terminology is agreed as `Check`, `Safe fix`, and `Unsafe fix`.

## 1. MD060 Table Strategy

- [x] 1.1 Add a table-block parser for `MD060`.
- [x] 1.2 Detect official `aligned`, `compact`, and `tight` style matches.
- [x] 1.3 Apply `style` values `any`, `aligned`, `compact`, and `tight`.
- [x] 1.4 Apply `aligned_delimiter=true`.
- [x] 1.5 Ignore fenced code blocks.

## 2. Safe Fix

- [x] 2.1 Add table-block formatter for compact style.
- [x] 2.2 Add table-block formatter for tight style.
- [x] 2.3 Add table-block formatter for aligned style with visual width.
- [x] 2.4 Return one whole-table fix per safe table block.
- [x] 2.5 Skip safe fix for inconsistent column counts and ambiguous pipe parsing.

## 3. Fixtures And Golden Coverage

- [x] 3.1 Add MD060 check pass fixtures for aligned, compact, and tight.
- [x] 3.2 Add MD060 check fail fixtures for mixed style tables.
- [x] 3.3 Add MD060 config fixtures for explicit styles and aligned delimiter.
- [x] 3.4 Add MD060 safe fix before/after fixtures.
- [x] 3.5 Update upstream golden baseline or known-delta state for MD060.

## 4. README And Dashboard State

- [x] 4.1 Change README rule map from `Safe fix yes/no` to state columns.
- [x] 4.2 Update AST lint so README state rows stay synchronized.
- [x] 4.3 Update dashboard/manual-required state for MD060.
- [x] 4.4 Keep official `fixable` metadata unchanged.

## Verification

- [x] `cargo test --test rule_fixture_harness --locked` succeeds.
- [x] `cargo test --test upstream_golden_comparison --locked` succeeds.
- [x] `cargo test --workspace --locked` succeeds.
- [x] `make check` succeeds.
- [x] `make dogfood` succeeds without baseline growth.
- [x] `git diff --check` succeeds.

## Definition of Done

- [x] `MD060` no longer flags official aligned/compact/tight examples as violations.
- [x] `MD060` can safely fix at least compact, tight, and aligned table style subsets.
- [x] Unsafe table rewrites remain out of default fix mode.
- [x] README expresses rule support as states, not yes/no booleans.
- [x] OpenSpec tasks reflect actual implementation and verification status.
