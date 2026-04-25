## Purpose

CLI の対話・非対話利用を満たし、Markdown linting を実行し、結果を機械可読に統合する。

## Requirements

### Requirement: CLI SHALL provide parity-oriented check and fix workflows

CLIは、単体利用に耐えるcheck/fix workflowを提供しなければならない（SHALL）。

#### Scenario: check と fix を実行する

- **WHEN** user が `kml check` を実行する
- **THEN** CLI は対象Markdownを再帰的にcheckする
- **WHEN** user が `kml check --fix` または `kml fmt` を実行する
- **THEN** CLI は安全なfixを適用し、残存違反を報告する

### Requirement: CLI SHALL support integration-friendly reporting

CLIは、CI、editor、pre-commit連携に適したreportingを提供しなければならない（SHALL）。

#### Scenario: machine-readable output を要求する

- **WHEN** user が `--output json` を指定する
- **THEN** CLI はsummary、files、diagnostics、errorsをJSONで出力する
- **THEN** CLI は既存の `--format json` を後方互換aliasとして扱う

### Requirement: CLI SHALL expose rule and configuration introspection

CLIは、ruleとconfigurationの状態を確認するsubcommandを提供しなければならない（SHALL）。

#### Scenario: rule と config を確認する

- **WHEN** user が `kml rule` または `kml rule MD013` を実行する
- **THEN** CLI はrule一覧またはrule詳細を表示する
- **WHEN** user が `kml config file` または `kml config get` を実行する
- **THEN** CLI は読み込まれるconfig情報を表示する

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
