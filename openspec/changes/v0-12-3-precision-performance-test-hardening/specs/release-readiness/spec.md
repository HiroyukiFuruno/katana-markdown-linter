## ADDED Requirements

### Requirement: v0.12.3 release readiness SHALL include UT and IT expansion evidence

`v0.12.3` の公開準備（release readiness）は、単体テスト（UT）と結合テスト（IT）の拡充結果を含まなければならない（SHALL）。

#### Scenario: release 前に test evidence を確認する

- **WHEN** developer が `v0.12.3` の公開確認（release check）を実行する
- **THEN** system はルール単位の単体テスト（rule-local unit tests）と文書単位の結合テスト（document-level integration tests）の追加範囲を tasks に記録している
- **THEN** system は OS 固有の path / shell / binary suffix 差分が既存 CI で検証されていることを確認する
- **THEN** system は test helper の都合だけで production behavior を変更していないことを確認する

### Requirement: release gates SHALL keep performance checks informative but stable

公開ゲート（release gate）は速度確認（performance check）を有用な情報として扱い、環境依存の不安定な閾値に依存してはならない（SHALL NOT）。

#### Scenario: performance gate を実行する

- **WHEN** developer が `make perf-check` を実行する
- **THEN** system は基準値比較（baseline comparison）を表示する
- **THEN** system は required benchmark case と report schema を検証する
- **THEN** system は CI の時計時間ゆらぎ（wall-clock fluctuation）だけで release を止めない
