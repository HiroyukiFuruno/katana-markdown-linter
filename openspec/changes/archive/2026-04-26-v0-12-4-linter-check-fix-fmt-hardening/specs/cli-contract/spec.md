## ADDED Requirements

### Requirement: system SHALL define mutation behavior for check, fix, and format commands

システムは、`check`、`fix`、`fmt` command が file を変更するかどうかを明確に定義しなければならない（SHALL）。

#### Scenario: command mutation behavior を確認する

- **WHEN** user が `check` command を実行する
- **THEN** system は対象 file を変更しない
- **WHEN** user が `check --fix` または `fix` command を実行する
- **THEN** system は default-safe fix の範囲だけで対象 file を変更できる
- **WHEN** user が `fmt` command を実行する
- **THEN** system は formatter policy の範囲だけで対象 file を変更できる
- **THEN** system は `fmt` を lint fix の別名として扱わない

### Requirement: system SHALL expose stable command result categories

システムは、CLI result を diagnostics、applied fixes、formatted output、errors に分けて扱えるようにしなければならない（SHALL）。

#### Scenario: result category を JSON で確認する

- **WHEN** user が `--format json` または `--output json` を指定する
- **THEN** system は diagnostics と applied fixes を混同しない
- **THEN** system は formatter result を lint fix result と混同しない
- **THEN** system は errors を command result と区別して返す

### Requirement: system SHALL keep stdin and stdout behavior editor-friendly

システムは、editor integration が使える stdin / stdout behavior を維持しなければならない（SHALL）。

#### Scenario: stdin input を処理する

- **WHEN** user が stdin から Markdown を渡して `check`、`fix`、または `fmt` を実行する
- **THEN** system は file path 前提の処理に依存して失敗しない
- **THEN** system は stdout payload に command result を安定して返す
- **THEN** system は stderr を diagnostics payload の代替として扱わない
