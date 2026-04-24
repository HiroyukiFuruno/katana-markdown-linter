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
