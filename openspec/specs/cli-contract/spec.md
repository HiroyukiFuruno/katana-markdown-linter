## Purpose

`check` / `fix` / `format` を含むCLI契約を定義し、入力対象・再帰走査・設定解決・出力形式の期待値を明確にする。

## Requirements

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

### Requirement: core command workflows SHALL remain convergent after parser migration

parser migration 後も、core command workflow は収束しなければならない（SHALL）。

#### Scenario: core command workflow を再実行する

- **WHEN** system が `check`、`check --fix`、`fix`、`fmt`、`check`、`fmt` を同じ corpus に順番に実行する
- **THEN** `check` は入力を書き換えない
- **THEN** `check --fix` と `fix` は default-safe fix だけを適用する
- **THEN** `fmt` は formatter policy の範囲だけを変更する
- **THEN** 再実行で同じ変更を繰り返さない
- **THEN** stdout JSON shape と exit code contract は維持される

### Requirement: directory traversal SHALL exclude reserved directories by default

CLI の directory traversal は、通常 git 管理しない予約領域 directory を既定で対象外にしなければならない（SHALL）。

#### Scenario: 既定の directory traversal を実行する

- **WHEN** user が `kml check`、`kml fix`、または `kml fmt` に directory path を渡す
- **THEN** system は `.git`、`node_modules`、dependency cache、build output、coverage output のような広く共通する予約領域 directory へ既定では入らない
- **THEN** system は予約領域配下の Markdown file を既定では診断せず、書き換えない
- **THEN** system は `.gitignore` がなくても同じ既定除外を適用する

### Requirement: reserved directory traversal SHALL require explicit opt-in

予約領域 directory を lint / fix / fmt 対象に戻す場合、user の明示 opt-in が必要である（SHALL）。

#### Scenario: 予約領域を明示的に対象にする

- **WHEN** user が予約領域を含める CLI option を指定して directory path を渡す
- **THEN** system は予約領域配下の Markdown file も対象候補に含める
- **THEN** `check`、`fix`、`fmt` は同じ opt-in 意味を共有する
- **THEN** usage documentation は既定除外と opt-in の意味を確認できる

### Requirement: explicit ignored directory traversal SHALL require scoped opt-in

`.gitignore` で除外された directory を明示的に対象へ戻す場合、走査全体ではなく明示 input 配下だけに効く opt-in を提供しなければならない（SHALL）。

#### Scenario: gitignore 済み directory を明示的に fix する

- **WHEN** user が `.gitignore` で除外された directory path を明示し、ignored path opt-in を指定して `kml fix` を実行する
- **THEN** system はその明示 directory 配下の Markdown file を対象に含める
- **THEN** system は他の ignored directory を広く対象に戻さない
- **THEN** user は `--no-ignore` より狭い範囲で ignored directory を修正できる

### Requirement: fix JSON SHALL expose applied fix details

`fix --output json` と `check --fix --output json` は、差分評価で rule と変更内容を突き合わせられるように、file ごとの適用 fix 詳細を出力しなければならない（SHALL）。

#### Scenario: safe fix が適用される

- **WHEN** user が JSON output で safe fix を実行する
- **THEN** system は file report に `fix_details` を含める
- **THEN** 各 fix detail は少なくとも `rule_id`、適用 range、replacement、`applied` を含める
- **THEN** 事後評価者は事前 diagnostic、fix result、git diff を file / hunk 単位で対応付けられる
