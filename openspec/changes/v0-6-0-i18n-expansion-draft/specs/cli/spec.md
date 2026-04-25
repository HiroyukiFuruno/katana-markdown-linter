## ADDED Requirements

### Requirement: CLI rule introspection SHALL honor selected locale

CLI は、`kml rule` と `kml rule <id>` の rule description を selected locale で表示しなければならない（SHALL）。

#### Scenario: rule list を localized 表示する

- **WHEN** user が `kml rule --locale ja` を実行する
- **THEN** CLI は rule ID と rule name に加えて Japanese description を表示する
- **THEN** unsupported explicit locale は existing CLI locale policy に従って hard error になる

#### Scenario: rule JSON を localized 表示する

- **WHEN** user が `kml rule MD003 --locale ja --format json` を実行する
- **THEN** CLI JSON は selected locale を示す
- **THEN** CLI JSON は localized description を含む
- **THEN** CLI JSON は English canonical description を失わない

### Requirement: CLI config validation errors SHALL expose localized message metadata

CLI は config validation error を text と JSON の両方で localized metadata 付きで返さなければならない（SHALL）。

#### Scenario: config validation error を JSON で出力する

- **WHEN** user が invalid config を使って `kml check --format json --locale ja` を実行する
- **THEN** CLI JSON error は localized message を含む
- **THEN** CLI JSON error は stable message ID と message parameters を含む
- **THEN** exit code は existing config error behavior と同じく `2` になる
