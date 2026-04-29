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

### Requirement: system SHALL verify localized rule document coverage

システムは、supported locale ごとに active rule の localized Markdown document が存在することを検証しなければならない（SHALL）。

#### Scenario: localized document coverage を検証する

- **WHEN** developer が repository quality gates を実行する
- **THEN** system は各 supported locale の `upstream_docs/<locale>/md*.md` を検査する
- **THEN** system は active rule ごとの Markdown document が欠けている場合に failure を報告する
- **THEN** English locale は existing `upstream_docs/md*.md` を canonical document として扱う

### Requirement: system SHALL reject English-copy localized rule documents

システムは、localized Markdown document が English canonical document の単純コピーになっていないことを検証しなければならない（SHALL）。

#### Scenario: English copy を検出する

- **WHEN** developer が repository quality gates を実行する
- **THEN** system は code fence、rule ID、URL、設定 key など翻訳対象外 token を除外して prose を比較する
- **THEN** supported non-English locale の prose が English prose と同一の場合は failure を報告する
- **THEN** 日本語、中国語、韓国語の document は対象 script の文字を含まない場合に failure を報告する
