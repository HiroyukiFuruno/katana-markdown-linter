# binary-distribution Specification

## ADDED Requirements

### Requirement: Release SHALL publish standalone kml binary archives

システムは、`kml` の単体バイナリ配布物（standalone binary artifact）を GitHub Release に添付しなければならない（SHALL）。

#### Scenario: release uploads binary archives

- **WHEN** release workflow が `vX.Y.Z` を公開する
- **THEN** system は supported target ごとの `kml` binary archive を作成する
- **AND** system は各 archive を GitHub Release に添付する
- **AND** archive は `kml` executable、`LICENSE`、短い install note を含む

### Requirement: Binary archive naming SHALL be stable

システムは、package manager と wrapper が参照できる安定した archive 名を使わなければならない（SHALL）。

#### Scenario: archive names are generated

- **WHEN** system が `vX.Y.Z` の binary archive を作成する
- **THEN** Unix archive 名は `katana-markdown-linter-vX.Y.Z-<target>.tar.gz` になる
- **AND** Windows archive 名は `katana-markdown-linter-vX.Y.Z-<target>.zip` になる
- **AND** `<target>` は Rust target triple と一致する

### Requirement: Binary archives SHALL have checksum files

システムは、binary archive ごとに SHA-256 checksum file を作らなければならない（SHALL）。

#### Scenario: checksum files are generated

- **WHEN** system が binary archive を作成する
- **THEN** system は `<archive>.sha256` を作成する
- **AND** checksum file は対応する archive の SHA-256 を含む
- **AND** release verification は archive と checksum file の対応を検証する

### Requirement: Binary archives SHALL be smoke-tested after extraction

システムは、archive 展開後の `kml` binary を実行して検証しなければならない（SHALL）。

#### Scenario: archive smoke test runs kml

- **WHEN** system が binary archive を展開する
- **THEN** system は展開された `kml` または `kml.exe` を実行する
- **AND** system は `kml --version` が release version を返すことを検証する
- **AND** system は小さな Markdown fixture に対して `kml check` が実行できることを検証する

### Requirement: Homebrew formula SHALL use verified release artifacts

Homebrew 定義ファイル（formula）は、検証済みの GitHub Release artifact と checksum を参照しなければならない（SHALL）。

#### Scenario: formula is generated

- **WHEN** system が `vX.Y.Z` 用の Homebrew formula を生成する
- **THEN** formula は対象 platform の release archive URL を参照する
- **AND** formula は対応する SHA-256 checksum を含む
- **AND** formula の test block は `kml --version` を実行する

### Requirement: Homebrew publication SHALL be separated from unverified tap mutation

システムは、Homebrew tap repository を未検証のまま直接変更してはならない（SHALL NOT）。

#### Scenario: tap update is prepared

- **WHEN** system が Homebrew 更新を準備する
- **THEN** system は formula 生成結果と checksum を検証する
- **AND** system は tap repository への変更を review 可能な差分として作る
- **AND** system は検証前に remote tap branch へ push しない

### Requirement: npm and pip wrappers SHALL be thin launchers

npm / pip の薄いラッパー（wrapper）は、`kml` の独自実装を持ってはならない（SHALL NOT）。

#### Scenario: wrapper installs kml

- **WHEN** user が npm または pip wrapper を install する
- **THEN** wrapper は platform に対応する公式 binary archive を取得する
- **AND** wrapper は checksum を検証する
- **AND** wrapper は取得した `kml` binary を実行する
- **AND** wrapper は lint rule や formatter logic を実装しない

### Requirement: Wrapper publication SHALL be gated by ownership and smoke tests

システムは、package ownership と smoke test が揃うまで npm / PyPI package を公式公開してはならない（SHALL NOT）。

#### Scenario: wrapper publish is attempted

- **WHEN** release flow が wrapper publish を実行しようとする
- **THEN** system は package name ownership と publish credential を確認する
- **AND** system は clean environment で wrapper install と `kml --version` を検証する
- **AND** 条件を満たさない場合、system は wrapper を公式導線として公開しない
