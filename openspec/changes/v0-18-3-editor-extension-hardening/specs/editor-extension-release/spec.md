## ADDED Requirements

### Requirement: editor extensions SHALL declare kml CLI compatibility

editor extension は、対応する `kml` CLI version range を明示しなければならない（SHALL）。

#### Scenario: extension starts language server

- **WHEN** extension starts `kml lsp`
- **THEN** extension verifies the configured `kml` executable version
- **AND** extension accepts versions in the documented compatibility range
- **AND** extension reports a clear error for unsupported versions

### Requirement: editor extension packages SHALL be release-verifiable

editor extension package は、release 前に機械的に検証できなければならない（SHALL）。

#### Scenario: release check validates extension packages

- **WHEN** developer runs `make release-check VERSION=vX.Y.Z`
- **THEN** system validates the VS Code extension package
- **AND** system validates the Zed extension package
- **AND** system verifies package contents exclude generated junk and local-only files
- **AND** system verifies each extension can launch `kml lsp` in smoke mode

### Requirement: editor extension docs SHALL separate setup workflows

editor extension docs は、利用者が実際に操作する workflow ごとに分かれていなければならない（SHALL）。

#### Scenario: user reads editor integration docs

- **WHEN** user wants config validation only
- **THEN** docs show schema mapping without requiring extension install
- **WHEN** user wants Markdown diagnostics or formatting
- **THEN** docs show extension install and `kml` binary setup
- **WHEN** user uses Neovim
- **THEN** docs show LSP configuration sample without claiming a maintained plugin exists

### Requirement: marketplace publish SHALL be gated by account and package verification

Marketplace / registry publish は、account 設定と package verification が揃うまで実行してはならない（SHALL NOT）。

#### Scenario: extension publish is requested

- **WHEN** release workflow or runbook publishes editor extensions
- **THEN** system confirms publisher account and package name are configured
- **AND** system confirms package validation passed for the exact release version
- **AND** system does not publish unverified packages automatically
