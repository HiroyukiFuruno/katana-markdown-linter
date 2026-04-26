# Tasks

## Definition of Ready

- [x] `v0.12.2` の GitHub Release と crates.io publish が完了していること
- [x] `make release-verify VERSION=v0.12.2` が成功していること
- [x] `v0.12.2` 公開（release）後のブランチ整理（branch hygiene）が完了していること
- [x] `main` から `release/v0.12.3` 作業ブランチを開始していること
- [x] `v0.12.3` は新機能ではなく精度・速度・テスト改善パッチ（patch）として扱う方針が確認済みであること

## 0. Issue Inventory

- [x] 0.1 誤検知、検出漏れ、速度劣化、test-gap を `bug` / `test-gap` / `design-debt` / `perf-risk` に分類する
- [x] 0.2 `MD034`、`MD059`、インラインコード（inline code）、HTML、reference、command prompt 周辺の残課題を棚卸しする
- [x] 0.3 単体テスト（UT）と結合テスト（IT）の不足をテスト層（test layer）ごとに整理する
- [x] 0.4 `v0.12.3` に含める修正と後続 change に回す修正を切り分ける
- [x] 0.5 棚卸し結果を tasks に追記し、以後の実装順に反映する

### Issue Inventory Notes

- `bug`: `MD034` は1行の最初の裸URL（bare URL）しか診断しておらず、同じ行の2件目以降を検出漏れにしていた。
- `bug`: `MD034` は `See (https://example.com).` の閉じ括弧を URL range に含め、fix 後の文書構造を壊し得た。
- `bug`: `MD059` は1行に複数の禁止 link text があっても1件だけを行全体 range で診断していた。
- `bug`: `MD059` は `` `[link](...)` `` のような1文字 backtick だけを想定し、複数 backtick の inline code span を通常本文として扱っていた。
- `perf-risk`: 多くの rule が `is_code_line` を行ごとに呼ぶ一方、`DocumentContext` は code block range を毎回線形走査していた。
- `perf-risk`: lint 実行ごとに rule 実体を box 化して組み立て直していた。
- `test-gap`: 複数 bare URL、閉じ括弧付き bare URL、同一行の複数禁止 link text、複数 backtick inline code span、bare URL fix の idempotence が不足していた。
- `test-gap`: Windows path / shell / binary suffix は `tests/cross_tool_cli_benchmark.rs` と AST lint の CI guard で既存検証があるため、`v0.12.3` では production behavior を曲げる追加変更をしない。
- `design-debt`: Markdown token parser の共有化、nested bracket/link title まで含む完全な link parser 化、Windows での action smoke / MCP stdio smoke 移植は後続 change に残す。
- `v0.12.3` 対象: `MD034` / `MD059` の検出精度、`DocumentContext` の code-line membership cache、static rule dispatch、UT/IT/idempotence regression。
- 後続対象: parser 抽象化、より広い Markdown tokenization、Windows 固有 smoke target の拡張。

## 1. 精度強化（Precision Hardening）

- [x] 1.1 修正対象 rule の最小再現をルール単位テスト（rule-local test）に追加する
- [x] 1.2 文書単位の混在 fixture（document-level mixed fixture）に再発条件を追加する
- [x] 1.3 `DocumentContext` を使うべき判定とルール単位（rule-local）で十分な判定を分ける
- [x] 1.4 誤検知修正後に unrelated diagnostics が増えていないことを確認する
- [x] 1.5 修正しない設計負債は後続 change として roadmap に残す

### Precision Notes

- `MD034`: `bare_url_ranges` で同一行の複数 bare URL をすべて診断する。HTML attribute、inline link、reversed link、reference definition、inline code、fenced code block の除外は維持する。
- `MD034`: URL 末尾の文末 punctuation と、URL 内で対応する opening delimiter がない `)` / `]` を fix range から外す。
- `MD059`: 同一行の Markdown link text を個別に診断し、diagnostic range を link text 部分に限定する。
- `MD059`: 複数 backtick の inline code span を skip して、code span 内の `[link](...)` を診断しない。
- `DocumentContext`: 行が fenced code block 内かどうかは共有 context 側で持つ。inline code span の細部は今回の修正対象 rule 内で十分に閉じる。
- 確認済み targeted tests: `cargo test --locked rules::markdown::rules::md034::tests`、`cargo test --locked rules::markdown::rules::md059::tests`、`cargo test --locked --test document_false_positive_regressions`。

## 2. 速度強化（Performance Hardening）

- [x] 2.1 実装前に `make bench` または `make perf-check` で基準値（baseline）を記録する
- [x] 2.2 directory traversal、config validation、DocumentContext、rule dispatch、reporting、fix application の高負荷経路（hot path）を分類する
- [x] 2.3 測定できる高負荷経路（hot path）だけを最小変更で改善する
- [x] 2.4 正しさのテスト（correctness test）を通した上で before / after を tasks または docs に記録する
- [x] 2.5 CI の required gate を不安定な時計時間の閾値（wall-clock threshold）に依存させない

### Performance Notes

- Baseline before implementation: `make perf-check`
  - `api_lint_large_document`: 44.401ms
  - `api_lint_clean_large_document`: 28.468ms
  - `api_fix_large_document`: 113.858ms
  - `api_lint_many_small_documents`: 1.979ms
  - `cli_check_many_small_files`: 10.757ms
  - `context_build_large_document`: 0.077ms
  - `context_heading_index_large_document`: 1.067ms
  - `context_table_index_large_document`: 1.100ms
  - `config_validate_representative`: 0.039ms
  - `api_rule_catalog`: 0.013ms
- Hot path classification:
  - `DocumentContext`: `is_code_line` が code block range を線形走査しており、rule 数と行数で増幅していた。
  - `rule dispatch`: lint 実行ごとの boxed rule vector construction を避けられる。
  - `directory traversal` / `reporting` / `config validation` / `fix application`: 今回の主因ではない。既存 gate と benchmark で継続監視する。
- Implemented change:
  - `DocumentContext` に `code_line_flags` を追加し、`is_code_line` を定数時間化した。
  - static `RuleEntry` に `evaluate_context` function pointer を持たせ、`evaluate_all` は static registry から直接 dispatch する。
- After implementation: `make perf-check`
  - `api_lint_large_document`: 7.260ms
  - `api_lint_clean_large_document`: 4.383ms
  - `api_fix_large_document`: 20.374ms
  - `api_lint_many_small_documents`: 2.064ms
  - `cli_check_many_small_files`: 10.648ms
  - `context_build_large_document`: 0.079ms
  - `context_heading_index_large_document`: 1.064ms
  - `context_table_index_large_document`: 1.101ms
  - `config_validate_representative`: 0.042ms
  - `api_rule_catalog`: 0.013ms
- `make perf-check` は required benchmark case と schema を検証し、時計時間そのものは情報として扱う。baseline refresh はこの patch では行わない。

## 3. テスト強化（Test Hardening）

- [x] 3.1 単体テスト（UT）を rule、config、path、fix idempotence、Windows path 表現に拡充する
- [x] 3.2 結合テスト（IT）を CLI workspace、fixture corpus、dogfood、cross-tool benchmark、Windows CI 差分に拡充する
- [x] 3.3 test helper が OS 固有の実行形式を正しく扱うことを固定する
- [x] 3.4 test の都合だけで商用コードの責務を曲げていないことを確認する
- [x] 3.5 必要な AST lint または Makefile target を追加し、回帰を機械的に止める

### Test Notes

- Rule-local UT:
  - `MD034`: same-line multiple bare URL detection and closing parenthesis range exclusion。
  - `MD059`: same-line multiple prohibited link text diagnostics, long inline code span exclusion, and unclosed code span marker exclusion。
  - `DocumentContext`: cached code-line membership returns expected true / false / out-of-range values。
- API UT:
  - `fix_applies_multiple_bare_url_fixes_idempotently` で複数 `MD034` fix が1回で収束し、2回目の fix で再変更されないことを固定した。
- Document-level IT:
  - `tests/document_false_positive_regressions.rs` の mixed fixture に複数 backtick inline code span を追加し、real violation fixture に same-line multiple bare URL を追加した。
- Existing IT/gate coverage:
  - CLI workspace / path expansion は `src/cli.rs` tests、fixture corpus は `tests/rule_fixture_harness.rs` と `tests/upstream_golden_comparison.rs`、dogfood は `make dogfood`、cross-tool benchmark は `tests/cross_tool_cli_benchmark.rs` で検証する。
  - Windows CI 差分は `.github/workflows/test-and-build.yml` の matrix と `tests/ast_linter/workflow_portability_guard.rs` で守る。
  - test helper の OS 固有 executable suffix と shell 差分は `tests/cross_tool_cli_benchmark.rs` の `cfg(windows)` / `cfg(not(windows))` mock で固定済み。
- Production code は performance / precision の本来責務だけを変更し、test helper の都合による behavior change は行っていない。
- 新しい AST lint / Makefile target は不要。既存 `make ast-lint`、`make dogfood`、`make perf-check`、`make release-check` の責務で足りる。

## 4. 公開準備（Release Preparation）

- [x] 4.1 crate version を `0.12.3` に更新する
- [x] 4.2 `CHANGELOG.md` に `v0.12.3` を追加する
- [x] 4.3 public docs を変更した場合は英語のみで記述し、`make ast-lint` を通す
- [x] 4.4 OpenSpec の task と spec delta を同期し、完了後に archive する
- [x] 4.5 release 前に `make release-check VERSION=v0.12.3` を通す

## Verification

- [x] `make fmt-check`
- [x] `make lint`
- [x] `make ast-lint`
- [x] `cargo test --workspace --locked`
- [x] `cargo test --all-features --locked`
- [x] `make dogfood`
- [x] `make perf-check`
- [x] GitHub Actions の Ubuntu / macOS / Windows CI が成功していること
- [x] `make release-check VERSION=v0.12.3`
- [x] `git diff --check`

## Definition of Done

- [x] 精度改善がルール単位テスト（rule-local test）と文書単位fixture（document-level fixture）の両方で固定されていること
- [x] 速度改善が基準値（baseline）と before / after で説明できること
- [x] 単体テスト（UT）と結合テスト（IT）の追加範囲が tasks に残っていること
- [x] `v0.12.3` に含めない課題が roadmap または後続 change に残っていること
- [x] 公開前ゲート（release gate）と CI が通っていること
