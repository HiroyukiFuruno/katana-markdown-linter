## Purpose

Performance benchmarking defines how this repository measures and reports
Markdown lint / fix hot paths without making CI depend on unstable wall-clock
thresholds.

## Requirements

### Requirement: system SHALL provide repeatable performance reports

システムは、Markdown lint / fix performance を比較できる repeatable report を生成しなければならない（SHALL）。

#### Scenario: benchmark report を生成する

- **WHEN** developer が `make bench` を実行する
- **THEN** system は release profile で benchmark runner を実行する
- **THEN** system は `target/perf-report.json` を生成する
- **THEN** report は case name、iterations、samples、total milliseconds、mean milliseconds、median milliseconds、min milliseconds、max milliseconds、standard deviation milliseconds を含む

### Requirement: system SHALL compare performance reports with a baseline

システムは、current performance report と committed baseline を比較できなければならない（SHALL）。

#### Scenario: performance baseline を確認する

- **WHEN** developer が `make perf-check` を実行する
- **THEN** system は `tests/fixtures/perf-baseline.json` を読み込む
- **THEN** system は current report に required benchmark case が存在することを検証する
- **THEN** system は required statistic fields が存在することを検証する
- **THEN** system は median milliseconds を中心とした差分 summary を出力する

### Requirement: system SHALL avoid duplicate lint evaluation in CLI fix mode

システムは、CLI fix mode で同じ content に対する不要な lint evaluation を避けなければならない（SHALL）。

#### Scenario: check --fix を実行する

- **WHEN** developer が `kml check --fix` または `kml fix` を実行する
- **THEN** system は fix 前 diagnostics を safe fix application に再利用する
- **THEN** system は fix 後 diagnostics を評価して remaining issues を報告する
- **THEN** system は従来と同じ fixed content、applied fix count、exit code を維持する

### Requirement: system SHALL benchmark representative API and CLI paths

システムは、API と CLI の代表的な performance path を benchmark report に含めなければならない（SHALL）。

#### Scenario: benchmark case coverage を確認する

- **WHEN** developer が `make bench` を実行する
- **THEN** report は diagnostics-heavy large document lint case を含む
- **THEN** report は clean large document lint case を含む
- **THEN** report は large document fix case を含む
- **THEN** report は many small document API lint case を含む
- **THEN** report は CLI directory check case を含む
- **THEN** report は config validation case を含む
- **THEN** report は rule catalog case を含む

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
