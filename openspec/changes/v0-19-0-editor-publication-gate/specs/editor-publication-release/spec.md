### Requirement: release versioning policy SHALL prevent no-op bump and failed publish drift

- `v0.18.7` のような既存公開済み version がある場合、同一版の再 publish 指定は実行前に停止する。
- `v0.18.x` patch は bugfix ベースでないとリリース対象外とする。

#### Scenario: existing version on one or more external channels

- **WHEN** `v0.18.7` のように既に外部チャネルで公開済み version を release-queue に設定する。
- **THEN** `verify-release-target` が失敗し、`release` / `release-github` の実行を停止する。
- **AND** メッセージは次 version を要求する内容であること。

### Requirement: v0.19.0 SHALL be release-justified

- `v0.19.0` は、VS Code / Zed marketplace 公開の実行面が実装に入る場合のみ許可する。
- 公開実行を伴わない gate や書類整備のみではバンプ不可。

#### Scenario: no publication change

- **WHEN** 0.18 系 bugfix と release guard の改善のみで終了する。
- **THEN** 次 version は `0.18.x` のままでよく、`v0.19.0` には進まない。

### Requirement: marketplace publication SHALL be gated by account/package verification

- account / publisher / package verification が未設定なら公開系 job は実行しない。

#### Scenario: publication requested but guard missing

- **WHEN** marketplace publish を実行しようとする。
- **THEN** system は明確な失敗理由を示し、実行を停止する。
- **AND** runbook は `deferred` での扱いを記録する。

### Requirement: release-readiness SHALL run self dogfood with kml

`release-check` 前提として、対象 branch は `kml` 自身に対して自己 dogfood を必須通過すること（SHALL）。

#### Scenario: self repository dogfood

- **WHEN** release-readiness を実施する前に `just dogfood` を実行する。
- **THEN** system は `README.md docs openspec` を対象として `target/dogfood-report.json` を生成する。
- **AND** `v0.18.7` 後の変更では、意図しない新規 warning / baseline 増分がないことを確認する。
- **AND** 意図的な差分がある場合は、`dogfood` baseline 更新手順を別途 evidence として保持する。

### Requirement: release-readiness SHALL define DoR / DoD

release 管理 change は、`v0.19.0` 前提を明確化するため、DoR と DoD を実行可能なチェック項目で持つこと（SHALL）。

#### Scenario: DoR / DoD are explicit and checkable

- **WHEN** release 準備を始める。
- **THEN** system は DoR に沿って、`v0.18.7` 再公開不可、既存版検知、marketplace 前提条件、dogfood 前提が満たされていることを確認する。
- **AND** system は DoD に沿って、`release-check` / `release-verify` / dogfood の結果を runbook に紐付け、`v0.19.0` 進行可否を一意に判定できることを示す。

### Requirement: change documents SHALL be internally consistent

proposal / design / tasks / spec の主要判断軸が同一条件で一致していること。

#### Scenario: per-change document consistency check

- **WHEN** change 実装前に `v0.19.0 Editor Publication Gate` の4文書を突合する。
- **THEN** `再公開不可（`v0.18.7`）`, `v0.19.0 Go/No-Go`, `dogfood 必須` の3軸が全文書で一致し、欠落・矛盾がないことを確認する。
- **AND** 矛盾がある場合は DoD を成立させない。
