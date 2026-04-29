## MODIFIED Requirements

### Requirement: CLI rule introspection SHALL honor selected locale

CLI は、`kml rule` と `kml rule <id>` の rule description を selected locale で表示しなければならない（SHALL）。

#### Scenario: rule list を localized 表示する

- **WHEN** user が `kml rule --locale <supported-locale>` を実行する
- **THEN** CLI は rule ID と rule name に加えて selected locale の description を表示する
- **THEN** unsupported explicit locale は existing CLI locale policy に従って hard error になる
- **THEN** supported locale の description は English description の単純コピーではない

#### Scenario: rule JSON を localized 表示する

- **WHEN** user が `kml rule MD003 --locale <supported-locale> --format json` を実行する
- **THEN** CLI JSON は selected locale を示す
- **THEN** CLI JSON は localized description を含む
- **THEN** CLI JSON は English canonical description を失わない
- **THEN** localized description は English canonical description の単純コピーではない
