## Purpose

Public API examples keep Rust embedding usage executable and visible for library users.

## Requirements

### Requirement: repository SHALL provide compilable Rust embedding examples

repository は、Rust program から `katana-markdown-linter` を組み込むための compile 可能な examples を提供しなければならない（SHALL）。

#### Scenario: example を compile する

- **WHEN** developer が example build または test command を実行する
- **THEN** system は public API examples を compile する
- **THEN** system は stale な snippet を検出できる

### Requirement: examples SHALL cover core library workflows

examples は、library 利用者が最初に必要とする core workflow を示さなければならない（SHALL）。

#### Scenario: core workflow を参照する

- **WHEN** developer が examples を読む
- **THEN** system は string check、file tree check、string fix、config load の最小例を提供する
- **THEN** examples は KatanA など特定 consuming application に依存しない

### Requirement: README SHALL link CLI and library usage paths

README は、CLI と Rust 組み込み API の利用導線を区別して案内しなければならない（SHALL）。

#### Scenario: 利用導線を確認する

- **WHEN** user が README を読む
- **THEN** system は `kml` CLI と Rust library embedding の違いを示す
- **THEN** system はそれぞれの最小実行例へ誘導する
