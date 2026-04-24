## Purpose

Release readiness defines the package metadata, validation gates, and install contract needed before publishing.

## Requirements

### Requirement: package metadata SHALL satisfy crates.io publishing requirements

システムは、crates.io へ publish できる package metadata を満たさなければならない（SHALL）。

#### Scenario: package を検証する

- **WHEN** developer が publish 前の package を作る
- **THEN** system は `license`、`readme`、`repository`、`description`、`keywords`、`categories` の metadata を含む
- **THEN** system は publish に不要なファイルを package に含めない

### Requirement: release validation SHALL run before publish

システムは、publish 前に package validation を実行しなければならない（SHALL）。

#### Scenario: dry-run を通す

- **WHEN** release pipeline が動く
- **THEN** system は `cargo package` か `cargo publish --dry-run` を実行する
- **THEN** system は失敗時に publish を止める

### Requirement: installed binary SHALL be usable with cargo install

システムは、ユーザーが `cargo install` で binary を導入できるようにしなければならない（SHALL）。

#### Scenario: binary を導入する

- **WHEN** user が crates.io から package を install する
- **THEN** system は `kml` executable target を提供する
- **THEN** user は PATH 上の `kml` コマンドとして呼び出せる

### Requirement: repository SHALL define quality gate responsibilities

repositoryは、localとCIで実行するquality gateの責務を明確に定義しなければならない（SHALL）。

#### Scenario: local gate を実行する

- **WHEN** developer が `make lint` を実行する
- **THEN** system は Clippy zero warning を検証する
- **WHEN** developer が `make ast-lint` を実行する
- **THEN** system は repository 固有の不変条件を検証する

### Requirement: repository SHALL protect release-critical invariants with AST lint

repositoryは、release と rule 互換性に関わる不変条件を AST lint で保護しなければならない（SHALL）。

#### Scenario: release-critical invariant を検証する

- **WHEN** developer が `make ast-lint` を実行する
- **THEN** system は fixture coverage、upstream drift、CLI traversal、signed release tag workflow を検証する
- **THEN** system は破壊的変更または未反映変更を失敗として報告する
