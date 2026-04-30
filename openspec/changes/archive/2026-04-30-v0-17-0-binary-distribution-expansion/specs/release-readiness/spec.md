# release-readiness Specification

## ADDED Requirements

### Requirement: release readiness SHALL verify binary distribution artifacts

release readiness は、Cargo package だけでなく binary distribution artifact を検証しなければならない（SHALL）。

#### Scenario: release check validates binary artifacts

- **WHEN** developer が `make release-check VERSION=vX.Y.Z` を実行する
- **THEN** system は binary archive と checksum を local で作成する
- **AND** system は archive 展開後の `kml --version` を検証する
- **AND** system は小さな Markdown fixture に対して `kml check` を実行する

### Requirement: release readiness SHALL verify Homebrew formula output

release readiness は、Homebrew 定義ファイル（formula）の出力を検証しなければならない（SHALL）。

#### Scenario: release check validates formula

- **WHEN** developer が `make release-check VERSION=vX.Y.Z` を実行する
- **THEN** system は release archive URL と checksum を使って formula を生成する
- **AND** system は formula が `kml --version` を test block に含むことを検証する
- **AND** system は formula が存在しない asset や checksum を参照する場合に失敗する

### Requirement: release readiness SHALL record wrapper publication state

release readiness は、npm / pip wrapper の公開状態を明示しなければならない（SHALL）。

#### Scenario: wrapper publication is deferred

- **WHEN** package ownership または trusted publishing 設定が未確定である
- **THEN** tasks ledger は wrapper publication を deferred として記録する
- **AND** README と docs は wrapper を公式 install 導線として表示しない
- **AND** release note は wrapper source / smoke test の準備状態と publish 状態を分けて説明する

### Requirement: release readiness SHALL update public installation documentation accurately

release readiness は、公開済みの install 導線だけを README と docs に記載しなければならない（SHALL）。

#### Scenario: docs are updated for release

- **WHEN** release preparation updates README or docs
- **THEN** documentation は Cargo、GitHub Release binary、Homebrew、公開済み wrapper の状態を分けて説明する
- **AND** documentation は未公開 wrapper を公式導線として扱わない
- **AND** documentation 変更後に `make ast-lint` が成功する

### Requirement: release readiness SHALL preserve existing crate install contract

release readiness は、Cargo install の既存導入契約を壊してはならない（SHALL NOT）。

#### Scenario: binary distribution is added

- **WHEN** system が standalone binary と Homebrew support を追加する
- **THEN** `cargo install katana-markdown-linter` で `kml` を導入できる状態を維持する
- **AND** existing GitHub Action install-source behavior を維持する
- **AND** binary artifact support のために crate metadata を弱めない
