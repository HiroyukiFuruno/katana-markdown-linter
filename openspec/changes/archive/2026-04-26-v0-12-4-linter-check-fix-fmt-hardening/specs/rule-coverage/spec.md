## ADDED Requirements

### Requirement: linter hardening SHALL prioritize existing rule correctness before distribution expansion

システムは、配布経路の拡大より前に既存 rule の正しさを優先して固めなければならない（SHALL）。

#### Scenario: core rule issue を棚卸しする

- **WHEN** developer が `v0.12.4` の実装に着手する
- **THEN** system は既存 rule の誤検知、検出漏れ、fix 事故、test gap を分類する
- **THEN** system は patch release で扱う対象と後続 change に送る対象を区別する
- **THEN** system は fixture 追加だけで済むものと実装修正が必要なものを区別する

### Requirement: safe fix behavior SHALL converge and remain idempotent

safe fix behavior は収束し、再実行時に不要な差分を増やしてはならない（SHALL）。

#### Scenario: safe fix を再実行する

- **WHEN** system が `check --fix` または `fix` を同じ content に複数回実行する
- **THEN** system は初回で適用できる default-safe fix を適用する
- **THEN** system は再実行時に同じ修正を繰り返し適用しない
- **THEN** system は残存違反を diagnostics として報告し続ける
- **THEN** system は unsafe fix を default-safe fix と混同しない

### Requirement: check, fix, and fmt corpus SHALL include mixed Markdown structures

`check`、`fix`、`fmt` の corpus は、複数構文が混ざる Markdown 文書を含まなければならない（SHALL）。

#### Scenario: mixed corpus を検証する

- **WHEN** developer が corpus tests を実行する
- **THEN** system は fenced code block、inline code、HTML block、table、nested list、reference link を含む fixture を検証する
- **THEN** system は check diagnostics、fixed content、formatted content を別々に検証する
- **THEN** system は unrelated diagnostics や不要な formatting diff が増えていないことを確認する
