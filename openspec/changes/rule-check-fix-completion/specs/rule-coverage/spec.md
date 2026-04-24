## MODIFIED Requirements

### Requirement: system SHALL check documents according to official markdownlint rule behavior

システムは、公式 markdownlint の rule behavior に準じて Markdown 文書を check しなければならない（SHALL）。

#### Scenario: fixture matrix に基づいて文書を check する

- **WHEN** system が Markdown 文書を解析する
- **THEN** system は公式doc由来fixtureの pass / fail 判定と一致する
- **THEN** system は rule の設定に従って enable / disable を判定する
- **THEN** system は境界値とイレギュラーパターンをfixtureで検証する

### Requirement: system SHALL support automatic fix only for rules that define fix behavior

システムは、upstream implementation または official documentation から安全な fix behavior を確認できる rule に対してのみ自動修正を適用しなければならない（SHALL）。

#### Scenario: fixture matrix に基づいて fix を実行する

- **WHEN** system が document fix を要求される
- **THEN** system は fixable rule の before / after fixture と一致する修正だけを適用する
- **THEN** system は fix 非対応 rule を無理に変更しない
- **THEN** system は fix 非対応 rule の理由を metadata として保持する
