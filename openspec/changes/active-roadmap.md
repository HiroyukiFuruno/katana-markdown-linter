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
- `v0.12.19`: `MD003` の safe-fix を追加し、`MD028` は作者判断が必要なため `v0.12.21` の by-design 宣言へ送る。
- `v0.12.20`: released patch として `v0.12.19` 後の performance 計測、`api_rule_catalog` の hot path 改善、baseline refresh を完了した。新規 rule/fix は入れていない。
- `v0.12.21`: KatanA 側ドキュメント feedback sweep、`MD007` bad-fix 修正、reserved / ignored directory の default exclude、残り rule の by-design 宣言を完了し、0.12.x closeout 条件を満たした。
- `v0.13.0`: `v0.12.21` release 後、MCP Registry / Hub 公開前の配布方式、`server.json`、security gate を決めた。公開自体はまだ行わない。
- `v0.14.0`: `v0.13.0` で選んだ MCPB package artifact と Registry metadata を実装し、公開導線を release workflow に組み込んだ。
- `v0.15.0`: API-hosted LLM から直接使う必要が出た場合だけ、遠隔 MCP 接続（remote MCP transport）を設計・実装する。
- `v0.16.0`: Introduce JSON schema and LSP entrypoint to enable editor auto-completion and real-time diagnostics.
- `v0.16.1`: KatanA 本体と同じ locale set に合わせ、description 系 API と rule document Markdown の i18n 欠落を閉じる。
- `v0.16.2`: `v0.17.0` の配布拡張を凍結したまま、document answer fix evaluation で `kml check --fix` の byte-for-byte 正しさを固める。
- `v0.17.0`: released distribution expansion with standalone binary artifacts, Homebrew formula generation, and deferred npm/pip wrapper publication gates.
- `v0.17.1`: post-release distribution closeout として npm / PyPI wrapper を公式 install channel に昇格し、Homebrew tap 更新、npm trusted publishing 後始末、`release-verify` の wrapper / tap 検証拡張を扱う。
- `v0.17.2`: npm package page polish と npm publish closeout を行う。`wrappers/npm/README.md`、keywords / homepage / bugs metadata、npm tarball verification、trusted publishing retry を `v0.18.0` より前に閉じる。
- `v0.17.3`: `v0.17.2` が GitHub Release / crates.io だけ先行公開されたため、PyPI page polish と release workflow の partial publish 防止を加え、npm / PyPI publish closeout を整合版で完了する。
- `v0.18.0`: config schema publication を product surface として固める。versioned schema URL、schema regression tests、editor validation docs を release gate に含める。
- `v0.18.1`: VS Code extension MVP を進める。`kml lsp` と config schema を共有エンジンにし、VS Code 側は薄い起動ラッパー（thin wrapper）として diagnostics / format / safe quick-fix を公開する。
- `v0.18.2`: Zed extension MVP を進める。VS Code extension の実装判断を再利用しつつ、Zed の language server extension 境界で `kml lsp` を起動できることを小さく検証する。
- `v0.18.3`: editor extension hardening を進める。VS Code / Zed の install docs、smoke tests、release verification、将来の Neovim docs-only sample をまとめて整える。

| Priority | Work Area | Change | Why Now |
| --- | --- | --- | --- |
| P0 | Distribution closeout for `v0.17.1` | `v0-17-1-distribution-closeout` | `v0.17.0` already published GitHub Release, crates.io, and the initial npm wrapper source. `v0.17.1` closed GitHub Release / crates.io / PyPI / Homebrew work, while npm publication is handed to `v0.17.3` after package README, PyPI README, metadata polish, and workflow hardening. |
| P1 | Schema publication for `v0.18.0` | `v0-18-0-schema-publication` | `kml config schema` already exists, but distribution docs still defer schema publication. The next durable step is a versioned schema contract, fixture-backed schema compatibility checks, and docs that editor integrations can rely on. |
| P1 | VS Code extension MVP for `v0.18.1` | `v0-18-1-vscode-extension-mvp` | The LSP entrypoint and schema command already exist. VS Code should become the first real editor target because the extension surface can stay thin: launch `kml lsp`, associate the config schema, and expose diagnostics / format / safe quick-fix without moving lint logic into the extension. |
| P2 | Zed extension MVP for `v0.18.2` | `v0-18-2-zed-extension-mvp` | Zed is the second target. Keep it behind VS Code so the shared `kml lsp` contract is already stable, then validate only the Zed-specific extension boundary and installation flow. |
| P2 | Editor extension hardening for `v0.18.3` | `v0-18-3-editor-extension-hardening` | After both target editors have MVPs, harden install docs, smoke tests, release verification, marketplace packaging, and optional docs-only Neovim configuration without expanding the core engine. |
| P2 | Release verification hardening | TBD: `release-verification-hardening` | `v0.17.0` exposed that external registry verification is broader than GitHub Release + crates.io. Add npm, PyPI, wrapper launch, and tap formula checks to the post-release verification path before the next distribution release. |
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
| Done | Precision fix+ continuous expansion for `v0.12.18` | `v0-12-18-md056-table-column-padding-safe-fix` | Added `MD029` regression fix for nested unordered-list interruption and `MD056` table-column safe-fix. |
| Done | MD003 safe-fix and MD028 fix policy for `v0.12.19` | `v0-12-19-md003-md028-fix-policy` | Added fixture-backed `MD003` setext-to-ATX safe-fix and kept `MD028` diagnostic-only as a manual-intent by-design candidate for `v0.12.21`. |
| Done | Performance measurement and hot path hardening for `v0.12.20` | `v0-12-20-performance-measurement-and-hotpath-hardening` | Completed post-`v0.12.19` measurement, fixed the `api_rule_catalog` metadata clone hot path, refreshed the baseline, and kept rule expansion frozen. |
| Done | KatanA feedback and 0.12.x closeout for `v0.12.21` | `archive/2026-04-29-v0-12-21-katana-feedback-and-012x-closeout` | Completed 524-file KatanA check/fix review, fixed the `MD007` bad-fix pattern, recorded release-blocking issues at 0, and closed remaining diagnostic-only rules with by-design reasons. |
| Done | MCP Registry and distribution planning for `v0.13.0` | `archive/2026-04-29-v0-13-0-mcp-registry-and-distribution-planning` | Defines package type, `server.json`, security review, and publish deferral before public Registry listing. |
| Done | MCP package and Registry publication for `v0.14.0` | `archive/2026-04-29-v0-14-0-mcp-package-and-registry-publication` | Adds the MCPB release artifact, Registry metadata rendering, release publication step, and release gates. |
| Done | Remote MCP transport for `v0.15.0` | `archive/2026-04-29-v0-15-0-remote-mcp-transport` | Adds self-hosted text-only Streamable HTTP transport while keeping workspace-backed remote access deferred. |
| Done | Config schema and editor integration for `v0.16.0` | `archive/2026-04-29-v0-16-0-config-schema-and-editor-integration` | Enables editor auto-completion and standardized LSP diagnostics. |
| Done | Full locale i18n for `v0.16.1` | `archive/2026-04-29-v0-16-1-full-locale-i18n` | Matches KatanA supported locales and localizes both rule descriptions and rule Markdown documentation. |
| Done | Document answer fix regressions for `v0.16.2` | `archive/2026-04-29-v0-16-2-document-answer-fix-regressions` | Freezes `v0.17.0` distribution work and verifies 250 document-level `check --fix` answer fixtures before the next distribution expansion. |
| Done | Binary distribution expansion for `v0.17.0` | `archive/2026-04-30-v0-17-0-binary-distribution-expansion` | Adds release binary archives, Homebrew formula generation, wrapper smoke coverage, and explicit wrapper publish deferral. |
| Done | npm package polish for `v0.17.2` | `archive/2026-04-30-v0-17-2-npm-package-polish` | Adds npm README / metadata / tarball verification and prepares trusted publishing retry before schema/editor work resumes. |
| Done | Release flow recovery for `v0.17.3` | `archive/2026-04-30-v0-17-3-release-flow-recovery` | Adds PyPI page README / metadata verification and prevents tag-push partial release before npm / PyPI wrapper jobs run. |

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
- `v0-12-17-fix-detail-api-and-md051-unicode-precision` -> `openspec/changes/archive/2026-04-28-v0-12-17-fix-detail-api-and-md051-unicode-precision`
- `v0-12-18-md056-table-column-padding-safe-fix` -> `openspec/changes/archive/2026-04-28-v0-12-18-md056-table-column-padding-safe-fix`
- `v0-12-19-md003-md028-fix-policy` -> `openspec/changes/archive/2026-04-29-v0-12-19-md003-md028-fix-policy`
- `v0-12-20-performance-measurement-and-hotpath-hardening` -> `openspec/changes/archive/2026-04-29-v0-12-20-performance-measurement-and-hotpath-hardening`
- `v0-12-21-katana-feedback-and-012x-closeout` -> `openspec/changes/archive/2026-04-29-v0-12-21-katana-feedback-and-012x-closeout`
- `v0-13-0-mcp-registry-and-distribution-planning` -> `openspec/changes/archive/2026-04-29-v0-13-0-mcp-registry-and-distribution-planning`
- `v0-14-0-mcp-package-and-registry-publication` -> `openspec/changes/archive/2026-04-29-v0-14-0-mcp-package-and-registry-publication`
- `v0-15-0-remote-mcp-transport` -> `openspec/changes/archive/2026-04-29-v0-15-0-remote-mcp-transport`
- `v0-16-0-config-schema-and-editor-integration` -> `openspec/changes/archive/2026-04-29-v0-16-0-config-schema-and-editor-integration`
- `v0-16-1-full-locale-i18n` -> `openspec/changes/archive/2026-04-29-v0-16-1-full-locale-i18n`
- `v0-16-2-document-answer-fix-regressions` -> `openspec/changes/archive/2026-04-29-v0-16-2-document-answer-fix-regressions`
- `v0-17-0-binary-distribution-expansion` -> `openspec/changes/archive/2026-04-30-v0-17-0-binary-distribution-expansion`
- `v0-17-2-npm-package-polish` -> `openspec/changes/archive/2026-04-30-v0-17-2-npm-package-polish`

## Suggested Order

1. `v0-17-1-distribution-closeout`: keep the release ledger as the handoff record for the partial `v0.17.1` closeout.
2. `v0-18-0-schema-publication`: make the existing config schema command a stable published contract before more editor polish depends on it.
3. `v0-18-1-vscode-extension-mvp`: build the first thin editor wrapper on the stable schema and existing LSP entrypoint.
4. `v0-18-2-zed-extension-mvp`: reuse the shared LSP contract and validate the Zed-specific extension boundary.
5. `v0-18-3-editor-extension-hardening`: harden docs, smoke tests, release checks, and packaging after both target editor MVPs exist.
6. `release-verification-hardening`: can run alongside the above if its write set stays limited to release scripts, `Makefile`, and release docs.

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
