## ADDED Requirements

### Requirement: v0.12.4 release readiness SHALL confirm core commands before distribution work resumes

`v0.12.4` の release readiness は、配布展開へ戻る前に中核コマンドの品質 gate を確認しなければならない（SHALL）。

#### Scenario: release 前 gate を確認する

- **WHEN** developer が `v0.12.4` release を準備する
- **THEN** system は `check` の no-write contract を確認する
- **THEN** system は `fix` と `check --fix` の safe fix contract を確認する
- **THEN** system は `fmt` の formatter contract と冪等性を確認する
- **THEN** system は performance evidence が command semantics を弱めていないことを確認する
- **THEN** system は release 後に `v0.13.0` 以降の配布展開へ戻れる状態にする
