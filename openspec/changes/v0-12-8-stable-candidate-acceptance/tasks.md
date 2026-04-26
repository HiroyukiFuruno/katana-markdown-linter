# Tasks

## Definition of Ready

- [x] `v0.12.7` の release と archive が完了している。
- [x] parser / context migration の evidence が存在する。
- [x] performance / convergence hardening の evidence が存在する。
- [x] `v0.12.x` は精度、速度、安定性だけを扱う方針である。
- [x] 進行中に task 外の高リスク不足が露見した場合だけ、作業を中断してユーザー判断を仰ぐ。

記録:

- `v0.12.7` は GitHub Release と crates.io publish まで完了済み。
- `v0.12.7` の branch hygiene は local `main`、remote `origin/main`、worktree 1 件だけの状態で完了済み。
- `v0.12.5` から `v0.12.7` までの evidence は、shared parser、context-sensitive rule migration、performance baseline、check/fix/fmt convergence として archive 済み。

## 0. Stable Score Definition

- [x] 100 点満点の stable score を tasks に明記する。
- [x] category score を `Precision correctness` 40点、`Safe command behavior` 20点、`Performance stability` 20点、`Release reproducibility` 10点、`Evidence quality` 10点に固定する。
- [x] `90+`、hard blocker なし、ユーザー受け入れあり、を stable 条件として固定する。
- [x] hard blocker 一覧を tasks に明記する。

採点基準:

| Category | Points | 評価対象 |
| --- | ---: | --- |
| Precision correctness | 40 | 誤検知、検出漏れ、構文除外、fixture matrix、upstream golden |
| Safe command behavior | 20 | `check` no-write、default-safe fix、`fix` / `fmt` 冪等性、collision safety |
| Performance stability | 20 | parser / context migration 後の benchmark、hot path、baseline explanation |
| Release reproducibility | 10 | local gate、PR required CI、OS matrix、package dry-run、install smoke、release-check |
| Evidence quality | 10 | tasks evidence、known limitation、dogfood findings、score report completeness |

stable 条件:

- score が 90 点以上。
- technical hard blocker が 0 件。
- ユーザーが `v0.12.8` を安定版として受け入れている。

technical hard blocker:

- 未分類の高優先度誤検知または検出漏れが残っている。
- `check` が入力を書き換える。
- default-safe fix に unsafe fix が混ざる。
- `fix` または `fmt` が再実行で不要差分を増やす。
- parser / AST 化で説明不能な重大 performance regression がある。
- `make release-check` または required CI が失敗している。
- dogfood で release-blocking finding が残っている。
- score report の根拠が tasks に残っていない。

## 1. Precision Score

- [x] 誤検知、検出漏れ、構文除外、fixture matrix、upstream golden の結果を確認する。
- [x] context-sensitive rule の未移譲または accepted limitation を確認する。
- [x] Precision correctness を 40 点満点で採点し、根拠を記録する。

Score: 38 / 40

根拠:

- `cargo test --locked --test document_false_positive_regressions`: 4 passed。
- `cargo test --locked --test rule_fixture_harness`: 18 passed。
- `cargo test --locked --test upstream_golden_comparison`: 5 passed。
- `cargo test --workspace --locked`: 212 passed、1 ignored。integration tests も通過。
- `cargo test --workspace --all-features --locked`: 212 passed、1 ignored。MCP feature tests 9 passed。
- `MD033`、`MD037`、`MD038`、`MD039`、`MD044`、`MD049`、`MD050` は shared parser または `DocumentContext` evidence に移譲済み。

Accepted limitation:

- HTML attribute parser の完全 shared 化と nested inline HTML semantics は、現時点では future AST work として扱う。既存 regression で release-blocking な誤検知または検出漏れは確認されていない。

## 2. Safe Command Behavior Score

- [x] `check` の no-write contract を確認する。
- [x] `check --fix` と `fix` の safe fix contract を確認する。
- [x] `fix` と `fmt` の冪等性を確認する。
- [x] fix collision と formatter scope の evidence を確認する。
- [x] Safe command behavior を 20 点満点で採点し、根拠を記録する。

Score: 20 / 20

根拠:

- `tests/cli_convergence_contract.rs` が `check` -> `check --fix` -> `fix` -> `fmt` -> `check` -> repeated `fmt` の収束を確認している。
- `check` は入力 file を変更しない。
- `check --fix` と `fix` は default-safe fix だけを適用する。
- `fix` 再実行は changed=false。
- `fmt` は formatter policy だけを適用し、再実行は changed=false。
- unsafe fix は default-safe fix へ混入していない。

## 3. Performance Score

- [x] parser / context migration 後の `make perf-check` を確認する。
- [x] link-heavy / inline-code-heavy / reference-heavy / parser index construction の結果を確認する。
- [x] unexplained regression がある場合は hard blocker または accepted limitation に分類する。
- [x] Performance stability を 20 点満点で採点し、根拠を記録する。

Score: 19 / 20

`make perf-check` result:

| Case | Current | Baseline | Ratio |
| --- | ---: | ---: | ---: |
| `api_lint_large_document` | 10.913ms | 10.766ms | 1.01x |
| `api_lint_clean_large_document` | 6.839ms | 6.763ms | 1.01x |
| `api_fix_large_document` | 34.221ms | 34.300ms | 1.00x |
| `api_lint_link_heavy_document` | 13.318ms | 13.467ms | 0.99x |
| `api_lint_inline_code_heavy_document` | 13.610ms | 13.525ms | 1.01x |
| `api_lint_reference_heavy_document` | 2.811ms | 2.814ms | 1.00x |
| `api_lint_table_heavy_document` | 4.756ms | 4.698ms | 1.01x |
| `api_fix_parser_heavy_document` | 25.290ms | 25.060ms | 1.01x |
| `api_format_parser_heavy_document` | 0.893ms | 0.846ms | 1.06x |
| `context_inline_token_index_large_document` | 9.883ms | 9.879ms | 1.00x |
| `cli_check_many_small_files` | 10.752ms | 11.140ms | 0.97x |
| `cli_fix_many_small_files` | 44.217ms | 46.206ms | 0.96x |
| `cli_fmt_many_small_files` | 49.310ms | 48.874ms | 1.01x |

解釈:

- required case と report schema は成功。
- parser-heavy / syntax-heavy / CLI path は baseline 比 0.96x-1.06x に収まっている。
- 説明不能な重大 regression はない。
- score から 1 点差し引いた理由は、`api_format_parser_heavy_document` と `context_build_large_document` に 1.06x の揺れがあり、安定版後も継続観測対象にするため。

## 4. Release Reproducibility Score

- [x] local gate と CI gate の対応を確認する。
- [ ] Ubuntu / macOS / Windows の required CI が通っていることを確認する。
- [x] package dry-run、install smoke、release-check の結果を確認する。
- [x] Release reproducibility を 10 点満点で採点し、根拠を記録する。

Score: 9 / 10

根拠:

- `make release-check VERSION=v0.12.8`: passed。
- `coverage-blocking`: 605 uncovered lines <= baseline 623。
- `cargo publish --dry-run --locked --allow-dirty`: passed。
- `cargo install --path . --locked --force --bin kml`: passed。
- `mcp-stdio-smoke`: passed。
- `action-smoke`: passed。
- PR required CI は release PR 作成後に確認するため、現時点では 1 点保留。

## 5. Evidence Quality Score

- [x] score report に category score、hard blocker、known limitation、verification result が含まれることを確認する。
- [x] dogfood findings が未対応のまま隠れていないことを確認する。
- [x] accepted limitation が将来の作業者に伝わる粒度で記録されていることを確認する。
- [x] Evidence quality を 10 点満点で採点し、根拠を記録する。

Score: 9 / 10

根拠:

- `openspec/changes/v0-12-8-stable-candidate-acceptance/stable-score.json` に machine-readable report を追加した。
- `make dogfood`: 0 current diagnostics、4 baseline diagnostics、4 resolved。
- accepted limitation は tasks に明記した。
- 最終ユーザー受け入れ判断が未記録のため 1 点保留。

## 6. Hard Blocker Review

- [x] 未分類の高優先度誤検知または検出漏れがないことを確認する。
- [x] `check` が入力を書き換えないことを確認する。
- [x] default-safe fix に unsafe fix が混ざっていないことを確認する。
- [x] `fix` または `fmt` が再実行で不要差分を増やさないことを確認する。
- [x] parser / AST 化で説明不能な重大 performance regression がないことを確認する。
- [x] `make release-check` が成功していることを確認する。
- [ ] required CI が成功していることを確認する。
- [x] dogfood の release-blocking finding が残っていないことを確認する。
- [x] score report の根拠が tasks に残っていることを確認する。

Technical hard blocker: 0 件。

Pending final gates:

- GitHub Actions required CI。
- ユーザー受け入れ判断。

## 7. User Acceptance

- [ ] score total、category score、hard blocker、known limitation、verification result をユーザーに提示する。
- [ ] ユーザーが `v0.12.8` を安定版として受け入れるか判断する。
- [ ] 受け入れられた場合、`v0.13.0` の DoR を満たす条件として記録する。
- [ ] 受け入れられない場合、追加の `v0.12.x` hardening scope を定義する。

現時点の判定:

| Category | Score |
| --- | ---: |
| Precision correctness | 38 / 40 |
| Safe command behavior | 20 / 20 |
| Performance stability | 19 / 20 |
| Release reproducibility | 9 / 10 |
| Evidence quality | 9 / 10 |

Total: 95 / 100

Technical hard blocker: 0 件。

安定版として確定するには、required CI の通過とユーザー受け入れ判断が必要。

## 8. Release Preparation

- [x] crate version を `0.12.8` に更新する。
- [x] `CHANGELOG.md` に stable candidate evaluation を英語で記載する。
- [x] OpenSpec delta を main specs に同期する。
- [ ] ユーザー受け入れ判断の完了後に archive する。
- [x] release 前に `make release-check VERSION=v0.12.8` を通す。

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
- [x] `make release-check VERSION=v0.12.8`
- [ ] GitHub Actions required CI
- [x] `git diff --check`
- [ ] ユーザー受け入れ判断

## Definition of Done

- [x] stable score が 100 点満点で採点されている。
- [x] score が 90 点以上である。
- [x] technical hard blocker が 0 件である。
- [ ] required CI が通過している。
- [ ] ユーザーが安定版として受け入れている。
- [ ] `v0.13.0` に進む DoR が満たされたことが記録されている。
