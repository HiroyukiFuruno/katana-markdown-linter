# Tasks

## Definition of Ready

- [ ] `v0.12.7` の release と archive が完了している。
- [ ] parser / context migration の evidence が存在する。
- [ ] performance / convergence hardening の evidence が存在する。
- [ ] `v0.12.x` は精度、速度、安定性だけを扱う方針である。
- [ ] 進行中に task 外の高リスク不足が露見した場合だけ、作業を中断してユーザー判断を仰ぐ。

## 0. Stable Score Definition

- [ ] 100 点満点の stable score を tasks に明記する。
- [ ] category score を `Precision correctness` 40点、`Safe command behavior` 20点、`Performance stability` 20点、`Release reproducibility` 10点、`Evidence quality` 10点に固定する。
- [ ] `90+`、hard blocker なし、ユーザー受け入れあり、を stable 条件として固定する。
- [ ] hard blocker 一覧を tasks に明記する。

## 1. Precision Score

- [ ] 誤検知、検出漏れ、構文除外、fixture matrix、upstream golden の結果を確認する。
- [ ] context-sensitive rule の未移譲または accepted limitation を確認する。
- [ ] Precision correctness を 40 点満点で採点し、根拠を記録する。

## 2. Safe Command Behavior Score

- [ ] `check` の no-write contract を確認する。
- [ ] `check --fix` と `fix` の safe fix contract を確認する。
- [ ] `fix` と `fmt` の冪等性を確認する。
- [ ] fix collision と formatter scope の evidence を確認する。
- [ ] Safe command behavior を 20 点満点で採点し、根拠を記録する。

## 3. Performance Score

- [ ] parser / context migration 後の `make perf-check` を確認する。
- [ ] link-heavy / inline-code-heavy / reference-heavy / parser index construction の結果を確認する。
- [ ] unexplained regression がある場合は hard blocker または accepted limitation に分類する。
- [ ] Performance stability を 20 点満点で採点し、根拠を記録する。

## 4. Release Reproducibility Score

- [ ] local gate と CI gate の対応を確認する。
- [ ] Ubuntu / macOS / Windows の required CI が通っていることを確認する。
- [ ] package dry-run、install smoke、release-check の結果を確認する。
- [ ] Release reproducibility を 10 点満点で採点し、根拠を記録する。

## 5. Evidence Quality Score

- [ ] score report に category score、hard blocker、known limitation、verification result が含まれることを確認する。
- [ ] dogfood findings が未対応のまま隠れていないことを確認する。
- [ ] accepted limitation が将来の作業者に伝わる粒度で記録されていることを確認する。
- [ ] Evidence quality を 10 点満点で採点し、根拠を記録する。

## 6. Hard Blocker Review

- [ ] 未分類の高優先度誤検知または検出漏れがないことを確認する。
- [ ] `check` が入力を書き換えないことを確認する。
- [ ] default-safe fix に unsafe fix が混ざっていないことを確認する。
- [ ] `fix` または `fmt` が再実行で不要差分を増やさないことを確認する。
- [ ] parser / AST 化で説明不能な重大 performance regression がないことを確認する。
- [ ] `make release-check` と required CI が成功していることを確認する。
- [ ] dogfood の release-blocking finding が残っていないことを確認する。
- [ ] score report の根拠が tasks に残っていることを確認する。

## 7. User Acceptance

- [ ] score total、category score、hard blocker、known limitation、verification result をユーザーに提示する。
- [ ] ユーザーが `v0.12.8` を安定版として受け入れるか判断する。
- [ ] 受け入れられた場合、`v0.13.0` の DoR を満たす条件として記録する。
- [ ] 受け入れられない場合、追加の `v0.12.x` hardening scope を定義する。

## 8. Release Preparation

- [ ] crate version を `0.12.8` に更新する。
- [ ] `CHANGELOG.md` に stable candidate evaluation を英語で記載する。
- [ ] OpenSpec delta を main specs に同期し、完了後に archive する。
- [ ] release 前に `make release-check VERSION=v0.12.8` を通す。

## Verification

- [ ] `make fmt-check`
- [ ] `make lint`
- [ ] `make ast-lint`
- [ ] `cargo test --workspace --locked`
- [ ] `cargo test --workspace --all-features --locked`
- [ ] `cargo test --locked --test document_false_positive_regressions`
- [ ] `cargo test --locked --test rule_fixture_harness`
- [ ] `cargo test --locked --test upstream_golden_comparison`
- [ ] `make dogfood`
- [ ] `make perf-check`
- [ ] `make release-check VERSION=v0.12.8`
- [ ] GitHub Actions required CI
- [ ] `git diff --check`
- [ ] ユーザー受け入れ判断

## Definition of Done

- [ ] stable score が 100 点満点で採点されている。
- [ ] score が 90 点以上である。
- [ ] hard blocker が 0 件である。
- [ ] ユーザーが安定版として受け入れている。
- [ ] `v0.13.0` に進む DoR が満たされたことが記録されている。
