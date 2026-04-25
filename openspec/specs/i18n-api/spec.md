## ADDED Requirements

### Requirement: library SHALL resolve arbitrary locale code strings for embedding consumers

Library API は、consumer application から受け取る任意の language / locale code string を kml が対応する locale に解決できなければならない（SHALL）。

#### Scenario: supported locale code を解決する

- **WHEN** consumer が `en`, `en-US`, `ja`, `ja-JP`, `en_US.UTF-8`, または `ja_JP.UTF-8` を resolver に渡す
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

- **WHEN** consumer が known rule id、English fallback description、Japanese locale code を渡す
- **THEN** system はその rule の Japanese description を返す
- **THEN** consumer は diagnostic-specific helper を直接使う必要がない

#### Scenario: unsupported locale の rule description を表示する

- **WHEN** consumer が unsupported locale code を渡す
- **THEN** system は English fallback description を返す
- **THEN** system は `RuleMeta.description` の existing value を変更しない

### Requirement: v0.4.3 locale metadata API SHALL remain source-compatible

v0.4.3 の locale metadata API は patch release として既存 public API を壊してはならない（SHALL）。

#### Scenario: existing consumer を compile する

- **WHEN** existing consumer が `RuleMeta.description` または existing `Locale` API を利用している
- **THEN** system は existing field と behavior を維持する
- **THEN** system は `Locale` を patch release で non-exhaustive に変更しない
- **THEN** system は new helper を additive API として提供する
