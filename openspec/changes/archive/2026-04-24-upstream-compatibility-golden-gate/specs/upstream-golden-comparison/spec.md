## ADDED Requirements

### Requirement: system SHALL compare kml diagnostics with upstream markdownlint diagnostics

システムは、同一 fixture input に対する `kml` と upstream `markdownlint` の diagnostics を比較しなければならない（SHALL）。

#### Scenario: diagnostics を比較する

- **WHEN** developer が upstream golden comparison を実行する
- **THEN** system は upstream `markdownlint` と `kml` を同じ Markdown input に対して実行する
- **THEN** system は rule id、line、column、range、fixability を正規化して比較する
- **THEN** system は unknown delta を failure として報告する

### Requirement: system SHALL compare fix output when both implementations can fix a rule

システムは、両実装で fix 可能な rule について修正結果を比較しなければならない（SHALL）。

#### Scenario: fix output を比較する

- **WHEN** fixture が fixable rule を対象にしている
- **THEN** system は upstream と `kml` の fixed output を比較する
- **THEN** system は意味のある差分を diagnostics として報告する
- **THEN** system は fix 非対応 rule を fix failure と混同しない

### Requirement: system SHALL separate deterministic golden checks from live upstream update checks

システムは、再現性のある golden check と live upstream 追従 check を分離しなければならない（SHALL）。

#### Scenario: deterministic gate を実行する

- **WHEN** CI が通常の golden gate を実行する
- **THEN** system は lock された upstream oracle または固定された実行環境を使用する
- **THEN** system は network access を必須条件にしない

#### Scenario: live upstream を確認する

- **WHEN** developer が live upstream update target を実行する
- **THEN** system は upstream default branch または明示 source と local golden baseline を比較する
- **THEN** system は追加対応が必要な差分を report する

### Requirement: system SHALL manage known upstream compatibility deltas explicitly

システムは、既知の upstream compatibility delta を構造化して管理しなければならない（SHALL）。

#### Scenario: known delta を適用する

- **WHEN** comparison harness が差分を検出する
- **THEN** system は known delta file と照合する
- **THEN** system は rule id、fixture、reason、解消条件が一致する差分のみを許容する
- **THEN** system は未登録差分を unknown delta として失敗させる
