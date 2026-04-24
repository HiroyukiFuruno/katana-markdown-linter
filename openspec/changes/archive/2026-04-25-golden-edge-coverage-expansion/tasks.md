## Definition of Ready

- [x] `check-diagnostic-i18n` が archive 済み、または全 task 完了済みであること
- [x] `docs/rule-coverage-dashboard.md` が現行 fixture matrix から再生成可能であること
- [x] upstream oracle 実行に必要な Node/npm 環境または skip 条件が明確であること
- [x] known delta file の schema が確認済みであること
- [x] `safe-fix-strategy-expansion` より先に実施する場合、fix implementation は含めないこと

## 1. Coverage Inventory

- [x] 1.1 active rule ごとの Golden 状態を一覧化する
- [x] 1.2 Edge coverage 0 の rule を一覧化する
- [x] 1.3 known delta と dashboard 表示の整合性を確認する
- [x] 1.4 `pending` を残す rule は rule id と残す理由を一覧化する

## 2. Golden Corpus Expansion

- [x] 2.1 `MD005` の golden case を追加する
- [x] 2.2 `MD030` の golden case を追加する
- [x] 2.3 reference/image/content group は pending として dashboard に残す
- [x] 2.4 whitespace/emphasis/code group は pending として dashboard に残す
- [x] 2.5 `MD005` / `MD030` の before/after golden cases を追加する

## 3. Edge Case Expansion

- [x] 3.1 no trailing newline / empty file cases は既存 harness で固定する
- [x] 3.2 CRLF cases を追加する
- [x] 3.3 Unicode / multibyte cases を追加する
- [x] 3.4 code span / fenced code exclusion cases を追加する
- [x] 3.5 table/list boundary cases を追加する

## 4. Harness And Dashboard

- [x] 4.1 dashboard generator が Golden / Edge count を現行 schema で検証する
- [x] 4.2 pending が残る場合は reason を必須にする
- [x] 4.3 known delta に reason / resolution がない場合は fail する
- [x] 4.4 `docs/rule-coverage-dashboard.md` を更新する

## Verification

- [x] `cargo test --test upstream_golden_comparison --locked` が成功する
- [x] `cargo test --test rule_fixture_harness --locked` が成功する
- [x] `make rule-dashboard` が成功する
- [x] `make check` が成功する
- [x] `git diff --check` が成功する

## Definition of Done

- [x] Golden `pending` が dashboard 上で理由なしに残っていないこと
- [x] active rule の edge coverage が rule group ごとに説明可能であること
- [x] known delta は reason と resolution を持つこと
- [x] この change 単体では runtime behavior を変更していないこと
