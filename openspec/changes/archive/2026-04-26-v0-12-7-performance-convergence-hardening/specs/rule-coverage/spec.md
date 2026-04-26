## ADDED Requirements

### Requirement: migrated rule corpus SHALL prove convergence

migrated rule corpus は、検出精度だけでなく fix / fmt の収束性を検証しなければならない（SHALL）。

#### Scenario: migrated corpus を検証する

- **WHEN** developer が migrated corpus tests を実行する
- **THEN** system は check diagnostics、fixed content、formatted content を別々に検証する
- **THEN** system は repeated fix で不要差分が増えないことを確認する
- **THEN** system は repeated fmt で不要差分が増えないことを確認する
