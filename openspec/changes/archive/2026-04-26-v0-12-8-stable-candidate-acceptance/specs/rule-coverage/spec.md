## ADDED Requirements

### Requirement: stable scoring SHALL include precision correctness

stable scoring は、rule precision を最大配点の評価対象にしなければならない（SHALL）。

#### Scenario: precision correctness を採点する

- **WHEN** developer が stable score を算出する
- **THEN** system は誤検知、検出漏れ、構文除外、fixture matrix、upstream golden の結果を 40 点満点で採点する
- **THEN** system は未分類の高優先度誤検知または検出漏れを hard blocker として扱う
- **THEN** system は accepted limitation を future work として記録する

### Requirement: stable scoring SHALL include safe command behavior

stable scoring は、safe command behavior を評価しなければならない（SHALL）。

#### Scenario: safe command behavior を採点する

- **WHEN** developer が stable score を算出する
- **THEN** system は `check` no-write、safe fix、fix/fmt idempotence、collision safety を 20 点満点で採点する
- **THEN** system は unsafe fix が default-safe fix に混ざる場合を hard blocker として扱う
- **THEN** system は repeated fix/fmt で不要差分が増える場合を hard blocker として扱う
