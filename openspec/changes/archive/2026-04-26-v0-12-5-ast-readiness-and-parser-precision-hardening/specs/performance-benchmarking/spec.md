## ADDED Requirements

### Requirement: parser readiness SHALL be measured on syntax-heavy documents

parser readiness work は、構文密度の高い Markdown document で性能影響を測定しなければならない（SHALL）。

#### Scenario: syntax-heavy benchmark を実行する

- **WHEN** developer が `make perf-check` を実行する
- **THEN** report は link-heavy document case を含む
- **THEN** report は inline-code-heavy document case を含む
- **THEN** report は reference-heavy document case を含む
- **THEN** report は parser index construction case を含む
- **THEN** system は benchmark 数値だけを目的に rule semantics を弱めない
