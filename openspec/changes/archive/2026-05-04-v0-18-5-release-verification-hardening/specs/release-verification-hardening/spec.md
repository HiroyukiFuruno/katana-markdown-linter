# Requirement: v0.18.5 release verification hardening

### Requirement: partial publish SHALL be fail-fast

#### Scenario: partial publish attempt

- **WHEN** GitHub Release など一部だけが公開され、対象チャネルのいずれかが未完了。
- **THEN** `release-check` は停止し、`release-verify` の記録に失敗理由を残す。

### Requirement: verifier consistency SHALL be one-way

#### Scenario: verification logic drift

- **WHEN** `release-check` と `release-verify` の判断値が不一致。
- **THEN** release 実行を中断し、齟齬レポートを出す。

### Requirement: shared check keys SHALL be mandatory

#### Scenario: missing key

- **WHEN** `target/release-verify-state.json` が `version` / `editor_artifacts.vscode.state` / `editor_artifacts.zed.state` / `publish_blockers` / `release_decision` のいずれかを欠く。
- **THEN** release 系列は停止し、欠如キーを blocker として報告する。

### Requirement: external publish blockers SHALL be explicit

- **WHEN** external registry / wrapper / Homebrew / publish-stop 条件のいずれかが満たされない。
- **THEN** release 停止状態として扱い、再実行条件を明示する。
