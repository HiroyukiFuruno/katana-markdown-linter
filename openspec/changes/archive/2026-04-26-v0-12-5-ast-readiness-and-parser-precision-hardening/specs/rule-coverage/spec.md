## ADDED Requirements

### Requirement: link-family precision SHALL use shared parser evidence

link 系 rule の精度改善は、共有 parser または `DocumentContext` の token index に基づかなければならない（SHALL）。

#### Scenario: link 系 rule を評価する

- **WHEN** `MD034`、`MD051`、`MD052`、`MD054`、`MD059` が document を評価する
- **THEN** system は inline code、image、reference definition、autolink を通常本文と混同しない
- **THEN** system は rule ごとの独立した簡易 parser を増やさない
- **THEN** system は誤検知と検出漏れを rule-local test と document-level fixture の両方で固定する

### Requirement: parser migration SHALL document non-migrated rules

parser migration は、移譲しない rule の理由を明文化しなければならない（SHALL）。

#### Scenario: rule を移譲しない

- **WHEN** rule が `v0.12.5` で shared parser へ移譲されない
- **THEN** system は理由を `already-context-safe`、`line-local-by-spec`、`future-ast-required`、`blocked` のいずれかで記録する
- **THEN** system は future work が必要なものを後続 version の tasks に残す
