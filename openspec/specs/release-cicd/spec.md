## Purpose

公開フローでの Release 実行条件と手順を明文化し、安全な公開を保証する。

## Requirements

### Requirement: release workflow SHALL validate Cargo package version before publishing

システムは、release 前に `Cargo.toml` の package version と release version が一致することを検証しなければならない（SHALL）。

#### Scenario: version を検証する

- **WHEN** release workflow が `vX.Y.Z` tag または manual input を受け取る
- **THEN** system は `Cargo.toml` の `package.version` と比較する
- **THEN** system は不一致の場合 release を停止する

### Requirement: release workflow SHALL run quality gates before release creation

システムは、GitHub Release 作成前に品質ゲートを実行しなければならない（SHALL）。

#### Scenario: release gate を実行する

- **WHEN** release workflow が実行される
- **THEN** system は format、test、clippy、upstream drift、package dry-run、install smoke test を実行する
- **THEN** system は失敗した gate がある場合 release を作成しない

### Requirement: release workflow SHALL create GitHub Release artifacts

システムは、GitHub Release に version tag と release artifact を作成しなければならない（SHALL）。

#### Scenario: GitHub Release を作成する

- **WHEN** release gate が全て成功する
- **THEN** system は `vX.Y.Z` tag を作成または利用する
- **THEN** system は GitHub Release を作成または更新する
- **THEN** system は `.crate` package と checksum を添付する

### Requirement: release workflow SHALL publish to crates.io only when explicitly enabled

システムは、crates.io publish を明示的に有効化した場合にのみ実行しなければならない（SHALL）。

#### Scenario: crate を publish する

- **WHEN** release workflow が publish enabled で実行される
- **THEN** system は `CARGO_REGISTRY_TOKEN` が存在することを確認する
- **THEN** system は `cargo publish` を実行する
- **THEN** system は token がない場合に明示的なエラーで停止する
