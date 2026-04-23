## Definition of Ready
- [x] phase1 の scaffold が完成していること
- [x] phase1 の `tasks.md` が全て完了していること
- [x] 公式 markdownlint の rule 一覧と config 仕様を参照できること
- [x] `.markdownlint.json` の生成形式が JSON object として確定していること
- [x] phase2 の完了条件が全 active rule の check 実装必須として固定されていること
- [x] 実装開始時点の未対応 rule 一覧が可視化され、全て phase2 の実装対象として扱われていること

## 1. Rule Coverage

- [ ] 1.1 公式 docs にある全 active rule を catalog 化し、deprecated / removed rule は別 state として記録する。各 rule は `mdxxx.rs` 単位で pure check/fix として分離し、`Document` を入力 contract とする
- [ ] 1.2 rule ごとの check 実装を official behavior に合わせ、未実装 rule が残る場合は `missing_check` として失敗扱いにする
- [ ] 1.3 fix 可否を `fixable` / `not_fixable` / `unknown_needs_review` の metadata で明示する
- [ ] 1.4 実装開始時点の未対応 rule 一覧を生成し、phase2 完了までに空にする

## 2. Configuration

- [x] 2.1 `.markdownlint.json` の default config を生成する helper を整備する
- [x] 2.2 既存 config の読み込みと validation を整備する
- [x] 2.3 rule 設定の不正値を検出できるようにする

## 3. Quality Gates

- [x] 3.1 rule ごとの unit test を追加する
- [x] 3.2 config helper の integration test を追加する
- [x] 3.3 check と fix の両方で回帰しないことを確認する

## Definition of Done
- [ ] official docs にある全 active rule が `implemented_check` として扱われていること
- [ ] fix 可否が全 active rule で `fixable` / `not_fixable` のどちらかに分類され、`unknown_needs_review` が残っていないこと
- [ ] config helper が `.markdownlint.json` を作成・検証し、不正値を具体的な rule ID / property 名付きで報告できること
