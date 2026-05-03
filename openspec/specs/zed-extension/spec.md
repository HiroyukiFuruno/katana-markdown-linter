# zed-extension Specification

## Purpose

The Zed extension provides KatanA Markdown Linter (kml) integration for the Zed
editor. It acts as a thin wrapper that registers `kml lsp` as a Markdown
language server, enabling diagnostics, formatting, and safe quick fixes within
Zed.

## Requirements

### Requirement: Zed extension SHALL register kml as a Markdown language server

Zed extension は、`kml lsp` を Markdown 用 language server として登録しなければならない（SHALL）。

#### Scenario: markdown workspace opens in Zed

- **WHEN** user opens a Markdown document in Zed
- **THEN** extension starts `kml lsp` over stdio
- **AND** extension registers it for Markdown documents
- **AND** extension does not implement markdownlint rules in extension code

### Requirement: Zed extension SHALL preserve the shared editor LSP contract

Zed extension は、VS Code extension と同じ editor-facing LSP contract を利用しなければならない（SHALL）。

#### Scenario: kml lsp returns editor features

- **WHEN** Zed extension initializes `kml lsp`
- **THEN** diagnostics are sourced from `textDocument/publishDiagnostics`
- **AND** formatting is sourced from `textDocument/formatting`
- **AND** range formatting is sourced from `textDocument/rangeFormatting`
- **AND** safe quick fixes are sourced from `textDocument/codeAction`

### Requirement: Zed extension SHALL support explicit kml path configuration

Zed extension は、`kml` executable の path を明示設定できなければならない（SHALL）。

#### Scenario: user configures kml path

- **WHEN** user sets the `kml` executable path for the Zed extension
- **THEN** extension launches that executable with the `lsp` argument
- **AND** extension reports a clear setup error when the executable cannot run
- **AND** extension does not fallback to a different executable without user consent

### Requirement: Zed docs SHALL cover schema-backed config editing

Zed docs は、markdownlint config の schema-backed editing を説明しなければならない（SHALL）。

#### Scenario: user configures schema validation in Zed

- **WHEN** user follows Zed editor integration docs
- **THEN** docs show how `.markdownlint.json` and `.markdownlint.jsonc` use the published schema
- **AND** docs distinguish schema validation from Markdown LSP diagnostics
- **AND** docs explain when the Zed extension is required
