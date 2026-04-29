## ADDED Requirements

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

### Requirement: generated localized rule documents SHALL match locale JSON data

生成対象の localized Markdown document は、locale JSON catalog の description と document label data から再生成可能でなければならない（SHALL）。

#### Scenario: generated document と locale JSON を照合する

- **WHEN** developer が repository quality gates を実行する
- **THEN** system は generated locale の document title が locale JSON の rule description と一致することを確認する
- **THEN** system は generated locale の document が locale JSON の rule document labels を含むことを確認する
- **THEN** system は手書き locale document には rule ID、localized metadata label、非 English prose の最低限の整合性を要求する
