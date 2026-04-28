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
- `v0.12.9`: 公開導線を広げる前の patch として、KatanA docs/assets Markdown と既存 performance corpus を使い、外部品質への confidence evidence を固める。
- `v0.12.14`: precision-first を維持したまま `DocumentContext` の inline 抽出統合 (`InlineIndex`)、`MD051` の `ignored_pattern` regex キャッシュ、`MD046` の `code_line_flags` 索引活用で hot path コストを削減する。
- `v0.12.15+`: 精度 fix+ 継続拡張。MD052 collapsed reference safe-fix、MD046 style 統一 fix、MD043 missing heading 挿入 fix、MD056 table column count 補完 fix、MD034 scheme 拡張、MD051 fragment Unicode/emoji 厳密化を逐次取り込む。
- `v0.13.0`: `v0.12.8` の stable 条件と `v0.12.9` の public confidence gate を満たしてから、MCP Registry / Hub 公開前の配布方式、`server.json`、security gate を決める。公開自体はまだ行わない。
- `v0.14.0`: `v0.13.0` で選んだ package artifact と Registry metadata を実装し、公開まで進める。
- `v0.15.0`: API-hosted LLM から直接使う必要が出た場合だけ、遠隔 MCP 接続（remote MCP transport）を設計・実装する。
- `v0.16.0`: Introduce JSON schema and LSP entrypoint to enable editor auto-completion and real-time diagnostics.
- `v0.17.0`: Expand distribution channels including Homebrew, standalone binary artifacts, and language wrappers (npm/pip).

| Priority | Work Area | Change | Why Now |
| --- | --- | --- | --- |
| Done | Golden and edge coverage for `v0.4.0` | `golden-edge-coverage-expansion` | Completed for `v0.4.0`; dashboard now derives golden status from the locked baseline and records edge coverage. |
| Done | Safe check/fix expansion for `v0.4.0` | `safe-fix-strategy-expansion` | Completed for `v0.4.0`; `MD005` and `MD030` safe subsets are locked, while `MD060` remains diagnostic/manual-required because official metadata marks it non-fixable. |
| Done | Table strategy for `v0.5.0` | `v0-5-0-table-strategy-md060` | Completed for `v0.5.0`; `MD060` now has table-block parsing, official style checks, and safe fix subsets. |
| Done | Source-preserving document context for `v0.5.0` | `archived/2026-04-27-source-preserving-document-context` | Completed for `v0.5.0`; `DocumentContext` now shares source-preserving structure across migrated rule families. |
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
| Done | Public confidence hardening for `v0.12.9` | `v0-12-9-public-confidence-hardening` | Completed as public confidence and required evidence gate before distribution expansion. |
| Done | Precision-zero-regression hot path consolidation for `v0.12.14` | `v0-12-14-precision-and-performance-hardening` | Eliminates O(L×b) line_in_blocks scan, replaces inside_code_span with partition_point binary search, and removes String allocation in the backtick scanner. context_inline_token_index improved ≈12×; inline-code and link benchmarks 3–7×. |
| Done | Dead-code removal and MD052 safe-fix for `v0.12.15` | `v0-12-15-precision-and-performance-hardening` | Removes unused `_line_index` parameter from `inside_code_span`, replaces defensive `.get(idx).copied().unwrap_or(false)` with direct indexing in 4 extractors, and adds safe-fix to MD052 (`[ref][]` → `[ref]`). |
| Done | MD046 safe-fix and MD034 scheme expansion for `v0.12.16` | `v0-12-16-precision-and-performance-hardening` | Adds per-block safe-fix to MD046 (indented→fenced), extends MD034 to ftp/ftps/mailto schemes, and replaces O(n) is_ignored_url scans with partition_point binary search. |
| Done | FixDetail API and MD051 Unicode/emoji precision for `v0.12.17` | `v0-12-17-fix-detail-api-and-md051-unicode-precision` | Adds `FixDetail { rule_id, range, applied }` to `FixResult` (Issue #43). Locks in MD051 emoji/CJK fragment behavior via comprehensive unit and integration tests. |
| P2 | Precision fix+ continuous expansion for `v0.12.18+` | (TBD) | Adds `MD043` (missing heading insertion) and `MD056` (table column count) fixes. Sequenced 1-2 rules per patch to preserve precision-first cadence. |
| P1 | MCP Registry and distribution planning for `v0.13.0` | `v0-13-0-mcp-registry-and-distribution-planning` | Proceed after release and user approval; defines package type, `server.json`, security review, and publish deferral before public Registry listing. |
| Frozen | MCP package and Registry publication for `v0.14.0` | `v0-14-0-mcp-package-and-registry-publication` | Implements the selected MCP package artifact and publishes Registry / Hub metadata after readiness gates pass. |
| Frozen | Remote MCP transport for `v0.15.0` | `v0-15-0-remote-mcp-transport` | Adds provider API reachable MCP transport only if local stdio support is not sufficient. |
| Frozen | Config schema and editor integration for `v0.16.0` | `v0-16-0-config-schema-and-editor-integration` | Enables editor auto-completion and standardized LSP diagnostics. |
| Frozen | Binary distribution expansion for `v0.17.0` | `v0-17-0-binary-distribution-expansion` | Expands beyond Cargo/GitHub Actions to Homebrew and standalone artifacts. |

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
- `v0-12-9-public-confidence-hardening` -> `openspec/changes/archive/2026-04-27-v0-12-9-public-confidence-hardening`
- `v0-12-10-quality-and-performance-hardening` -> `openspec/changes/archive/2026-04-27-v0-12-10-quality-and-performance-hardening`
- `source-preserving-document-context` -> `openspec/changes/archive/2026-04-27-source-preserving-document-context`
- `v0-12-14-precision-and-performance-hardening` -> `openspec/changes/archive/2026-04-28-v0-12-14-precision-and-performance-hardening`
- `v0-12-15-precision-and-performance-hardening` -> `openspec/changes/archive/2026-04-28-v0-12-15-precision-and-performance-hardening`
- `v0-12-16-precision-and-performance-hardening` -> `openspec/changes/archive/2026-04-28-v0-12-16-precision-and-performance-hardening`

## Suggested Order

1. Apply `v0-12-9-public-confidence-hardening`; it proves external corpus confidence before distribution expansion.
2. Apply `v0-13-0-mcp-registry-and-distribution-planning` only after `v0.12.9` public confidence score is at least 90 and release-blocking issues are 0.
3. Apply `v0-14-0-mcp-package-and-registry-publication` only after the `v0.13.0` package and security gates are complete.
4. Apply `v0-15-0-remote-mcp-transport` only when API-hosted LLM usage is a concrete requirement; local stdio support is already covered by `v0.12.0`.
5. Apply `v0-16-0-config-schema-and-editor-integration` after remote transport is stable.
6. Apply `v0-17-0-binary-distribution-expansion` after the core and editor tools are well-established.

## Deferred Until v0.12.8 Stable Acceptance

- `distribution`: MCP Registry / Hub 公開、MCP package artifact、遠隔 MCP 接続（remote MCP transport）は、`v0.12.8` の stable score 90 点以上、hard blocker 0 件、ユーザー受け入れ完了まで凍結する。
- `design-debt`: Markdown token parser の共有化、nested bracket/link title を含む link parser 化、inline code span parser の rule 間共通化は、`v0.12.5` から `v0.12.6` の安定版準備として扱う。
- `ci-gap`: Windows では `cargo check`、`cargo fmt`、`cargo test` までを release 前 CI の責務にする。`make action-smoke` と `make mcp-stdio-smoke` の Windows 移植は、shell / path / `.exe` suffix の差分を切り分け、安定版 score の release reproducibility に影響する場合だけ `v0.12.x` に含める。
- `unsafe-fix`: `MD013` の line-length wrap fix は list / table / code 文脈ガードが必須で、デフォルトの safe-fix contract に組み込めない。`v0.13.x` 以降に独立 change として切り出し、`unsafe-fix` mode の opt-in 設定下でのみ有効化する。

## Repository Guardrails

- New OpenSpec change files are ignored by the current `.gitignore`; commit them with explicit `git add -f openspec/changes/<change>` when they should be tracked.
- `check-diagnostic-i18n` was archived as `openspec/changes/archive/2026-04-25-check-diagnostic-i18n`.
- Keep one implementation change active at a time unless the write sets are disjoint and the roadmap is updated to show the parallelism.
- `formatter-productization` must not be mixed into `v0.8.0`; linter precision and safe fix coverage come first.
- `unsafe-fix-mode-and-confirmation` must not be mixed into `v0.7.0` or `v0.8.0`; it changes CLI safety semantics and belongs to `v0.9.0`.
- Registry publication must not be mixed into `v0-13-0-mcp-registry-and-distribution-planning`; `v0.13.0` decides the gate and `v0.14.0` executes it.
- Remote MCP transport must not be used as a replacement for local stdio distribution; it has a different auth and workspace safety boundary.
