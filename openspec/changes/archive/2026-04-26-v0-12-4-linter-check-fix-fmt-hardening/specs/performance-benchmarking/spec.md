## ADDED Requirements

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
