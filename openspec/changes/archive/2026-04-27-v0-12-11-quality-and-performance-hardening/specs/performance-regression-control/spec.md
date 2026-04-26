## ADDED Requirements

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
