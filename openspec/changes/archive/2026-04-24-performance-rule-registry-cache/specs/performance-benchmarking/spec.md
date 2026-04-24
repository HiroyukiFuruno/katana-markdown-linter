## ADDED Requirements

### Requirement: system SHALL cache rule metadata on metadata hot paths

システムは、CLI config validation と rule catalog construction で rule metadata の不要な再構築を避けなければならない（SHALL）。

#### Scenario: cached metadata registry を利用する

- **WHEN** developer が rule metadata API または `kml check` の config validation を実行する
- **THEN** system は official rule metadata registry を process 内で再利用する
- **THEN** system は user-configurable rule metadata registry を process 内で再利用する
- **THEN** system は public owned rule vector API の互換性を維持する
- **THEN** system は lint diagnostics と runtime dispatch behavior を従来と同じに保つ

#### Scenario: catalog construction を再利用する

- **WHEN** developer が `available_rules`、`implemented_rules`、`missing_rules`、または `rule_catalog` を実行する
- **THEN** system は cached rule catalog source を再利用する
- **THEN** system は caller に owned result を返し、caller mutation が cache を破壊しないようにする

#### Scenario: config validation を再利用する

- **WHEN** CLI が file ごとの effective config を validate する
- **THEN** system は cached user-configurable metadata registry を使って validation を実行する
- **THEN** system は validation result と CLI behavior を従来と同じに保つ
