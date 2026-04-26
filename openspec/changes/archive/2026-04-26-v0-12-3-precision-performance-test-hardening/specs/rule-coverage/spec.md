## ADDED Requirements

### Requirement: precision fixes SHALL cover false positives and false negatives

精度修正（precision fix）は誤検知と検出漏れの両方を扱わなければならない（SHALL）。

#### Scenario: precision issue を修正する

- **WHEN** developer が rule behavior を変更する
- **THEN** system は issue を `bug`、`test-gap`、`design-debt`、`perf-risk` のいずれかで分類する
- **THEN** system は最小再現のルール単位テスト（rule-local test）を追加する
- **THEN** system は必要に応じて文書単位fixture（document-level fixture）に複数構文が混ざった再発条件を追加する

### Requirement: context-sensitive rule tests SHALL include structural exclusion cases

context-sensitive rule test は、通常本文ではない構造の除外ケースを含まなければならない（SHALL）。

#### Scenario: structural exclusion を検証する

- **WHEN** rule が URL、link、table、heading、list、command prompt、emphasis に見える行を評価する
- **THEN** system は inline code、fenced code block、HTML block、table block、reference definition 内の期待値を検証する
- **THEN** system は unrelated diagnostics が増えていないことを確認する
- **THEN** system は fixture 追加だけで済むものと実装修正（implementation fix）が必要なものを区別する
