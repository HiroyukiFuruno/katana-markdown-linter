# Requirement: v0.18.6 publication preconditions

### Requirement: manual publish prerequisites SHALL be explicit

#### Scenario: VS Code / Zed publish attempt

- **WHEN** 手動 publish を開始する。
- **THEN** account / publisher / package の3条件が揃っていなければ開始しない。

### Requirement: Neovim SHALL be docs-only

#### Scenario: editor scope review

- **WHEN** editor 対応範囲を確認する。
- **THEN** Neovim は docs-only のみ扱い、plugin 実装を新規追加しない。

### Requirement: stop reason SHALL be persisted

- **WHEN** publish precondition が不足する。
- **THEN** 停止理由を runbook と release 根拠として残す。
