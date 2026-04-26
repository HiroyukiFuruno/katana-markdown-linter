## ADDED Requirements

### Requirement: context-sensitive migration SHALL cover rule families

context-sensitive migration は、単一 rule ではなく rule family 単位で coverage を固定しなければならない（SHALL）。

#### Scenario: rule family を検証する

- **WHEN** developer が migration tests を実行する
- **THEN** system は link/reference、inline content、block structure、table/list/heading/fence の fixture を検証する
- **THEN** system は false positive と false negative の両方を検証する
- **THEN** system は unrelated diagnostics が増えていないことを確認する

### Requirement: safe fixes SHALL preserve source-range intent after migration

safe fix を持つ rule は、migration 後も source range の意図を維持しなければならない（SHALL）。

#### Scenario: migrated fix を適用する

- **WHEN** system が migrated rule の fix を適用する
- **THEN** system は original source range に基づいて replacement を適用する
- **THEN** system は overlapping fix と adjacent fix の競合を検出または安全に解決する
- **THEN** system は unsafe fix を default-safe fix と混同しない
