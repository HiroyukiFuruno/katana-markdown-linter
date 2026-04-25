## ADDED Requirements

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
