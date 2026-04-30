## ADDED Requirements

### Requirement: editor wrappers SHALL depend on kml lsp protocol capabilities

editor wrapper は、`kml lsp` の protocol capabilities に依存しなければならない（SHALL）。

#### Scenario: editor initializes kml lsp

- **WHEN** editor wrapper sends `initialize`
- **THEN** `kml lsp` returns server name `katana-markdown-linter`
- **AND** `kml lsp` advertises document formatting support
- **AND** `kml lsp` advertises range formatting support
- **AND** `kml lsp` advertises code action support

### Requirement: kml lsp SHALL publish diagnostics for opened and changed documents

`kml lsp` は、opened / changed document に対して diagnostics を返さなければならない（SHALL）。

#### Scenario: markdown document changes

- **WHEN** editor wrapper sends `textDocument/didOpen` or `textDocument/didChange`
- **THEN** `kml lsp` publishes diagnostics for that document URI
- **AND** diagnostic codes match the same lint rules used by `kml check`

### Requirement: kml lsp SHALL keep editor actions safe by default

`kml lsp` の editor action は、default-safe behavior を維持しなければならない（SHALL）。

#### Scenario: editor requests code actions

- **WHEN** editor wrapper requests `textDocument/codeAction`
- **THEN** `kml lsp` returns only safe quick-fix actions by default
- **AND** unsafe fix behavior is not exposed without explicit opt-in
