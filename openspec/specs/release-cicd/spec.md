## Purpose

公開フローでの Release 実行条件、検証手順、artifact 公開範囲を明文化し、local gate と CI が同じ品質条件で安全に公開できる状態を保証する。

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

### Requirement: CI SHALL verify supported operating systems before release

システムは、release 前の CI で supported operating systems を検証しなければならない（SHALL）。

#### Scenario: Windows を含む CI matrix を実行する

- **WHEN** pull request または main push が CI 対象ファイルを変更する
- **THEN** system は Ubuntu、macOS、Windows の job を実行する
- **THEN** system は Windows 上で Rust workspace の build と test を検証する
- **THEN** system は Windows 固有の path、shell、binary suffix 差分で失敗した場合に release を止める

### Requirement: release workflow SHALL keep publish execution single-runner

システムは、crates.io publish を複数 OS job から実行してはならない（SHALL NOT）。

#### Scenario: release workflow を実行する

- **WHEN** release workflow が GitHub Release または crates.io publish を実行する
- **THEN** system は publish step を単一 runner で実行する
- **THEN** system は OS compatibility verification を release workflow の publish job ではなく、release 前 CI / preflight の責務として扱う

### Requirement: CI cache strategy SHALL be explicit and observable

システムは、CI cache strategy を明示し、cache hit/miss と安全境界を確認できなければならない（SHALL）。

#### Scenario: cache を利用する

- **WHEN** CI、release-preflight、release workflow が Rust build を実行する
- **THEN** system は cache key が OS、lockfile、toolchain、feature set の差分を安全に扱うことを保証する
- **THEN** system は stale artifact によって test が誤って通る状態を避ける
- **THEN** system は workflow log から cache hit/miss を確認できる

### Requirement: local and CI release gates SHALL remain aligned

local と CI の release gate は、意図しない乖離を起こしてはならない（SHALL NOT）。

#### Scenario: release gate を変更する

- **WHEN** developer が workflow の release-critical step を変更する
- **THEN** system は対応する Makefile target または AST lint の更新を要求する
- **THEN** system は local `make release-check` と CI release gate の差分を可視化する

### Requirement: release workflow SHALL build binary artifacts for supported targets

release workflow は、supported target ごとの `kml` binary artifact を build しなければならない（SHALL）。

#### Scenario: release builds target matrix

- **WHEN** release workflow が `vX.Y.Z` を公開する
- **THEN** system は `x86_64-unknown-linux-gnu`、`x86_64-apple-darwin`、`aarch64-apple-darwin`、`x86_64-pc-windows-msvc` の `kml` binary を build する
- **AND** system は target ごとの build failure を release failure として扱う
- **AND** system は publish job が成功した target だけを黙って公開する状態を作らない

### Requirement: release workflow SHALL upload binary artifacts with checksums

release workflow は、binary archive と checksum を GitHub Release に添付しなければならない（SHALL）。

#### Scenario: release uploads binary assets

- **WHEN** release workflow が GitHub Release を作成または更新する
- **THEN** system は target ごとの archive を release asset として upload する
- **AND** system は target ごとの `.sha256` file を release asset として upload する
- **AND** existing release を更新する場合、system は同じ名前の binary asset を安全に置き換える

### Requirement: release workflow SHALL reuse local packaging scripts

release workflow は、local release gate と同じ packaging script を使わなければならない（SHALL）。

#### Scenario: packaging script is changed

- **WHEN** developer が binary packaging logic を変更する
- **THEN** system は local `make release-check` から同じ script を実行する
- **AND** system は GitHub Actions 専用の未検証 shell logic だけで release artifact を作らない

### Requirement: release verification SHALL include binary assets

release verification は、tag、GitHub Release、crates.io に加えて binary asset の公開状態を確認しなければならない（SHALL）。

#### Scenario: release verify runs after publication

- **WHEN** developer が `make release-verify VERSION=vX.Y.Z` を実行する
- **THEN** system は GitHub Release に必要な binary archive が存在することを確認する
- **AND** system は各 archive の checksum file が存在することを確認する
- **AND** system は少なくとも current platform の archive を取得して `kml --version` を検証する

### Requirement: release workflow SHALL not publish wrappers without explicit enablement

release workflow は、npm / PyPI wrapper を明示的な enablement なしに publish してはならない（SHALL NOT）。

#### Scenario: wrapper trusted publishing is absent

- **WHEN** release workflow が wrapper publish step に到達する
- **THEN** system は publish enable flag と trusted publishing job を確認する
- **AND** enable flag がない場合、system は wrapper publish を skip する
- **AND** enable flag がある場合、system は長期 token ではなく trusted publishing で registry に publish する
- **AND** skip した wrapper を release note で公式公開済みとして扱わない
