## MODIFIED Requirements

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

## ADDED Requirements

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
