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
- **THEN** report は case name、iterations、total milliseconds、average milliseconds を含む

### Requirement: system SHALL compare performance reports with a baseline

システムは、current performance report と committed baseline を比較できなければならない（SHALL）。

#### Scenario: performance baseline を確認する

- **WHEN** developer が `make perf-check` を実行する
- **THEN** system は `tests/fixtures/perf-baseline.json` を読み込む
- **THEN** system は current report に required benchmark case が存在することを検証する
- **THEN** system は差分 summary を出力する

### Requirement: system SHALL avoid duplicate lint evaluation in CLI fix mode

システムは、CLI fix mode で同じ content に対する不要な lint evaluation を避けなければならない（SHALL）。

#### Scenario: check --fix を実行する

- **WHEN** developer が `kml check --fix` または `kml fix` を実行する
- **THEN** system は fix 前 diagnostics を safe fix application に再利用する
- **THEN** system は fix 後 diagnostics を評価して remaining issues を報告する
- **THEN** system は従来と同じ fixed content、applied fix count、exit code を維持する
