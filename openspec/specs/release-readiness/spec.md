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

### Requirement: v0.12.3 release readiness SHALL include UT and IT expansion evidence

`v0.12.3` の公開準備（release readiness）は、単体テスト（UT）と結合テスト（IT）の拡充結果を含まなければならない（SHALL）。

#### Scenario: release 前に test evidence を確認する

- **WHEN** developer が `v0.12.3` の公開確認（release check）を実行する
- **THEN** system はルール単位の単体テスト（rule-local unit tests）と文書単位の結合テスト（document-level integration tests）の追加範囲を tasks に記録している
- **THEN** system は OS 固有の path / shell / binary suffix 差分が既存 CI で検証されていることを確認する
- **THEN** system は test helper の都合だけで production behavior を変更していないことを確認する

### Requirement: release gates SHALL keep performance checks informative but stable

公開ゲート（release gate）は速度確認（performance check）を有用な情報として扱い、環境依存の不安定な閾値に依存してはならない（SHALL NOT）。

#### Scenario: performance gate を実行する

- **WHEN** developer が `make perf-check` を実行する
- **THEN** system は基準値比較（baseline comparison）を表示する
- **THEN** system は required benchmark case と report schema を検証する
- **THEN** system は CI の時計時間ゆらぎ（wall-clock fluctuation）だけで release を止めない
