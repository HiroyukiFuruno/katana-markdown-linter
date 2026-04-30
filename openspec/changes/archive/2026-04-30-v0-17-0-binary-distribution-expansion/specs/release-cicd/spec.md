# release-cicd Specification

## ADDED Requirements

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
