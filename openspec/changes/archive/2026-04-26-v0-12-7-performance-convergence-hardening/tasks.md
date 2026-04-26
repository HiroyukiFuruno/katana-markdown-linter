# Tasks

## Definition of Ready

- [x] `v0.12.6` の release と archive が完了している。
- [x] context-sensitive rule migration の分類と evidence が存在する。
- [x] link-heavy / inline-code-heavy / reference-heavy benchmark case が存在する。
- [x] `v0.12.x` は精度、速度、安定性だけを扱う方針である。
- [x] 進行中に task 外の高リスク不足が露見した場合だけ、作業を中断してユーザー判断を仰ぐ。

## 0. Evidence Inventory

- [x] `v0.12.5` と `v0.12.6` の parser / migration evidence を読み直す。
- [x] 未説明の performance regression、fix oscillation、fmt idempotence gap を分類する。
- [x] `v0.12.8` の score に必要な evidence の不足を tasks に残す。

記録:

- `v0.12.5`: link / inline-code / reference shared parser foundation と parser-index benchmark を追加済み。
- `v0.12.6`: `MD033`、`MD037`、`MD038`、`MD039`、`MD044`、`MD049`、`MD050` を shared parser / `DocumentContext` evidence へ移譲済み。
- 未説明の fix oscillation は確認されていない。`tests/cli_convergence_contract.rs` で parser-heavy workflow を固定した。
- `fmt` idempotence gap は確認されていない。repeated `fmt` は changed=false を返す。
- `v0.12.8` に残す不足は、最終ユーザー受け入れ判断と required CI の release 後確認だけ。

## 1. Performance Hardening

- [x] 実装前に `make perf-check` を実行し、baseline を記録する。
- [x] parser index construction の repeated cost を確認する。
- [x] API lint / API fix / CLI check / CLI fix / CLI fmt の hot path を分類する。
- [x] 必要な最適化だけを行い、rule semantics を変えない。
- [x] baseline refresh が必要な場合は、正しさの gate 後に理由付きで行う。

実装前基準は `v0.12.6` の refreshed baseline とする。

- `api_lint_large_document`: 10.845ms
- `api_lint_clean_large_document`: 6.826ms
- `api_fix_large_document`: 34.378ms
- `api_lint_link_heavy_document`: 13.755ms
- `api_lint_inline_code_heavy_document`: 13.855ms
- `api_lint_reference_heavy_document`: 2.807ms
- `context_inline_token_index_large_document`: 10.244ms
- `cli_check_many_small_files`: 10.699ms
- `cli_fix_many_small_files`: 43.135ms
- `cli_fmt_many_small_files`: 47.523ms

実装:

- `DocumentContext::is_inside_inline_code` を全 span の線形走査から、開始位置に基づく `partition_point` の近傍確認へ変更した。
- `examples/perf_benchmark.rs` に `api_lint_table_heavy_document`、`api_fix_parser_heavy_document`、`api_format_parser_heavy_document` を追加した。
- `scripts/ci/perf-check.py` の required case に追加 case を登録した。

最終 baseline refresh snapshot:

- `api_lint_large_document`: 10.766ms
- `api_lint_clean_large_document`: 6.763ms
- `api_fix_large_document`: 34.300ms
- `api_format_large_document`: 6.431ms
- `api_lint_link_heavy_document`: 13.467ms
- `api_lint_inline_code_heavy_document`: 13.525ms
- `api_lint_reference_heavy_document`: 2.814ms
- `api_lint_table_heavy_document`: 4.698ms
- `api_fix_parser_heavy_document`: 25.060ms
- `api_format_parser_heavy_document`: 0.846ms
- `context_inline_token_index_large_document`: 9.879ms
- `cli_check_many_small_files`: 11.140ms
- `cli_fix_many_small_files`: 46.206ms
- `cli_fmt_many_small_files`: 48.874ms

最終 `make perf-check` comparison:

- `api_lint_large_document`: 11.827ms / baseline 10.766ms / 1.10x
- `api_lint_clean_large_document`: 6.823ms / baseline 6.763ms / 1.01x
- `api_fix_large_document`: 34.244ms / baseline 34.300ms / 1.00x
- `api_lint_link_heavy_document`: 13.234ms / baseline 13.467ms / 0.98x
- `api_lint_inline_code_heavy_document`: 13.604ms / baseline 13.525ms / 1.01x
- `api_lint_reference_heavy_document`: 2.784ms / baseline 2.814ms / 0.99x
- `api_lint_table_heavy_document`: 4.721ms / baseline 4.698ms / 1.00x
- `api_fix_parser_heavy_document`: 25.154ms / baseline 25.060ms / 1.00x
- `api_format_parser_heavy_document`: 0.850ms / baseline 0.846ms / 1.00x
- `context_inline_token_index_large_document`: 9.838ms / baseline 9.879ms / 1.00x
- `cli_check_many_small_files`: 10.961ms / baseline 11.140ms / 0.98x
- `cli_fix_many_small_files`: 44.446ms / baseline 46.206ms / 0.96x
- `cli_fmt_many_small_files`: 47.569ms / baseline 48.874ms / 0.97x

解釈:

- parser-heavy / table-heavy の新規 case は stable scoring 用 evidence として追加した。
- `api_lint_large_document` の 1.10x は wall-clock variance と diagnostics-heavy mix の揺れとして扱う。link-heavy、inline-code-heavy、context inline index は 0.98x-1.01x に収まっている。
- rule semantics は変更していない。最適化は `is_inside_inline_code` の lookup 方式だけ。

## 2. Check / Fix / Fmt Convergence

- [x] `check` が no-write contract を保つことを再確認する。
- [x] `check --fix` と `fix` が同じ safe fix contract に従うことを確認する。
- [x] `fix` 再実行で同じ変更を繰り返さないことを確認する。
- [x] `fmt` が formatter policy の範囲だけを変更することを確認する。
- [x] `fmt` 再実行で差分が出ないことを確認する。

記録:

- `tests/cli_convergence_contract.rs` で `check` -> `check --fix` -> `fix` -> `fmt` -> `check` -> repeated `fmt` を同一 parser-heavy file に対して検証した。
- `check` は入力を書き換えない。
- `check --fix` は safe fix を適用し、`fix` 再実行は changed=false。
- `fmt` は formatter policy のみを適用し、再実行は changed=false。
- stdout JSON の `command`、`changed`、`summary.total_issues` を確認した。

## 3. Corpus Expansion

- [x] mixed corpus に parser-heavy document を追加する。
- [x] link-heavy / inline-code-heavy / table-heavy / reference-heavy の check/fix/fmt expectations を固定する。
- [x] dogfood 対象で見つかった違和感を再現可能な finding として記録する。

記録:

- parser-heavy convergence corpus は `tests/cli_convergence_contract.rs` に追加した。
- table-heavy benchmark は `api_lint_table_heavy_document` として追加した。
- parser-heavy fix / fmt benchmark は `api_fix_parser_heavy_document`、`api_format_parser_heavy_document` として追加した。
- dogfood は 0 current diagnostics。release-blocking finding はない。

## 4. Stable Score Dry Run

- [x] `v0.12.8` の安定版スコア項目に沿って dry-run 採点する。
- [x] 90点未満または hard blocker に相当する不足を次の task として分類する。
- [x] 採点根拠を tasks に残す。

Dry-run score:

| Category | Score | 根拠 |
| --- | ---: | --- |
| Precision correctness | 38 / 40 | context-sensitive regression、fixture harness、upstream golden が成功。HTML attribute parser の完全 shared 化は accepted limitation。 |
| Safe command behavior | 20 / 20 | parser-heavy convergence で no-write、safe fix、fix/fmt 冪等性を確認。 |
| Performance stability | 18 / 20 | new benchmark case と final perf-check が成功。large document lint に 1.10x の揺れがあるが、syntax-heavy / context index は安定。 |
| Release reproducibility | 9 / 10 | local `make release-check VERSION=v0.12.7` 成功。required CI は PR 後に確認する。 |
| Evidence quality | 9 / 10 | tasks、baseline、verification は揃った。最終ユーザー受け入れは `v0.12.8` で行う。 |

Total: 94 / 100

Hard blocker dry-run: 0 件。

`v0.12.8` に残す判断:

- required CI の結果を final score に反映する。
- ユーザー受け入れ判断を最終工程として実施する。

## 5. Release Preparation

- [x] crate version を `0.12.7` に更新する。
- [x] `CHANGELOG.md` に performance / convergence hardening を英語で記載する。
- [x] OpenSpec delta を main specs に同期し、完了後に archive する。
- [x] release 前に `make release-check VERSION=v0.12.7` を通す。

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
- [x] `make release-check VERSION=v0.12.7`
- [x] `git diff --check`

## Definition of Done

- [x] parser / context migration 後の performance evidence が揃っている。
- [x] check / fix / fmt の収束性が corpus で固定されている。
- [x] stable score dry-run が実行され、不足が分類されている。
- [x] 配布展開に関係する task は進んでいない。
