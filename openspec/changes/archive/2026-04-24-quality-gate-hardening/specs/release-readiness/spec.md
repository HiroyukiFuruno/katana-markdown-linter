## ADDED Requirements

### Requirement: repository SHALL define quality gate responsibilities

repositoryは、localとCIで実行するquality gateの責務を明確に定義しなければならない（SHALL）。

#### Scenario: local gate を実行する

- **WHEN** developer が `make lint` を実行する
- **THEN** system はClippy zero warningを検証する
- **WHEN** developer が `make ast-lint` を実行する
- **THEN** system はrepository固有の不変条件を検証する

### Requirement: repository SHALL protect release-critical invariants with AST lint

repositoryは、releaseとrule互換性に関わる不変条件をAST lintで保護しなければならない（SHALL）。

#### Scenario: release-critical invariant を検証する

- **WHEN** developer が `make ast-lint` を実行する
- **THEN** system はfixture coverage、upstream drift、CLI traversal、signed release tag workflowを検査する
- **THEN** system は破壊的変更または未反映変更を失敗として報告する
