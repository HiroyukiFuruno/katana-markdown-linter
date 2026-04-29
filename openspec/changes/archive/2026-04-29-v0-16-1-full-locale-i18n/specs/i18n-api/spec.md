## MODIFIED Requirements

### Requirement: library SHALL resolve arbitrary locale code strings for embedding consumers

Library API は、consumer application から受け取る任意の language / locale code string を kml が対応する locale に解決できなければならない（SHALL）。

#### Scenario: supported locale code を解決する

- **WHEN** consumer が `en`, `en-US`, `ja`, `ja-JP`, `zh-CN`, `zh_TW.UTF-8`, `ko`, `pt-BR`, `fr-FR`, `de-DE`, `es-ES`, または `it-IT` を resolver に渡す
- **THEN** system は対応する supported locale を返す
- **THEN** system は大文字小文字、hyphen / underscore、charset suffix の違いを吸収する

#### Scenario: unsupported locale code を解決する

- **WHEN** consumer が unsupported locale code を default resolver に渡す
- **THEN** system は English locale を返す
- **WHEN** consumer が unsupported locale code を fallback 指定 resolver に渡す
- **THEN** system は consumer が指定した fallback locale を返す

### Requirement: library SHALL expose localized rule metadata descriptions

Library API は、rule metadata の description を caller が指定した locale code に基づいて描画できなければならない（SHALL）。

#### Scenario: known rule description を localized 表示する

- **WHEN** consumer が known rule id、English fallback description、supported locale code を渡す
- **THEN** system はその rule の localized description を返す
- **THEN** consumer は diagnostic-specific helper を直接使う必要がない
- **THEN** localized description は English fallback の単純コピーではない

#### Scenario: unsupported locale の rule description を表示する

- **WHEN** consumer が unsupported locale code を渡す
- **THEN** system は English fallback description を返す
- **THEN** system は canonical English description を保持する API を残す

### Requirement: library SHALL expose localized rule catalog APIs

Library API は、consumer application が rule catalog を指定 locale で取得できる API を提供しなければならない（SHALL）。

#### Scenario: localized catalog を取得する

- **WHEN** consumer が supported または unsupported language code を渡して localized catalog API を呼び出す
- **THEN** system は existing resolver fallback policy に従って locale を解決する
- **THEN** system は rule description を解決済み locale に合わせて返す
- **THEN** system は rule ID、rule name、docs URL、fixability、lifecycle を保持する
- **THEN** supported locale の description は English canonical description の単純コピーではない

### Requirement: translation coverage SHALL be gateable

Translation coverage は、supported locale ごとの漏れを CI / AST lint で検出できなければならない（SHALL）。

#### Scenario: translation coverage を検証する

- **WHEN** developer が repository quality gates を実行する
- **THEN** system は supported locale が同じ message ID set を持つことを確認する
- **THEN** system は active rule descriptions が全 supported locale catalog に存在することを確認する
- **THEN** system は supported locale の rule document Markdown が全 active rule 分存在することを確認する
- **THEN** system は missing translation と English の単純コピーを failure として報告する

### Requirement: locale translation data SHALL be separated from runtime logic

Locale translation data は、Rust の分岐ロジックではなく、機械検査できる JSON catalog として管理されなければならない（SHALL）。

#### Scenario: locale JSON catalog を検証する

- **WHEN** developer が repository quality gates を実行する
- **THEN** system は `src/i18n/locales/languages.json` と `<locale>.json` の対応を確認する
- **THEN** system は KatanA 本体と同じ locale code set であることを確認する
- **THEN** system は `rule_descriptions` の key set が active rule set と一致することを確認する
- **THEN** system は English catalog が canonical rule metadata と一致することを確認する

## ADDED Requirements

### Requirement: library SHALL expose localized rule documentation Markdown

Library API は、rule documentation Markdown を caller が指定した locale code に基づいて取得できなければならない（SHALL）。

#### Scenario: supported locale の rule document を取得する

- **WHEN** consumer が known rule id と supported locale code を渡す
- **THEN** system はその locale の Markdown document を返す
- **THEN** Markdown document は rule ID、設定 key、example code block を保持する
- **THEN** Markdown prose は English document の単純コピーではない

#### Scenario: unsupported locale の rule document を取得する

- **WHEN** consumer が unsupported locale code を渡す
- **THEN** system は English Markdown document を返す
- **THEN** unknown rule handling は existing behavior と同じ error contract を保つ
