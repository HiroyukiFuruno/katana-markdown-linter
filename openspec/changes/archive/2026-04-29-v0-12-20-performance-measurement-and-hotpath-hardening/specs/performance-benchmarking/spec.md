## ADDED Requirements

### Requirement: v0.12.20 performance pass SHALL start from measured post-v0.12.19 evidence

`v0.12.20` の性能対応は、`v0.12.19` 適用後の測定結果から開始しなければならない（SHALL）。

#### Scenario: post-v0.12.19 baseline を測る

- **WHEN** developer が `v0.12.20` に着手する
- **THEN** system は `make bench` または `make perf-check` の結果を記録する
- **THEN** system は `api_lint_*`、`api_fix_*`、`api_rule_catalog`、`context_*`、`cli_*` のどこが変化したかを case 単位で分類する
- **THEN** system は測定結果なしに最適化を始めない

### Requirement: performance improvements SHALL be evidence-led and semantics-preserving

性能改善は、測定で示された hot path に限定し、lint / fix / fmt の意味を変えてはならない（SHALL NOT）。

#### Scenario: hot path を修正する

- **WHEN** developer が traversal、context、config validation、fix application のいずれかを変更する
- **THEN** system は変更前後の benchmark case を記録する
- **THEN** system は diagnostics、fixed content、formatter output、exit code の互換性を維持する
- **THEN** system は benchmark 数値だけを目的に rule semantics を弱めない

### Requirement: cross-tool timing SHALL remain optional evidence

cross-tool timing は有用な比較資料として扱い、required CI gate にしてはならない（SHALL NOT）。

#### Scenario: optional tool がない環境で測る

- **WHEN** developer が cross-tool benchmark を実行する
- **THEN** system は `mado`、`rumdl`、`hyperfine` がない場合に skipped として記録する
- **THEN** system は skipped を release-blocking failure として扱わない
- **THEN** system は required validation を `make` の通常 gate で完結させる
