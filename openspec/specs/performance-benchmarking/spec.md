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

### Requirement: system SHALL provide an opt-in cross-tool CLI benchmark

システムは、`kml` の CLI performance を `mado` および `rumdl` と比較できる opt-in benchmark を提供しなければならない（SHALL）。

#### Scenario: cross-tool benchmark を実行する

- **WHEN** developer が cross-tool benchmark target を実行する
- **THEN** system は `target/release/kml` を benchmark 対象として使用する
- **THEN** system は `mado` と `rumdl` が利用可能な場合に benchmark 対象として含める
- **THEN** system は missing optional tool を failure ではなく skipped case として report する
- **THEN** system は JSON report と Markdown summary を生成する
- **THEN** report は tool version、timing method、mode、enabled rule set、skipped reason を含む

### Requirement: system SHALL separate default and common-subset comparisons

システムは、各 tool の default behavior comparison と、共通 rule subset comparison を区別しなければならない（SHALL）。

#### Scenario: default comparison を実行する

- **WHEN** developer が default comparison を実行する
- **THEN** system は各 tool の default enabled rule set を使って check benchmark を実行する
- **THEN** report は mode を `default` として記録する
- **THEN** report は default mode が各 tool 固有の enabled rule set であることを limitation として記録する

#### Scenario: common-subset comparison を実行する

- **WHEN** developer が common-subset comparison を実行する
- **THEN** system は共通候補 rule subset 用の generated config を各 tool の format で作成する
- **THEN** system は `MD001,MD004,MD005,MD009,MD010,MD012,MD013,MD014,MD021,MD022,MD023,MD024,MD025,MD026,MD028,MD029,MD030,MD031,MD033,MD034,MD035,MD036,MD037,MD038,MD039,MD040,MD041,MD046,MD047` を common-subset candidate として扱う
- **THEN** report は mode を `common` として記録する
- **THEN** report は unsupported rule または option がある場合に detected limitation として記録する

### Requirement: system SHALL benchmark check and fix workflows safely

システムは、cross-tool benchmark で check workflow と fix workflow を安全に測定しなければならない（SHALL）。

#### Scenario: diagnostics-heavy check benchmark を実行する

- **WHEN** system が diagnostics-heavy corpus に対して check benchmark を実行する
- **THEN** system は expected violation exit code を successful measured run として normalize する
- **THEN** system は clean corpus に対する non-zero exit を failed case として report する

#### Scenario: fix benchmark を実行する

- **WHEN** system が fix benchmark を実行する
- **THEN** system は各 measured run ごとに mutable workspace copy を作成する
- **THEN** system は source corpus を直接変更しない
- **THEN** system は fix workflow を提供しない tool を skipped case として report する

### Requirement: system SHALL keep cross-tool benchmarking outside required CI

システムは、cross-tool benchmark を required CI gate から分離しなければならない（SHALL）。

#### Scenario: required CI を実行する

- **WHEN** GitHub Actions required CI または `make check` が実行される
- **THEN** system は `mado`、`rumdl`、または `hyperfine` の installation を required step としない
- **THEN** system は cross-tool benchmark を required merge gate として実行しない

### Requirement: fix application SHALL avoid repeated full-document offset scans

Fix application は、1回の `fix_with_results` 実行内で fix candidate ごとに文書全体を再走査してはならない（SHALL NOT）。

#### Scenario: diagnostics-heavy document を fix する

- **WHEN** system が多数の fix candidate を持つ document に `fix_with_results` を実行する
- **THEN** system は line start index を1回構築する
- **THEN** system は各 fix range をその index から byte range に変換する
- **THEN** system は fixed content と applied fix count を既存 behavior と一致させる

#### Scenario: Unicode と EOF range を処理する

- **WHEN** fix range が Unicode text または virtual EOF position を指す
- **THEN** system は UTF-8 boundary を壊さない
- **THEN** system は line `N + 1`, column `1` の EOF insertion behavior を維持する

### Requirement: v0.7.0 performance documentation SHALL record before and after

v0.7.0 performance work は、変更前後の計測結果と解釈を documentation に残さなければならない（SHALL）。

#### Scenario: performance change を完了する

- **WHEN** developer が v0.7.0 performance change を完了する
- **THEN** system は `make perf-check` または同等の local benchmark 結果を記録する
- **THEN** system は intentional baseline refresh を行う
- **THEN** system は benchmark numbers を correctness gate ではなく local snapshot として扱う
