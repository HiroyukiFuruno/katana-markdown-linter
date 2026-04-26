## ADDED Requirements

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
