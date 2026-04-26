# Tasks

## Definition of Ready

- [x] `v0.12.4` の release と archive が完了している。
- [x] `main` が release 後の基準になっている。
- [x] `v0.12.x` は安定版到達まで精度、速度、安定性だけを扱う方針である。
- [x] `v0.13.0` の配布展開は `v0.12.8` の安定版受け入れ後まで凍結する。
- [x] 進行中に task 外の高リスク不足が露見した場合だけ、作業を中断してユーザー判断を仰ぐ。

## 0. Stability Roadmap Alignment

- [x] `active-roadmap.md` に `v0.12.5` から `v0.12.8` までの安定版ロードマップを反映する。
- [x] `v0.13.0` の Definition of Ready に `v0.12.8` の安定版受け入れ完了を追加する。
- [x] 精度、速度、安定性に関係しない task が `v0.12.5` に混ざっていないことを確認する。

## 1. Parser Readiness Inventory

- [x] 構文依存 rule を `link`、`inline-code`、`reference`、`HTML`、`code-block`、`table`、`list`、`heading` に分類する。
- [x] rule ごとの手書き parser と shared `DocumentContext` 利用状況を棚卸しする。
- [x] AST / parser へ移譲する rule と、現時点では移譲しない rule を理由付きで記録する。
- [x] 外部 parser dependency が必要かどうかを実装前に仮説として整理する。

## 2. Source-Preserving Parser Foundation

- [x] inline code span token を source range 付きで抽出する。
- [x] inline link / image / autolink token を source range 付きで抽出する。
- [x] collapsed / full reference link と reference definition を source range 付きで抽出する。
- [x] token index を `DocumentContext` の lazy index として参照できるようにする。
- [x] empty input、CRLF、Unicode、unclosed marker、nested bracket、link title の境界値を unit test で固定する。

## 3. Link Rule Precision Migration

- [x] `MD034` の bare URL 除外判定を shared token に寄せる。
- [x] `MD051` の fragment link 抽出を shared link token に寄せる。
- [x] `MD052` の reference 判定を shared reference token に寄せる。
- [x] `MD054` の link style 判定を shared link/reference token に寄せる。
- [x] `MD059` の link text 判定を shared link token に寄せる。
- [x] `MD053` が reference definition index と矛盾しないことを確認する。

## 4. Precision Fixtures

- [x] rule-local test に nested bracket、link title、inline code、image、reference definition の境界値を追加する。
- [x] document-level mixed fixture に link-heavy / inline-code-heavy の再発条件を追加する。
- [x] 誤検知と検出漏れの両方を固定する。
- [x] unrelated diagnostics が増えていないことを確認する。

## 5. Performance Evidence

- [x] 実装前に `make perf-check` の結果を記録する。
- [x] link-heavy、inline-code-heavy、reference-heavy、parser index construction の benchmark case を追加する。
- [x] 実装後に `make perf-check` を実行し、性能変化と理由を tasks に記録する。
- [x] 性能改善のために command semantics を弱めていないことを確認する。

実装前 `make perf-check` median snapshot:

- `api_lint_large_document`: 8.272ms
- `api_lint_clean_large_document`: 5.742ms
- `api_fix_large_document`: 21.003ms
- `api_format_large_document`: 6.648ms
- `api_lint_many_small_documents`: 2.292ms
- `context_build_large_document`: 0.081ms
- `context_heading_index_large_document`: 1.115ms
- `context_table_index_large_document`: 1.122ms
- `cli_check_many_small_files`: 12.054ms
- `cli_fix_many_small_files`: 42.976ms
- `cli_fmt_many_small_files`: 46.495ms
- `config_validate_representative`: 0.040ms
- `api_rule_catalog`: 0.013ms

実装後 baseline refresh median snapshot:

- `api_lint_large_document`: 10.247ms
- `api_lint_clean_large_document`: 6.334ms
- `api_fix_large_document`: 31.293ms
- `api_format_large_document`: 6.554ms
- `api_lint_many_small_documents`: 2.073ms
- `api_lint_link_heavy_document`: 13.502ms
- `api_lint_inline_code_heavy_document`: 13.664ms
- `api_lint_reference_heavy_document`: 2.915ms
- `context_build_large_document`: 0.080ms
- `context_heading_index_large_document`: 1.054ms
- `context_table_index_large_document`: 1.080ms
- `context_inline_token_index_large_document`: 9.870ms
- `cli_check_many_small_files`: 10.965ms
- `cli_fix_many_small_files`: 93.014ms
- `cli_fmt_many_small_files`: 305.575ms
- `config_validate_representative`: 0.040ms
- `api_rule_catalog`: 0.014ms

速度差分の解釈:

- link / inline-code parser index を追加したため lint / fix の source parsing cost は増えた。
- CLI `fix` / `fmt` は temp workspace mutation と filesystem variance の影響が大きく、release gate では informational として扱う。
- command semantics は弱めていない。link / inline-code / reference の除外精度を上げる方向の変更に限定している。

最終 `make perf-check` comparison:

- `api_lint_large_document`: 10.100ms / baseline 10.247ms / 0.99x
- `api_lint_clean_large_document`: 6.294ms / baseline 6.334ms / 0.99x
- `api_fix_large_document`: 31.332ms / baseline 31.293ms / 1.00x
- `api_format_large_document`: 6.472ms / baseline 6.554ms / 0.99x
- `api_lint_many_small_documents`: 2.040ms / baseline 2.073ms / 0.98x
- `api_lint_link_heavy_document`: 13.402ms / baseline 13.502ms / 0.99x
- `api_lint_inline_code_heavy_document`: 13.619ms / baseline 13.664ms / 1.00x
- `api_lint_reference_heavy_document`: 2.911ms / baseline 2.915ms / 1.00x
- `context_inline_token_index_large_document`: 9.840ms / baseline 9.870ms / 1.00x
- `cli_check_many_small_files`: 10.507ms / baseline 10.965ms / 0.96x
- `cli_fix_many_small_files`: 43.092ms / baseline 93.014ms / 0.46x
- `cli_fmt_many_small_files`: 49.389ms / baseline 305.575ms / 0.16x

## 6. Release Preparation

- [x] crate version を `0.12.5` に更新する。
- [x] `CHANGELOG.md` に parser readiness、link precision、performance evidence を英語で記載する。
- [x] OpenSpec delta を main specs に同期し、完了後に archive する。
- [x] release 前に `make release-check VERSION=v0.12.5` を通す。

## Verification

- [x] `make fmt-check`
- [x] `make lint`
- [x] `make ast-lint`
- [x] `cargo test --workspace --locked`
- [x] `cargo test --workspace --all-features --locked`
- [x] `cargo test --locked --test document_false_positive_regressions`
- [x] `make dogfood`
- [x] `make perf-check`
- [x] `make release-check VERSION=v0.12.5`
- [x] `git diff --check`

## Definition of Done

- [x] link / inline-code / reference の shared parser foundation が存在する。
- [x] 優先 link rule が rule-local parser だけに依存しない。
- [x] 精度改善が UT と IT の両方で固定されている。
- [x] parser index の性能影響が測定されている。
- [x] `v0.13.0` の配布展開はまだ進んでいない。
