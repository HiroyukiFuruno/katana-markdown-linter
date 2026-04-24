## Purpose

公式 markdownlint 文書とローカル実装の差分を構造的に検証し、定義ズレを早期に検知する。

## Requirements

### Requirement: system SHALL structurally parse markdownlint rule documents

システムは、upstream の各 rule document を構造的に解析しなければならない（SHALL）。

#### Scenario: rule document を読む

- **WHEN** system が rule の document md を解析する
- **THEN** system は rule id、name、summary、tags、properties を抽出する
- **THEN** system は parser が壊れた場合に失敗を報告する

### Requirement: system SHALL compare local rule metadata against upstream rule documents

システムは、local rule metadata が upstream rule document と乖離していないかを比較しなければならない（SHALL）。

#### Scenario: metadata の乖離を検出する

- **WHEN** system が local rule metadata を点検する
- **THEN** system は description、fixability、default config、property schema の差分を検出する
- **THEN** system は差分を JSON と Markdown summary で報告する

### Requirement: system SHALL compare local config helpers against upstream config contracts

システムは、local の `.markdownlint.json` helper と upstream config contract を比較しなければならない（SHALL）。

#### Scenario: config contract を検査する

- **WHEN** system が config drift check を実行する
- **THEN** system は property の型、必須性、既定値の差分を確認する
- **THEN** system は既知の差分と未知の差分を区別する
- **THEN** system は未知の差分を CI failure として扱う
