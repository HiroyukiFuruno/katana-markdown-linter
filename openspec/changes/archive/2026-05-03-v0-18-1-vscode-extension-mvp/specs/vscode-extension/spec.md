## ADDED Requirements

### Requirement: VS Code extension SHALL launch kml LSP as a thin wrapper

VS Code extension は、`kml lsp` を起動する薄いラッパー（thin wrapper）でなければならない（SHALL）。

#### Scenario: markdown document opens

- **WHEN** user opens a Markdown document in VS Code
- **THEN** extension starts `kml lsp` over stdio
- **AND** extension attaches the language client to Markdown documents
- **AND** extension does not implement markdownlint rules in TypeScript

### Requirement: VS Code extension SHALL expose diagnostics and safe editor actions

VS Code extension は、LSP 経由で diagnostics、format、safe quick-fix を公開しなければならない（SHALL）。

#### Scenario: user edits Markdown

- **WHEN** `kml lsp` publishes diagnostics for an open Markdown document
- **THEN** VS Code displays those diagnostics in the editor
- **AND** format document uses `textDocument/formatting`
- **AND** range format uses `textDocument/rangeFormatting`
- **AND** quick fix uses `textDocument/codeAction`

### Requirement: VS Code extension SHALL support explicit kml path configuration

VS Code extension は、`kml` executable の path を明示設定できなければならない（SHALL）。

#### Scenario: kml is not on PATH

- **WHEN** user configures an absolute `kml` path
- **THEN** extension launches that executable with the `lsp` argument
- **AND** extension reports a clear error when the configured executable cannot run
- **AND** extension does not silently fallback to an unrelated binary

### Requirement: VS Code extension SHALL associate markdownlint config files with schema

VS Code extension は、markdownlint config files に published schema を関連付けなければならない（SHALL）。

#### Scenario: user opens markdownlint config

- **WHEN** user opens `.markdownlint.json` or `.markdownlint.jsonc`
- **THEN** VS Code can use the published schema for completion and validation
- **AND** extension keeps manual schema mapping documented for users who do not install it
