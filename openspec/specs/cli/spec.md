## Purpose

CLI の対話・非対話利用を満たし、Markdown linting を実行し、結果を機械可読に統合する。

## Requirements

### Requirement: CLI SHALL provide parity-oriented check and fix workflows

CLIは、単体利用に耐えるcheck/fix workflowを提供しなければならない（SHALL）。

#### Scenario: check は診断だけを行う

- **WHEN** user が `kml check` を実行する
- **THEN** CLI は対象Markdownを再帰的にcheckする
- **THEN** CLI は対象ファイルを書き換えない
- **THEN** CLI は違反があれば残存診断を報告する

#### Scenario: check --fix は safe fix 後に再診断する

- **WHEN** user が `kml check --fix` を実行する
- **THEN** CLI は default-safe fix だけを適用する
- **THEN** CLI は fix 後の内容を再 check する
- **THEN** CLI は適用済み修正と残存違反を報告する
- **THEN** CLI は unsafe fix を暗黙に適用しない

#### Scenario: fix は明示的な safe fix command として動作する

- **WHEN** user が `kml fix` を実行する
- **THEN** CLI は default-safe fix だけを適用する
- **THEN** CLI は適用済み修正と残存違反を報告する
- **THEN** CLI は再実行時に不要な差分を増やさない

#### Scenario: fmt は formatter contract を使う

- **WHEN** user が `kml fmt` を実行する
- **THEN** CLI は lint fix ではなく formatter contract に従って整形する
- **THEN** CLI は整形後の内容が再実行で変化しないことを期待できる
- **THEN** CLI は lint 違反を直す目的で unsafe fix を適用しない

### Requirement: CLI SHALL keep check, fix, and fmt output contracts distinct

CLIは、`check`、`fix`、`fmt` の責務に応じて output contract を分離しなければならない（SHALL）。

#### Scenario: command-specific output を返す

- **WHEN** user が `kml check` を実行する
- **THEN** CLI は diagnostics を中心に出力する
- **WHEN** user が `kml fix` または `kml check --fix` を実行する
- **THEN** CLI は applied fixes と remaining diagnostics を区別して出力する
- **WHEN** user が `kml fmt` を実行する
- **THEN** CLI は formatter result を出力し、lint fix result と混同しない

#### Scenario: JSON output と text output を混在させない

- **WHEN** user が JSON output を指定する
- **THEN** CLI は command-specific result を JSON payload として出力する
- **THEN** CLI は human-readable progress や status を同じ stdout payload に混在させない

### Requirement: CLI SHALL support integration-friendly reporting

CLIは、CI、editor、pre-commit連携に適したreportingを提供しなければならない（SHALL）。

#### Scenario: machine-readable output を要求する

- **WHEN** user が `--output json` を指定する
- **THEN** CLI はsummary、files、diagnostics、errorsをJSONで出力する
- **THEN** CLI は既存の `--format json` を後方互換aliasとして扱う

### Requirement: CLI SHALL expose rule and configuration introspection

CLIは、ruleとconfigurationの状態を確認するsubcommandを提供しなければならない（SHALL）。

#### Scenario: rule と config を確認する

- **WHEN** user が `kml rule` または `kml rule MD013` を実行する
- **THEN** CLI はrule一覧またはrule詳細を表示する
- **WHEN** user が `kml config file` または `kml config get` を実行する
- **THEN** CLI は読み込まれるconfig情報を表示する

### Requirement: CLI rule introspection SHALL honor selected locale

CLI は、`kml rule` と `kml rule <id>` の rule description を selected locale で表示しなければならない（SHALL）。

#### Scenario: rule list を localized 表示する

- **WHEN** user が `kml rule --locale ja` を実行する
- **THEN** CLI は rule ID と rule name に加えて Japanese description を表示する
- **THEN** unsupported explicit locale は existing CLI locale policy に従って hard error になる

#### Scenario: rule JSON を localized 表示する

- **WHEN** user が `kml rule MD003 --locale ja --format json` を実行する
- **THEN** CLI JSON は selected locale を示す
- **THEN** CLI JSON は localized description を含む
- **THEN** CLI JSON は English canonical description を失わない

### Requirement: CLI config validation errors SHALL expose localized message metadata

CLI は config validation error を text と JSON の両方で localized metadata 付きで返さなければならない（SHALL）。

#### Scenario: config validation error を JSON で出力する

- **WHEN** user が invalid config を使って `kml check --format json --locale ja` を実行する
- **THEN** CLI JSON error は localized message を含む
- **THEN** CLI JSON error は stable message ID と message parameters を含む
- **THEN** exit code は existing config error behavior と同じく `2` になる
### Requirement: system SHALL separate CLI orchestration responsibilities with shared execution contracts

システムは、`check`、`fix`、`fmt` の主要パスを一貫した実行契約で扱い、`output` と exit code の契約を壊さない形で責務を分離しなければならない（SHALL）。

#### Scenario: CLI 実行契約を固定する

- **WHEN** developer が `kml check`、`kml fix`、`kml fmt` を実行する
- **THEN** システムは入力展開、設定読込、検証、診断集約、エラー整形を同じ契約で処理する
- **THEN** `--output json` の `files`、`summary`、`errors` 構造と exit code が変更されない
- **THEN** 既存契約は既存の `cli_path_context_contract` と一致する

### Requirement: system SHALL generate and version internal quality evidence

内部品質の可視化は、主要 hotspot の再発防止のための契約として固定されなければならない（SHALL）。

#### Scenario: 1回目の内部品質計測を保存する

- **WHEN** developer が `make internal-quality-check`（新規）を実行する
- **THEN** システムは `target/internal-quality-report.json` を出力する
- **THEN** レポートは `src` の上位 LOC ファイル、実行時長い hot path、主要 CLI ファイル分割候補を含める
- **THEN** report 形式が欠落していない場合のみ次工程へ進める

#### Scenario: 内部品質 evidence を比較する

- **WHEN** 開発者が同一 change の実装前後で internal quality evidence を比較する
- **THEN** 主要 hot path と大規模責務ファイルの再発性を追跡できることを確認する
- **THEN** 変更内容がない項目に対して、新たな回帰がある場合は `review` で検出できる

### Requirement: system SHALL keep internal refactor scoped by executable tests

内部リファクタは、実行可能テストで回帰防止した状態で進めなければならない（SHALL）。

#### Scenario: リファクタ前後の回帰を検知する

- **WHEN** internal refactor の変更を実施する
- **THEN** `cargo test --workspace --locked` は成功する
- **THEN** `cargo test --test cli_convergence_contract --locked` と `cargo test --test cli_path_context_contract --locked` が成功する
- **THEN** 同じ入力に対する CLI の診断総数、順序、exit code が再現される
