## ADDED Requirements

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
