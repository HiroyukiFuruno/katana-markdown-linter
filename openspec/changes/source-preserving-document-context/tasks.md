## Definition of Ready

- [x] `v0-5-0-table-strategy-md060` is archived or otherwise confirmed complete
- [x] `performance-hotpath-competition` is archived or otherwise confirmed complete
- [x] mado/rumdl architecture findings have been reviewed and the selected direction is `DocumentContext` first, AST partial/lazy second
- [x] Current benchmark baseline is available via `make bench` / `make perf-check`
- [x] Current upstream golden and fixture matrix gates are passing or known failures are documented

## 0. Planning Contract

- [x] 0.1 Add delta spec requirements for `DocumentContext`, partial AST, compatibility, and performance gates
- [x] 0.2 Confirm implementation excludes unsafe fix mode and MCP productization
- [x] 0.3 Confirm dirty main worktree changes are isolated before implementation starts

## 1. Context Contract

- [x] 1.1 Define `DocumentContext` / `MarkdownDocument` data model
- [x] 1.2 Define line offset and line/column conversion helpers
- [x] 1.3 Define source-preserving range types for internal use
- [x] 1.4 Define eager vs lazy fields and document why each field is eager or lazy
- [x] 1.5 Add unit tests for empty input, no trailing newline, CRLF, Unicode, and mixed Markdown constructs

## 2. Structural Indexes

- [x] 2.1 Implement heading extraction
- [x] 2.2 Implement code block and code fence range extraction
- [x] 2.3 Implement reference extraction
- [x] 2.4 Implement table block extraction
- [x] 2.5 Add fixture tests that compare extracted structures with expected source ranges

## 3. AST Partial Introduction

- [x] 3.1 Choose parser dependency and feature shape based on measured need
- [x] 3.2 Add optional/lazy AST construction behind `DocumentContext`
- [x] 3.3 Prove AST construction is not required for simple line-only rules
- [x] 3.4 Document mismatch policy when AST interpretation differs from markdownlint expectations
- [x] 3.5 Keep AST structures out of the public API unless a follow-up change explicitly accepts that contract

## 4. Rule API Migration

- [x] 4.1 Add context-based rule evaluation API with legacy adapter
- [x] 4.2 Migrate one heading rule family to context-based evaluation
- [x] 4.3 Migrate one reference or table rule family to context-based evaluation
- [x] 4.4 Ensure configured rule evaluation uses the same context path
- [x] 4.5 Keep public `lint` and `fix` API backward compatible

## 5. Fix Safety And Convergence

- [x] 5.1 Ensure migrated rules generate fixes from original source ranges
- [x] 5.2 Add regression tests for overlapping fixes across migrated rule families
- [x] 5.3 Record which fixes remain line-local and which require structural context
- [x] 5.4 Leave unsafe fix mode to `unsafe-fix-mode-and-confirmation`

## 6. Performance And Verification

- [x] 6.1 Capture before/after benchmark report
- [x] 6.2 Add microbenchmarks for migrated rule families
- [x] 6.3 Run upstream golden comparison
- [x] 6.4 Run rule fixture harness
- [x] 6.5 Run dogfood

## Verification

- [x] `cargo fmt --all -- --check`
- [x] `cargo test --workspace --locked`
- [x] `cargo test --test ast_linter --locked`
- [x] `cargo test --test rule_fixture_harness --locked`
- [x] `cargo test --test upstream_golden_comparison --locked`
- [x] `make dogfood`
- [x] `make perf-check`
- [x] `git diff --check`

## Definition of Done

- [x] `DocumentContext` is the preferred internal rule input for migrated rules
- [x] AST is introduced only as optional/lazy support, not as the sole source of fix ranges
- [x] At least one heading family and one reference/table/fence family use context-based evaluation
- [x] Existing public library APIs remain source-compatible
- [x] Benchmarks and golden fixtures show no unexplained regression
