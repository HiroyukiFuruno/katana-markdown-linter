## ADDED Requirements

### Requirement: system SHALL provide a check command for markdownlint parity linting

システムは、Markdown 文書の lint check を行う `check` command を提供しなければならない（SHALL）。

#### Scenario: 文書を check する

- **WHEN** user が `check` command を実行する
- **THEN** system は入力された file / glob を lint する
- **THEN** system は違反があれば非 zero exit code を返す

### Requirement: system SHALL provide a fix command for automatic correction

システムは、修正可能な違反を自動で直す `fix` command を提供しなければならない（SHALL）。

#### Scenario: 文書を fix する

- **WHEN** user が `fix` command を実行する
- **THEN** system は fixable rule の修正を適用する
- **THEN** system は修正後の結果を再 check できる状態にする

### Requirement: system SHALL provide a helper that creates .markdownlint.json

システムは、`.markdownlint.json` を作成する helper command を提供しなければならない（SHALL）。

#### Scenario: 初期設定を作る

- **WHEN** user が helper command を実行する
- **THEN** system は `.markdownlint.json` を出力する
- **THEN** system は公式 default に沿う初期設定を生成する

### Requirement: system SHALL honor explicit JSON and JSONC config selection for CLI execution

システムは、CLI 実行時に明示された `.markdownlint.json` または `.markdownlint.jsonc` config を優先しなければならない（SHALL）。

#### Scenario: 明示 config を使う

- **WHEN** user が `--config` を指定して command を実行する
- **THEN** system はその config を用いて lint / fix を行う
- **THEN** system は helper 生成物と競合しない

### Requirement: system SHALL use deterministic config discovery when config is omitted

システムは、CLI 実行時に config が明示されていない場合、決定的な順序で `.markdownlint.json` と `.markdownlint.jsonc` を探索しなければならない（SHALL）。

#### Scenario: config を自動探索する

- **WHEN** user が `--config` なしで command を実行する
- **THEN** system は current directory の `.markdownlint.json`、次に `.markdownlint.jsonc` を探索する
- **THEN** system は見つからない場合に parent directory を順に探索する
- **THEN** system は複数候補を非決定的に選ばない

### Requirement: system SHALL support JSON output for check and fix commands

システムは、`check` と `fix` command で machine-readable JSON output を提供しなければならない（SHALL）。

#### Scenario: JSON output を要求する

- **WHEN** user が `--format json` を指定して `check` または `fix` を実行する
- **THEN** system は diagnostics、applied fixes、errors を JSON として出力する
- **THEN** system は human-readable text と JSON を同じ stdout payload に混在させない
