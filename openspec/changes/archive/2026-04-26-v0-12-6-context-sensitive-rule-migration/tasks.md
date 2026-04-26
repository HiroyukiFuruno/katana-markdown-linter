# Tasks

## Definition of Ready

- [x] `v0.12.5` の release と archive が完了している。
- [x] shared parser token または `DocumentContext` index の foundation が存在する。
- [x] `v0.12.x` は精度、速度、安定性だけを扱う方針である。
- [x] 進行中に task 外の高リスク不足が露見した場合だけ、作業を中断してユーザー判断を仰ぐ。

## 0. Rule Migration Inventory

- [x] context-sensitive rule を全件列挙する。
- [x] 各 rule を `parser-backed`、`document-context-backed`、`line-local-by-spec`、`future-work` に分類する。
- [x] 現在の誤検知、検出漏れ、fix risk、test gap を rule ごとに記録する。
- [x] `future-work` にした rule の理由と次の解消条件を記録する。

分類:

| Category | Rules | 記録 |
| --- | --- | --- |
| `parser-backed` | `MD033`、`MD034`、`MD037`、`MD038`、`MD039`、`MD044`、`MD049`、`MD050`、`MD051`、`MD052`、`MD054`、`MD059` | shared inline-code / link / reference token を根拠にする。v0.12.6 では `MD033`、`MD037`、`MD038`、`MD039`、`MD044`、`MD049`、`MD050` を追加移譲した。 |
| `document-context-backed` | `MD011`、`MD013`、`MD014`、`MD026`、`MD029`、`MD031`、`MD046`、`MD048`、`MD053`、`MD055`、`MD056`、`MD060` | code block、heading、table、reference definition の既存 `DocumentContext` index と mixed fixture で確認する。 |
| `line-local-by-spec` | `MD009`、`MD010`、`MD012`、`MD018`、`MD019`、`MD020`、`MD021`、`MD027`、`MD028`、`MD030`、`MD032`、`MD035`、`MD040`、`MD041`、`MD042`、`MD043`、`MD045`、`MD047`、`MD058` | 仕様上の主要判定が行単位または既存 block iteration で閉じる。追加 parser 移譲は現時点では精度改善につながりにくい。 |
| `future-work` | broader external AST conversion、HTML attribute parser の完全 shared 化、nested inline HTML semantics | v0.12.6 の安定化範囲では API / fix range への影響が大きい。`v0.12.8` の score で blocker 化した場合だけ `v0.12.x` に戻す。 |

リスク記録:

- `MD033`: inline HTML が長い code span / unclosed code span 内にある場合の誤検知を shared inline-code range で固定した。fix なし。
- `MD037`、`MD049`、`MD050`: emphasis / strong marker の inline code 内誤検知を shared inline-code range で固定した。fix なし。
- `MD038`: code span 内スペースの fix range を shared inline-code token から作るようにした。unclosed code span は fix 対象外。
- `MD039`: link text spacing の source range を shared link token から作るようにした。image は対象外。
- `MD044`: `code_blocks=false` の inline code 除外を shared inline-code range で固定した。`code_blocks=true` の既存挙動は維持した。
- `MD034`、`MD051`、`MD052`、`MD054`、`MD059`: v0.12.5 の link / reference migration を再確認し、document-level regression に残した。

## 1. Link And Reference Family

- [x] `v0.12.5` で移譲した link family の migration 状態を再確認する。
- [x] `MD053` の duplicate reference definition と shared reference index の整合を確認する。
- [x] `MD034`、`MD051`、`MD052`、`MD054`、`MD059` の document-level regression を拡張する。

記録:

- `tests/document_false_positive_regressions.rs` の mixed fixture で inline code、image、reference definition、autolink、HTML attribute URL の誤検知が増えていないことを確認する。
- `MD053` は duplicate reference definition を real violation case として維持し、shared reference index と矛盾しない。

## 2. Inline Content Family

- [x] emphasis / strong / code span 系 rule の inline code 除外を共有 token へ寄せる。
- [x] HTML block / inline HTML の除外判定を `DocumentContext` または parser token へ寄せる。
- [x] `MD033`、`MD037`、`MD038`、`MD039`、`MD044`、`MD049`、`MD050` の境界値 test を追加する。

記録:

- `MD033`: `src/rules/markdown/rules/md033.rs` に分離し、fenced code block と inline code span を `DocumentContext` で除外する。
- `MD037`: `spaces_in_emphasis.rs` で shared inline-code range 内の emphasis marker を除外する。
- `MD038`: `spaces_in_code.rs` で shared inline-code token を直接使い、long marker と unclosed marker を固定する。
- `MD039`: `md039.rs` で shared link token を使い、nested text と image skip を固定する。
- `MD044`: `md044.rs` で `code_blocks=false` の inline code 除外を shared inline-code range に移す。
- `MD049`、`MD050`: emphasis / strong style 判定前に shared inline-code range を確認する。

## 3. Block Structure Family

- [x] table rule が `DocumentContext` の table index と矛盾しないことを確認する。
- [x] list / heading / fence / command prompt rule が code block 内を通常本文として扱わないことを確認する。
- [x] `MD011`、`MD013`、`MD014`、`MD026`、`MD029`、`MD031`、`MD046`、`MD048`、`MD055`、`MD056`、`MD060` の mixed fixture を拡張する。

記録:

- table family は既存の `DocumentContext` table index と `tests/rule_fixture_harness.rs` / `tests/document_false_positive_regressions.rs` で維持する。
- list / heading / fence / command prompt family は code block 内の Markdown 風文字列を mixed fixture に含め、通常本文として診断しないことを確認する。
- この change では block family の新規 parser 化は行わない。既存 `DocumentContext` 依存の確認と回帰固定に限定する。

## 4. Fix Safety

- [x] migrated rule の fix range が original source range に基づくことを確認する。
- [x] overlapping / adjacent fix の collision test を追加または更新する。
- [x] default-safe fix が unsafe fix を混ぜていないことを確認する。

記録:

- `MD038` は inline-code token の `full_range` と marker length から fix range を構築する。
- `MD039` は shared link token の text range から fix range を構築する。
- `MD044` は original line range と `SourceRange` の対応で replacement を作る。
- unsafe fix は追加していない。default-safe fix の対象 rule だけを維持する。
- collision behavior は既存の fix application suite と `make release-check VERSION=v0.12.6` で確認する。

## 5. Performance

- [x] 実装前に `make perf-check` の結果を記録する。
- [x] context-sensitive rule migration 後の parser / context index cost を測定する。
- [x] repeated scan が減った箇所と増えた箇所を tasks に記録する。

実装前基準は v0.12.5 の refreshed baseline とする。

- `api_lint_large_document`: 10.247ms
- `api_lint_clean_large_document`: 6.334ms
- `api_fix_large_document`: 31.293ms
- `api_lint_link_heavy_document`: 13.502ms
- `api_lint_inline_code_heavy_document`: 13.664ms
- `context_inline_token_index_large_document`: 9.870ms
- `cli_check_many_small_files`: 10.965ms
- `cli_fix_many_small_files`: 93.014ms
- `cli_fmt_many_small_files`: 305.575ms

移譲直後の `make perf-check` snapshot:

- `api_lint_large_document`: 11.014ms / baseline 10.247ms / 1.07x
- `api_lint_clean_large_document`: 6.929ms / baseline 6.334ms / 1.09x
- `api_fix_large_document`: 35.115ms / baseline 31.293ms / 1.12x
- `context_inline_token_index_large_document`: 9.965ms / baseline 9.870ms / 1.01x

最終 baseline refresh snapshot:

- `api_lint_large_document`: 10.845ms
- `api_lint_clean_large_document`: 6.826ms
- `api_fix_large_document`: 34.378ms
- `api_format_large_document`: 6.500ms
- `api_lint_link_heavy_document`: 13.755ms
- `api_lint_inline_code_heavy_document`: 13.855ms
- `api_lint_reference_heavy_document`: 2.807ms
- `context_inline_token_index_large_document`: 10.244ms
- `cli_check_many_small_files`: 10.699ms
- `cli_fix_many_small_files`: 43.135ms
- `cli_fmt_many_small_files`: 47.523ms

最終 `make perf-check` comparison:

- `api_lint_large_document`: 11.038ms / baseline 10.845ms / 1.02x
- `api_lint_clean_large_document`: 6.845ms / baseline 6.826ms / 1.00x
- `api_fix_large_document`: 34.579ms / baseline 34.378ms / 1.01x
- `api_lint_link_heavy_document`: 14.148ms / baseline 13.755ms / 1.03x
- `api_lint_inline_code_heavy_document`: 13.743ms / baseline 13.855ms / 0.99x
- `context_inline_token_index_large_document`: 10.281ms / baseline 10.244ms / 1.00x
- `cli_check_many_small_files`: 11.014ms / baseline 10.699ms / 1.03x
- `cli_fix_many_small_files`: 43.438ms / baseline 43.135ms / 1.01x
- `cli_fmt_many_small_files`: 41.153ms / baseline 47.523ms / 0.87x

解釈:

- `MD033`、`MD037`、`MD044`、`MD049`、`MD050` は line に backtick がある場合だけ inline-code index を確認する。
- `MD038` は rule-local scan をやめて shared inline-code token を直接使う。
- `MD039` は bracket scan をやめて shared link token を使う。
- parser / context index cost は増えている。v0.12.7 はこの増分を convergence / performance cleanup の対象にする。

## 6. Release Preparation

- [x] crate version を `0.12.6` に更新する。
- [x] `CHANGELOG.md` に context-sensitive rule migration を英語で記載する。
- [x] OpenSpec delta を main specs に同期し、完了後に archive する。
- [x] release 前に `make release-check VERSION=v0.12.6` を通す。

## Verification

- [x] `make fmt-check`
- [x] `make lint`
- [x] `make ast-lint`
- [x] `cargo test --workspace --locked`
- [x] `cargo test --workspace --all-features --locked`
- [x] `cargo test --locked --test document_false_positive_regressions`
- [x] `cargo test --locked --test rule_fixture_harness`
- [x] `cargo test --locked --test upstream_golden_comparison`
- [x] `make dogfood`
- [x] `make perf-check`
- [x] `make release-check VERSION=v0.12.6`
- [x] `git diff --check`

## Definition of Done

- [x] context-sensitive rule の migration classification が完了している。
- [x] 優先 rule family が単一行文字列だけに依存しない。
- [x] migration 後の誤検知と検出漏れが fixture で固定されている。
- [x] fix safety と performance evidence が記録されている。
