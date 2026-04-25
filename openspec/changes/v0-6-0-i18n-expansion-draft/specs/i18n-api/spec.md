## ADDED Requirements

### Requirement: library SHALL expose localized rule catalog APIs

Library API は、consumer application が rule catalog を指定 locale で取得できる API を提供しなければならない（SHALL）。

#### Scenario: localized catalog を取得する

- **WHEN** consumer が supported または unsupported language code を渡して localized catalog API を呼び出す
- **THEN** system は existing resolver fallback policy に従って locale を解決する
- **THEN** system は rule description を解決済み locale に合わせて返す
- **THEN** system は rule ID、rule name、docs URL、fixability、lifecycle を保持する

### Requirement: Locale enum SHALL be non-exhaustive for future locale additions

`Locale` enum は、将来の locale 追加に備えて non-exhaustive でなければならない（SHALL）。

#### Scenario: future locale を追加する

- **WHEN** system が v0.6.0 以降で supported locale を増やす
- **THEN** external consumer は wildcard match を要求される
- **THEN** system は locale 追加ごとに exhaustive match consumer を破壊しない
- **THEN** documentation は v0.5.x 以前の exhaustive match consumer 向け migration note を含む

### Requirement: config validation errors SHALL have localized stable message metadata

Config validation errors は、localized rendering に使える stable message ID と structured parameters を持たなければならない（SHALL）。

#### Scenario: invalid config を localized 表示する

- **WHEN** system が unknown rule、unknown property、invalid type、invalid enum、invalid root を検出する
- **THEN** system は stable message ID を返す
- **THEN** system は rule ID、property、expected、actual、allowed values を applicable な structured parameters として返す
- **THEN** system は English fallback と Japanese message rendering を提供する

### Requirement: translation coverage SHALL be gateable

Translation coverage は、supported locale ごとの漏れを CI / AST lint で検出できなければならない（SHALL）。

#### Scenario: translation coverage を検証する

- **WHEN** developer が repository quality gates を実行する
- **THEN** system は supported locale が同じ message ID set を持つことを確認する
- **THEN** system は active rule descriptions が Japanese catalog に存在することを確認する
- **THEN** system は missing translation を failure として報告する

## MODIFIED Requirements

### Requirement: library SHALL expose localized rule metadata descriptions

Library API は、rule metadata の description を caller が指定した locale code に基づいて描画できなければならない（SHALL）。

#### Scenario: known rule description を localized 表示する

- **WHEN** consumer が known rule id、English fallback description、Japanese locale code を渡す
- **THEN** system はその rule の Japanese description を返す
- **THEN** consumer は diagnostic-specific helper を直接使う必要がない

#### Scenario: unsupported locale の rule description を表示する

- **WHEN** consumer が unsupported locale code を渡す
- **THEN** system は English fallback description を返す
- **THEN** system は canonical English description を保持する API を残す
