## ADDED Requirements

### Requirement: v0.12.20 release readiness SHALL prove performance changes preserve correctness

`v0.12.20` の release 前確認は、性能変更が正しさを壊していないことを証明しなければならない（SHALL）。

#### Scenario: performance release gate を通す

- **WHEN** developer が `v0.12.20` の release check を行う
- **THEN** system は `make perf-check` または `make perf-check-strict` の結果を記録する
- **THEN** system は `make test`、`make ast-lint`、`make dogfood` の結果を記録する
- **THEN** system は説明不能な重大性能退行がある場合、`v0.12.21` へ進まない

### Requirement: v0.12.20 SHALL prepare the handoff to KatanA feedback sweep

`v0.12.20` は、次の `v0.12.21` で KatanA 側ドキュメントを使った feedback sweep を行える状態で完了しなければならない（SHALL）。

#### Scenario: v0.12.21 へ引き継ぐ

- **WHEN** `v0.12.20` の validation が完了する
- **THEN** system は性能上の known limitation と non-blocking follow-up を tasks または release evidence に残す
- **THEN** system は KatanA feedback sweep で確認すべき観点を `v0.12.21` の tasks に引き継ぐ
- **THEN** system は未分類の performance blocker を残したまま closeout に進まない
