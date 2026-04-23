## ADDED Requirements

### Requirement: system SHALL expose every official markdownlint rule in the rule catalog

システムは、公式 markdownlint documentation にある active rule を rule catalog として公開し、deprecated / removed rule は lifecycle state として区別しなければならない（SHALL）。

#### Scenario: rule catalog を参照する

- **WHEN** developer が利用可能な rule 一覧を取得する
- **THEN** system は official docs にある active rule ID と説明を返す
- **THEN** system は各 rule の metadata を区別できる
- **THEN** system は deprecated / removed rule を active rule と混同しない

### Requirement: system SHALL check documents according to official markdownlint rule behavior

システムは、公式 markdownlint の rule behavior に準じて Markdown 文書を check しなければならない（SHALL）。

#### Scenario: 文書を check する

- **WHEN** system が Markdown 文書を解析する
- **THEN** system は rule ごとの違反を返す
- **THEN** system は rule の設定に従って enable / disable を判定する

### Requirement: system SHALL support automatic fix only for rules that define fix behavior

システムは、upstream implementation または official documentation から安全な fix behavior を確認できる rule に対してのみ自動修正を適用しなければならない（SHALL）。

#### Scenario: fix を実行する

- **WHEN** system が document fix を要求される
- **THEN** system は fixable rule の修正だけを適用する
- **THEN** system は fix 非対応 rule を無理に変更しない
- **THEN** system は fix 非対応 rule の理由を metadata として保持する
