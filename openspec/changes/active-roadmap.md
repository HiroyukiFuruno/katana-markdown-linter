# Active Roadmap

This file maps visible post-`v0.3.0` work areas to OpenSpec changes.

## Version Direction

- `v0.4.0`: check/fix 拡充を優先する。default fix は safe subset のみを対象にする。
- `v0.5.0`: `DocumentContext` 主体の AST 部分導入を優先する。unsafe fix mode は別 change として扱い、default safe fix には混ぜない。
- `v0.6.0`: i18n を library / CLI / MCP の product surface として拡充し、将来 locale 追加のための coverage gate と API 安定化を優先する。
- `v0.7.0`: performance hot path を優先する。まず fix range application の繰り返し走査を削減し、cache/watch は別 change に分離する。
- `v0.8.0`: linter と safe fix の精度を優先する。未対応で安全に対応できる rule/fix を継続拡張する。
- `v0.8.1`: `DocumentContext` を使って小さな safe-fix 拡張を継続する。`MD048` の fence style check/fix を unsafe mode より先に閉じる。
- `v0.9.0`: unsafe fix mode を追加する。default safe fix contract は維持し、interactive / automation opt-in を必須にする。
- `v0.10.0`: formatter productization を検討する。lint-driven fix とは別の deterministic / idempotent policy として設計する。
- `v0.11.0`: `rumdl` を参考に、distribution / editor / automation integration を拡張する。

| Priority | Work Area | Change | Why Now |
| --- | --- | --- | --- |
| Done | Golden and edge coverage for `v0.4.0` | `golden-edge-coverage-expansion` | Completed for `v0.4.0`; dashboard now derives golden status from the locked baseline and records edge coverage. |
| Done | Safe check/fix expansion for `v0.4.0` | `safe-fix-strategy-expansion` | Completed for `v0.4.0`; `MD005` and `MD030` safe subsets are locked, while `MD060` remains diagnostic/manual-required because official metadata marks it non-fixable. |
| Done | Table strategy for `v0.5.0` | `v0-5-0-table-strategy-md060` | Completed for `v0.5.0`; `MD060` now has table-block parsing, official style checks, and safe fix subsets. |
| Done | Source-preserving document context for `v0.5.0` | `source-preserving-document-context` | Completed for `v0.5.0`; `DocumentContext` now shares source-preserving structure across migrated rule families. |
| Done | i18n product surface for `v0.6.0` | `v0-6-0-i18n-expansion-draft` | Completed for `v0.6.0`; Rust API, CLI rule/config output, MCP metadata, and translation coverage gates now share stable localized metadata. |
| Done | Performance hot path for `v0.7.0` | `v0-7-0-performance-hotpath` | Completed for `v0.7.0`; fix range application now uses a per-apply line offset index and the release is published. |
| Done | Linter precision and safe fix expansion for `v0.8.0` | `safe-fix-coverage-continuous-expansion` | Completed for `v0.8.0`; `MD035` and `MD055` safe fixes are fixture/golden locked and the release is published. |
| P1 | Context-safe fix batch for `v0.8.1` | `v0-8-1-context-safe-fix-batch` | Close the `MD048` safe subset while the required block ranges and collision guards are already available. |
| P2 | Unsafe fix mode for `v0.9.0` | `unsafe-fix-mode-and-confirmation` | Unsafe fix requires explicit user opt-in, CLI confirmation, and automation guardrails. |
| P2 | Formatter productization for `v0.10.0` | `formatter-productization` | Separate formatter semantics from lint-driven safe fixes after linter precision is stronger. |
| P3 | Tool distribution and editor expansion for `v0.11.0` | `tool-distribution-and-editor-expansion` | Use rumdl as a reference for distribution, pre-commit, GitHub Action, schema, and editor workflows without copying implementation. |
| P2 | MCP productization | `mcp-workspace-tools-productization` | Current MCP server is experimental and text-first only. |
| Done | Performance hot path work | `performance-hotpath-competition` | Completed; docs now include baseline, profile summary, before/after, and local regression guidance. |

Archived completed changes:

- `golden-edge-coverage-expansion` -> `openspec/changes/archive/2026-04-25-golden-edge-coverage-expansion`
- `safe-fix-strategy-expansion` -> `openspec/changes/archive/2026-04-25-safe-fix-strategy-expansion`
- `v0-5-0-table-strategy-md060` -> `openspec/changes/archive/2026-04-25-v0-5-0-table-strategy-md060`
- `performance-hotpath-competition` -> `openspec/changes/archive/2026-04-25-performance-hotpath-competition`
- `v0-6-0-i18n-expansion-draft` -> `openspec/changes/archive/2026-04-25-v0-6-0-i18n-expansion-draft`
- `v0-7-0-performance-hotpath` -> `openspec/changes/archive/2026-04-25-v0-7-0-performance-hotpath`
- `safe-fix-coverage-continuous-expansion` -> `openspec/changes/archive/2026-04-25-safe-fix-coverage-continuous-expansion`

## Suggested Order

1. Apply `v0-8-1-context-safe-fix-batch` for `v0.8.1`.
2. Apply `unsafe-fix-mode-and-confirmation` for `v0.9.0`.
3. Apply `formatter-productization` for `v0.10.0`.
4. Apply `tool-distribution-and-editor-expansion` for `v0.11.0`.
5. Apply `mcp-workspace-tools-productization` when workspace MCP usage is prioritized.
6. Use `source-preserving-document-context` as the baseline for later structural fix work.

## Repository Guardrails

- New OpenSpec change files are ignored by the current `.gitignore`; commit them with explicit `git add -f openspec/changes/<change>` when they should be tracked.
- `check-diagnostic-i18n` was archived as `openspec/changes/archive/2026-04-25-check-diagnostic-i18n`.
- Keep one implementation change active at a time unless the write sets are disjoint and the roadmap is updated to show the parallelism.
- `formatter-productization` must not be mixed into `v0.8.0`; linter precision and safe fix coverage come first.
- `unsafe-fix-mode-and-confirmation` must not be mixed into `v0.7.0` or `v0.8.0`; it changes CLI safety semantics and belongs to `v0.9.0`.
