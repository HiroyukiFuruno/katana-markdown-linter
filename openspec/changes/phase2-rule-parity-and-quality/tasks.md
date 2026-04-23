## Definition of Ready
- [x] phase1 の scaffold が完成していること
- [x] phase1 の `tasks.md` が全て完了していること
- [x] 公式 markdownlint の rule 一覧と config 仕様を参照できること
- [x] `.markdownlint.json` の生成形式が JSON object として確定していること
- [x] phase2 の完了条件が全 active rule の check 実装必須として固定されていること
- [x] 実装開始時点の未対応 rule 一覧が可視化され、全て phase2 の実装対象として扱われていること

## 1. Rule Coverage

- [x] 1.1 公式 docs にある全 active rule を catalog 化し、deprecated / removed rule は別 state として記録する。各 rule は `mdxxx.rs` 単位で pure check/fix として分離し、`Document` を入力 contract とする
  - [x] MD001 heading-increment
  - [x] MD003 heading-style
  - [x] MD004 ul-style
  - [x] MD005 list-indent
  - [x] MD007 ul-indent
  - [x] MD009 trailing-spaces
  - [x] MD010 hard-tabs
  - [x] MD011 no-reversed-links
  - [x] MD012 no-multiple-blanks
  - [x] MD013 line-length
  - [x] MD014 dollar-signs-before-commands
  - [x] MD018 no-missing-space-atx
  - [x] MD019 no-multiple-space-atx
  - [x] MD020 no-space-in-blockquote
  - [x] MD021 no-multiple-space-blockquote
  - [x] MD022 blanks-around-headings
  - [x] MD023 heading-start-left
  - [x] MD024 no-duplicate-heading
  - [x] MD025 single-h1
  - [x] MD026 no-trailing-punctuation
  - [x] MD027 no-multiple-space-blockquote
  - [x] MD028 no-blanks-blockquote
  - [x] MD029 ol-prefix
  - [x] MD030 list-marker-space
  - [x] MD031 blanks-around-fences
  - [x] MD032 blanks-around-lists
  - [x] MD033 no-inline-html
  - [x] MD034 no-bare-urls
  - [x] MD035 hr-style
  - [x] MD036 no-emphasis-as-heading
  - [x] MD037 spaces-in-emphasis
  - [x] MD038 spaces-in-code
  - [x] MD039 no-space-in-links
  - [x] MD040 fenced-code-language
  - [x] MD041 first-line-heading
  - [x] MD042 no-empty-links
  - [x] MD043 required-headings
  - [x] MD044 proper-names
  - [x] MD045 no-alt-text
  - [x] MD046 code-block-style
  - [x] MD047 single-trailing-newline
  - [x] MD048 code-fence-style
  - [x] MD049 emphasis-style
  - [x] MD050 strong-style
  - [x] MD051 link-fragments
  - [x] MD052 reference-links-images
  - [x] MD053 link-definitions
  - [x] MD054 link-style
  - [x] MD055 table-pipe-style
  - [x] MD056 table-column-count
  - [x] MD058 table-spacing
  - [x] MD059 link-text
  - [x] MD060 table-column-style
- [x] 1.2 rule ごとの check 実装を official behavior に合わせ、未実装 rule が残る場合は `missing_check` として失敗扱いにする
- [x] 1.3 fix 可否を `fixable` / `not_fixable` / `unknown_needs_review` の metadata で明示する
- [x] 1.4 実装開始時点の未対応 rule 一覧を生成し、phase2 完了までに空にする

## 2. Configuration

- [x] 2.1 `.markdownlint.json` の default config を生成する helper を整備する
- [x] 2.2 既存 config の読み込みと validation を整備する
- [x] 2.3 rule 設定の不正値を検出できるようにする

## 3. Quality Gates

- [x] 3.1 rule ごとの unit test を追加する
- [x] 3.2 config helper の integration test を追加する
- [x] 3.3 check と fix の両方で回帰しないことを確認する

## Definition of Done
- [x] official docs にある全 active rule が `implemented_check` として扱われていること
- [x] fix 可否が全 active rule で `fixable` / `not_fixable` のどちらかに分類され、`unknown_needs_review` が残っていないこと
- [x] config helper が `.markdownlint.json` を作成・検証し、不正値を具体的な rule ID / property 名付きで報告できること
