## ADDED Requirements

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
