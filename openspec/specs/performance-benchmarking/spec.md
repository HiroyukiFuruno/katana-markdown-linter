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

### Requirement: system SHALL classify and track performance regressions with per-case context

性能は改善/退行を `case` 単位で記録し、速度差の背景を追えるように固定しなければならない（SHALL）。

#### Scenario: required cases を比較する

- **WHEN** developer が `make perf-check` を実行する
- **THEN** system は required cases の現行値と baseline 比較を必ず出力する
- **THEN** report/ baseline に含まれない case がないことを確認する
- **THEN** 現行レポートは必要項目（iterations/samples/median_ms 等）を必ず保持する

### Requirement: system SHALL detect non-acceptable regressions in strict mode

精度を維持したまま性能退行だけを制御するため、strict モードでは閾値ベースで検知する（SHALL）。

#### Scenario: strict パフォーマンスチェックを実行する

- **WHEN** `scripts/ci/perf-check.py` が `--strict` で実行される
- **THEN** 主要 case の median 比率がしきい値を超えたときに失敗する
- **THEN** 失敗時はどの case が原因かを出力する
- **THEN** しきい値と実行結果は evidence として残す

### Requirement: system SHALL preserve correctness before any speed optimization

速度改善は、検知/修正の正しさと収束性を損なってはならない（SHALL NOT）。

#### Scenario: 最適化前提を確認する

- **WHEN** 性能改善候補を実装する
- **THEN** precision 関連の既存 contract テスト（`rule_fixture_harness` / `upstream_golden` / `public confidence`）が通る
- **THEN** `check`、`fix`、`fmt` の収束証跡（2回実行）を確認する
- **THEN** 収束確認が壊れる場合は性能変更を見直す

### Requirement: system SHALL add a repeatable inline-code-heavy regression sentinel

以前に大きな退行が見えた inline-code-heavy 系は、継続監視対象として固定されなければならない（SHALL）。

#### Scenario: inline-code-heavy sentinel を測る

- **WHEN** baseline を更新または変更前後で `make perf-check` が実行される
- **THEN** `api_lint_inline_code_heavy_document` の median 比率が記録される
- **THEN** 退行理由が説明できる場合のみ次の baseline refresh を許可する
- **THEN** 説明不能な 2 倍近い退行は hard blocker 扱いする

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

### Requirement: public confidence performance SHALL compare synthetic and real-document corpora

公開前 confidence の性能確認は、既存の synthetic benchmark と実文書寄り corpus の差分を分けて説明しなければならない（SHALL）。

#### Scenario: public confidence performance を確認する

- **WHEN** developer が `v0.12.9` の performance check を実行する
- **THEN** system は既存 `make perf-check` の benchmark result を記録する
- **THEN** system は KatanA corpus または curated public confidence fixture の check / fix / fmt timing を記録する
- **THEN** system は regression が synthetic corpus 由来か real-document corpus 由来かを分けて説明する
- **THEN** system は benchmark 数値だけを目的に rule semantics を弱めない

### Requirement: external corpus performance blockers SHALL stop distribution planning

外部 corpus で説明不能な重大性能劣化がある場合、配布計画へ進んではならない（SHALL NOT）。

#### Scenario: performance blocker を確認する

- **WHEN** external corpus timing に説明不能な重大 regression がある
- **THEN** system は finding を `perf-regression` として分類する
- **THEN** system は release blocker として tasks に記録する
- **THEN** system は `v0.13.0` の配布計画へ進まない

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

### Requirement: v0.12.3 performance work SHALL start from measured baseline

`v0.12.3` の速度改善（performance work）は、実装前の測定基準値（baseline）から開始しなければならない（SHALL）。

#### Scenario: hot path を選ぶ

- **WHEN** developer が performance 改善に着手する
- **THEN** system は `make bench` または `make perf-check` の結果を記録する
- **THEN** system は対象の高負荷経路（hot path）と改善仮説を tasks に残す
- **THEN** system は測定不能な体感速度だけを根拠に実装しない

### Requirement: performance improvements SHALL preserve lint and fix correctness

速度改善（performance improvement）は lint / fix の正しさ（correctness）を変えてはならない（SHALL NOT）。

#### Scenario: hot path を最適化する

- **WHEN** developer が traversal、config、context、rule dispatch、reporting、fix application を変更する
- **THEN** system は既存 diagnostics、fixed content、exit code、JSON shape を維持する
- **THEN** system は正しさのテスト（correctness test）を通してから基準値更新（baseline refresh）を行う
- **THEN** system は benchmark 数値だけを目的に rule semantics を変更しない

### Requirement: v0.12.4 performance evidence SHALL include check, fix, and fmt paths

`v0.12.4` の performance evidence は、`check`、`fix`、`fmt` の代表経路を含めなければならない（SHALL）。

#### Scenario: core command performance を測定する

- **WHEN** developer が `v0.12.4` の performance check を実行する
- **THEN** system は directory check path を測定対象に含める
- **THEN** system は diagnostics-heavy lint path を測定対象に含める
- **THEN** system は safe fix path を測定対象に含める
- **THEN** system は formatter path を測定対象に含める
- **THEN** system は測定前後の解釈を release evidence として残す

### Requirement: performance improvements SHALL preserve check, fix, and fmt semantics

速度改善（performance improvement）は、`check`、`fix`、`fmt` の意味を変えてはならない（SHALL NOT）。

#### Scenario: core command hot path を最適化する

- **WHEN** developer が traversal、config resolution、rule dispatch、fix application、formatter pass を変更する
- **THEN** system は diagnostics を維持する
- **THEN** system は fixed content を維持する
- **THEN** system は formatted content を維持する
- **THEN** system は exit code と JSON shape を維持する
- **THEN** system は benchmark 数値だけを目的に command contract を弱めない

### Requirement: parser readiness SHALL be measured on syntax-heavy documents

parser readiness work は、構文密度の高い Markdown document で性能影響を測定しなければならない（SHALL）。

#### Scenario: syntax-heavy benchmark を実行する

- **WHEN** developer が `make perf-check` を実行する
- **THEN** report は link-heavy document case を含む
- **THEN** report は inline-code-heavy document case を含む
- **THEN** report は reference-heavy document case を含む
- **THEN** report は parser index construction case を含む
- **THEN** system は benchmark 数値だけを目的に rule semantics を弱めない

### Requirement: post-migration performance SHALL be explained before stable scoring

parser / context migration 後の performance は、安定版スコアリング前に説明されなければならない（SHALL）。

#### Scenario: post-migration performance を確認する

- **WHEN** developer が `v0.12.7` の performance check を実行する
- **THEN** system は parser index construction、API lint、API fix、CLI check、CLI fix、CLI fmt の代表 case を比較する
- **THEN** system は unexplained regression を release evidence として記録する
- **THEN** system は benchmark 数値だけを目的に rule semantics を弱めない

### Requirement: baseline refresh SHALL follow correctness gates

baseline refresh は正しさの gate 後にだけ行わなければならない（SHALL）。

#### Scenario: baseline を更新する

- **WHEN** developer が performance baseline を更新する
- **THEN** system は precision fixture、fix/fmt convergence、dogfood、release-check が成功していることを確認する
- **THEN** system は refresh 対象 case、before / after、理由を tasks に記録する

### Requirement: stable scoring SHALL include performance stability

stable scoring は、performance stability を評価しなければならない（SHALL）。

#### Scenario: performance stability を採点する

- **WHEN** developer が stable score を算出する
- **THEN** system は parser / context migration 後の benchmark を 20 点満点で採点する
- **THEN** system は link-heavy、inline-code-heavy、reference-heavy、parser index construction の結果を含める
- **THEN** system は説明不能な重大 regression を hard blocker として扱う
- **THEN** system は benchmark 数値だけを目的に rule semantics を弱めない
