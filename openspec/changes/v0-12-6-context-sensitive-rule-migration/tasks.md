# Tasks

## Definition of Ready

- [ ] `v0.12.5` の release と archive が完了している。
- [ ] shared parser token または `DocumentContext` index の foundation が存在する。
- [ ] `v0.12.x` は精度、速度、安定性だけを扱う方針である。
- [ ] 進行中に task 外の高リスク不足が露見した場合だけ、作業を中断してユーザー判断を仰ぐ。

## 0. Rule Migration Inventory

- [ ] context-sensitive rule を全件列挙する。
- [ ] 各 rule を `parser-backed`、`document-context-backed`、`line-local-by-spec`、`future-work` に分類する。
- [ ] 現在の誤検知、検出漏れ、fix risk、test gap を rule ごとに記録する。
- [ ] `future-work` にした rule の理由と次の解消条件を記録する。

## 1. Link And Reference Family

- [ ] `v0.12.5` で移譲した link family の migration 状態を再確認する。
- [ ] `MD053` の duplicate reference definition と shared reference index の整合を確認する。
- [ ] `MD034`、`MD051`、`MD052`、`MD054`、`MD059` の document-level regression を拡張する。

## 2. Inline Content Family

- [ ] emphasis / strong / code span 系 rule の inline code 除外を共有 token へ寄せる。
- [ ] HTML block / inline HTML の除外判定を `DocumentContext` または parser token へ寄せる。
- [ ] `MD037`、`MD038`、`MD039`、`MD044`、`MD049`、`MD050` の境界値 test を追加する。

## 3. Block Structure Family

- [ ] table rule が `DocumentContext` の table index と矛盾しないことを確認する。
- [ ] list / heading / fence / command prompt rule が code block 内を通常本文として扱わないことを確認する。
- [ ] `MD011`、`MD013`、`MD014`、`MD026`、`MD029`、`MD031`、`MD046`、`MD048`、`MD055`、`MD056`、`MD060` の mixed fixture を拡張する。

## 4. Fix Safety

- [ ] migrated rule の fix range が original source range に基づくことを確認する。
- [ ] overlapping / adjacent fix の collision test を追加または更新する。
- [ ] default-safe fix が unsafe fix を混ぜていないことを確認する。

## 5. Performance

- [ ] 実装前に `make perf-check` の結果を記録する。
- [ ] context-sensitive rule migration 後の parser / context index cost を測定する。
- [ ] repeated scan が減った箇所と増えた箇所を tasks に記録する。

## 6. Release Preparation

- [ ] crate version を `0.12.6` に更新する。
- [ ] `CHANGELOG.md` に context-sensitive rule migration を英語で記載する。
- [ ] OpenSpec delta を main specs に同期し、完了後に archive する。
- [ ] release 前に `make release-check VERSION=v0.12.6` を通す。

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
- [ ] `make release-check VERSION=v0.12.6`
- [ ] `git diff --check`

## Definition of Done

- [ ] context-sensitive rule の migration classification が完了している。
- [ ] 優先 rule family が単一行文字列だけに依存しない。
- [ ] migration 後の誤検知と検出漏れが fixture で固定されている。
- [ ] fix safety と performance evidence が記録されている。
