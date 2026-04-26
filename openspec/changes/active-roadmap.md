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
- `v0.12.0`: MCP workspace tools を productize する。local stdio server として file/directory check と explicit apply を固定する。
- `v0.12.2`: released patch として CI/CD parity、Windows 検証、cache strategy、誤検知回帰 fixture を締め直した。
- `v0.12.3`: released patch として lint 精度、速度（performance）、単体テスト（UT）/結合テスト（IT）拡充を完了した。
- `v0.12.4`: 展開より前の patch として、`linter` / `check` / `fix` / `fmt` の正しさ、収束性、冪等性、速度、release gate を固める。
- `v0.12.5`: 安定版に向けた AST readiness と parser precision を優先する。link / inline code / reference の共有 parser と性能計測を作る。
- `v0.12.6`: released patch として context-sensitive rule を parser / `DocumentContext` へ移譲し、単一行文字列だけに依存する rule を減らした。
- `v0.12.7`: released patch として parser / context migration 後の performance、check / fix / fmt convergence、baseline evidence を固めた。
- `v0.12.8`: released patch として stable score 100 点、hard blocker 0 件、ユーザー受け入れ判断を記録した。
- `v0.13.0`: `v0.12.8` の stable score 90 点以上、hard blocker 0 件、ユーザー受け入れ完了を満たしてから、MCP Registry / Hub 公開前の配布方式、`server.json`、security gate を決める。公開自体はまだ行わない。`v0.12.9` を継続するかは `v0.12.8` release 後に議論する。
- `v0.14.0`: `v0.13.0` で選んだ package artifact と Registry metadata を実装し、公開まで進める。
- `v0.15.0`: API-hosted LLM から直接使う必要が出た場合だけ、遠隔 MCP 接続（remote MCP transport）を設計・実装する。

| Priority | Work Area | Change | Why Now |
| --- | --- | --- | --- |
| Done | Golden and edge coverage for `v0.4.0` | `golden-edge-coverage-expansion` | Completed for `v0.4.0`; dashboard now derives golden status from the locked baseline and records edge coverage. |
| Done | Safe check/fix expansion for `v0.4.0` | `safe-fix-strategy-expansion` | Completed for `v0.4.0`; `MD005` and `MD030` safe subsets are locked, while `MD060` remains diagnostic/manual-required because official metadata marks it non-fixable. |
| Done | Table strategy for `v0.5.0` | `v0-5-0-table-strategy-md060` | Completed for `v0.5.0`; `MD060` now has table-block parsing, official style checks, and safe fix subsets. |
| Done | Source-preserving document context for `v0.5.0` | `source-preserving-document-context` | Completed for `v0.5.0`; `DocumentContext` now shares source-preserving structure across migrated rule families. |
| Done | i18n product surface for `v0.6.0` | `v0-6-0-i18n-expansion-draft` | Completed for `v0.6.0`; Rust API, CLI rule/config output, MCP metadata, and translation coverage gates now share stable localized metadata. |
| Done | Performance hot path for `v0.7.0` | `v0-7-0-performance-hotpath` | Completed for `v0.7.0`; fix range application now uses a per-apply line offset index and the release is published. |
| Done | Linter precision and safe fix expansion for `v0.8.0` | `safe-fix-coverage-continuous-expansion` | Completed for `v0.8.0`; `MD035` and `MD055` safe fixes are fixture/golden locked and the release is published. |
| Done | Context-safe fix batch for `v0.8.1` | `v0-8-1-context-safe-fix-batch` | Completed for `v0.8.1`; `MD048` now uses block ranges, style config, and collision-guarded safe fixes. |
| Done | Unsafe fix mode for `v0.9.0` | `unsafe-fix-mode-and-confirmation` | Completed for `v0.9.0`; unsafe fix candidates now require explicit opt-in, interactive confirmation or `--yes`, and publish safety metadata to API/CLI/MCP/reporting surfaces. |
| Done | Formatter productization for `v0.10.0` | `formatter-productization` | Completed for `v0.10.0`; `kml fmt` now uses a dedicated layout formatter API for indentation and newline normalization with editor-friendly exit behavior. |
| Done | Tool distribution and editor expansion for `v0.11.0` | `tool-distribution-and-editor-expansion` | Completed for `v0.11.0`; root GitHub Action is now official and verified through action smoke checks. |
| Done | MCP workspace tools for `v0.12.0` | `mcp-workspace-tools-productization` | Completed for `v0.12.0`; `kml-mcp` now exposes workspace-safe file/directory tools with preview and explicit apply. |
| Done | Performance hot path work | `performance-hotpath-competition` | Completed; docs now include baseline, profile summary, before/after, and local regression guidance. |
| Done | Quality hardening and CI parity for `v0.12.2` | `v0-12-2-quality-hardening-and-ci-parity` | Released in `v0.12.2`; Windows CI, cache strategy review, and file-level false-positive regression coverage are now in place. |
| Done | Precision, performance, and test hardening for `v0.12.3` | `v0-12-3-precision-performance-test-hardening` | Completed for `v0.12.3`; `MD034` / `MD059` precision, code-line membership caching, static rule dispatch, and test evidence are archived. |
| Done | Core linter/check/fix/fmt hardening for `v0.12.4` | `v0-12-4-linter-check-fix-fmt-hardening` | Completed for `v0.12.4`; CLI command contracts, stdin validation, core command integration tests, and check/fix/fmt performance cases are archived. |
| Done | AST readiness and parser precision for `v0.12.5` | `v0-12-5-ast-readiness-and-parser-precision-hardening` | Completed for `v0.12.5`; link / inline-code / reference precision now uses shared parser evidence and measured parser-index cost. |
| Done | Context-sensitive rule migration for `v0.12.6` | `v0-12-6-context-sensitive-rule-migration` | Completed for `v0.12.6`; MD033 / MD037 / MD038 / MD039 / MD044 / MD049 / MD050 now use shared parser or `DocumentContext` evidence, and the final performance baseline is refreshed. |
| Done | Performance and convergence hardening for `v0.12.7` | `v0-12-7-performance-convergence-hardening` | Completed for `v0.12.7`; parser migration speed, safe fix behavior, check/fix/fmt idempotence, and stable-score dry run are archived. |
| Done | Stable candidate scoring and acceptance for `v0.12.8` | `v0-12-8-stable-candidate-acceptance` | Completed for `v0.12.8`; stable score 100 点、hard blocker 0 件、ユーザー受け入れ判断を記録した。 |
| Frozen | MCP Registry and distribution planning for `v0.13.0` | `mcp-registry-and-distribution-planning` | Waits until `v0.12.8` is accepted as stable; then defines package type, `server.json`, security review, and publish deferral before public Registry listing. |
| Frozen | MCP package and Registry publication for `v0.14.0` | `v0-14-0-mcp-package-and-registry-publication` | Implements the selected MCP package artifact and publishes Registry / Hub metadata after readiness gates pass. |
| Frozen | Remote MCP transport for `v0.15.0` | `v0-15-0-remote-mcp-transport` | Adds provider API reachable MCP transport only if local stdio support is not sufficient. |

Archived completed changes:

- `golden-edge-coverage-expansion` -> `openspec/changes/archive/2026-04-25-golden-edge-coverage-expansion`
- `safe-fix-strategy-expansion` -> `openspec/changes/archive/2026-04-25-safe-fix-strategy-expansion`
- `v0-5-0-table-strategy-md060` -> `openspec/changes/archive/2026-04-25-v0-5-0-table-strategy-md060`
- `performance-hotpath-competition` -> `openspec/changes/archive/2026-04-25-performance-hotpath-competition`
- `v0-6-0-i18n-expansion-draft` -> `openspec/changes/archive/2026-04-25-v0-6-0-i18n-expansion-draft`
- `v0-7-0-performance-hotpath` -> `openspec/changes/archive/2026-04-25-v0-7-0-performance-hotpath`
- `safe-fix-coverage-continuous-expansion` -> `openspec/changes/archive/2026-04-25-safe-fix-coverage-continuous-expansion`
- `v0-8-1-context-safe-fix-batch` -> `openspec/changes/archive/2026-04-25-v0-8-1-context-safe-fix-batch`
- `unsafe-fix-mode-and-confirmation` -> `openspec/changes/archive/2026-04-25-unsafe-fix-mode-and-confirmation`
- `formatter-productization` -> `openspec/changes/archive/2026-04-25-formatter-productization`
- `tool-distribution-and-editor-expansion` -> `openspec/changes/archive/2026-04-25-tool-distribution-and-editor-expansion`
- `mcp-workspace-tools-productization` -> `openspec/changes/archive/2026-04-26-mcp-workspace-tools-productization`
- `v0-12-2-quality-hardening-and-ci-parity` -> `openspec/changes/archive/2026-04-26-v0-12-2-quality-hardening-and-ci-parity`
- `v0-12-3-precision-performance-test-hardening` -> `openspec/changes/archive/2026-04-26-v0-12-3-precision-performance-test-hardening`
- `v0-12-4-linter-check-fix-fmt-hardening` -> `openspec/changes/archive/2026-04-26-v0-12-4-linter-check-fix-fmt-hardening`
- `v0-12-5-ast-readiness-and-parser-precision-hardening` -> `openspec/changes/archive/2026-04-26-v0-12-5-ast-readiness-and-parser-precision-hardening`
- `v0-12-6-context-sensitive-rule-migration` -> `openspec/changes/archive/2026-04-26-v0-12-6-context-sensitive-rule-migration`
- `v0-12-7-performance-convergence-hardening` -> `openspec/changes/archive/2026-04-26-v0-12-7-performance-convergence-hardening`
- `v0-12-8-stable-candidate-acceptance` -> `openspec/changes/archive/2026-04-26-v0-12-8-stable-candidate-acceptance`

## Suggested Order

1. Decide after `v0.12.8` release whether to continue hardening as `v0.12.9` or move to `v0.13.0` planning.
2. Apply `mcp-registry-and-distribution-planning` only if the next direction is `v0.13.0`; `v0.12.8` stable score is at least 90, hard blockers are 0, and user acceptance is recorded.
3. Apply `v0-14-0-mcp-package-and-registry-publication` only after the `v0.13.0` package and security gates are complete.
4. Apply `v0-15-0-remote-mcp-transport` only when API-hosted LLM usage is a concrete requirement; local stdio support is already covered by `v0.12.0`.

## Deferred Until v0.12.8 Stable Acceptance

- `distribution`: MCP Registry / Hub 公開、MCP package artifact、遠隔 MCP 接続（remote MCP transport）は、`v0.12.8` の stable score 90 点以上、hard blocker 0 件、ユーザー受け入れ完了まで凍結する。
- `design-debt`: Markdown token parser の共有化、nested bracket/link title を含む link parser 化、inline code span parser の rule 間共通化は、`v0.12.5` から `v0.12.6` の安定版準備として扱う。
- `ci-gap`: Windows では `cargo check`、`cargo fmt`、`cargo test` までを release 前 CI の責務にする。`make action-smoke` と `make mcp-stdio-smoke` の Windows 移植は、shell / path / `.exe` suffix の差分を切り分け、安定版 score の release reproducibility に影響する場合だけ `v0.12.x` に含める。

## Repository Guardrails

- New OpenSpec change files are ignored by the current `.gitignore`; commit them with explicit `git add -f openspec/changes/<change>` when they should be tracked.
- `check-diagnostic-i18n` was archived as `openspec/changes/archive/2026-04-25-check-diagnostic-i18n`.
- Keep one implementation change active at a time unless the write sets are disjoint and the roadmap is updated to show the parallelism.
- `formatter-productization` must not be mixed into `v0.8.0`; linter precision and safe fix coverage come first.
- `unsafe-fix-mode-and-confirmation` must not be mixed into `v0.7.0` or `v0.8.0`; it changes CLI safety semantics and belongs to `v0.9.0`.
- Registry publication must not be mixed into `mcp-registry-and-distribution-planning`; `v0.13.0` decides the gate and `v0.14.0` executes it.
- Remote MCP transport must not be used as a replacement for local stdio distribution; it has a different auth and workspace safety boundary.
