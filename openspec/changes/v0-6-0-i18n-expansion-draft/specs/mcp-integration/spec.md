## ADDED Requirements

### Requirement: MCP metadata tools SHALL accept explicit locale

MCP metadata tools は、client が明示した locale に基づいて metadata を localized できなければならない（SHALL）。

#### Scenario: localized rule list を取得する

- **WHEN** MCP client が `rule_list` に `locale` を渡す
- **THEN** system は lenient library resolver で locale を解決する
- **THEN** response は resolved locale と localized rule descriptions を含む
- **THEN** locale が省略された場合は English を返す

#### Scenario: localized rule detail を取得する

- **WHEN** MCP client が `rule_get` に rule ID と `locale` を渡す
- **THEN** system は該当 rule の localized description を返す
- **THEN** unknown rule handling は existing behavior と同じ error contract を保つ

### Requirement: MCP diagnostic and config responses SHALL localize when requested

MCP check/config tools は、client が明示した locale に基づいて diagnostic と config validation error を localized できなければならない（SHALL）。

#### Scenario: check_text diagnostic を localized 表示する

- **WHEN** MCP client が `check_text` に Markdown content と Japanese locale を渡す
- **THEN** response diagnostics は Japanese message を含む
- **THEN** response diagnostics は stable message ID と parameters を保持する

#### Scenario: config_validate error を localized 表示する

- **WHEN** MCP client が `config_validate` に invalid config と Japanese locale を渡す
- **THEN** response errors は Japanese message を含む
- **THEN** response errors は stable kind、message ID、parameters、expected/actual/allowed metadata を含む
