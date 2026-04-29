# Tasks

## Definition of Ready

- [x] `v0.15.0` remote transport work is completed and released.
- [x] Requirements for JSON schema (rule descriptions, types) are stable in the library.
- [x] Use cases for LSP diagnostics and formatting are prioritized.

## 1. JSON Schema Implementation

- [x] 1.1 Add `schemars` (or equivalent) to generate JSON schema from `MarkdownLintConfig`.
- [x] 1.2 Implement `kml config schema` command.
- [x] 1.3 Add automated test to verify schema correctness against example configs.
- [x] 1.4 Document the stable schema URL.

## 2. LSP Entrypoint

- [x] 2.1 Design the LSP server architecture (stdio-based).
- [x] 2.2 Implement basic LSP lifecycle (initialize, shutdown).
- [x] 2.3 Implement diagnostic reporting on file open/change.
- [x] 2.4 Implement document formatting via LSP.
- [x] 2.5 Implement code actions for safe fixes.

## 3. Editor Integration & Docs

- [x] 3.1 Create VS Code configuration guide for schema and LSP.
- [x] 3.2 Create Neovim/Zed integration guides.
- [x] 3.3 Update README with editor support section.

## 4. Validation Hardening

- [x] 4.1 Update CLI config validation to use the JSON schema.
- [x] 4.2 Update MCP `config_validate` to use the JSON schema.
- [x] 4.3 Add integration tests for schema-based validation.

## Verification

- [x] `kml config schema` outputs valid JSON schema.
- [x] LSP server responds to initialize and formatting requests.
- [x] Diagnostics are correctly reported over LSP.
- [x] `make check` passes.
- [x] `make ast-lint` passes.

## Definition of Done

- [x] JSON schema is available and used for configuration validation.
- [x] LSP entrypoint provides diagnostics and formatting.
- [x] Editor integration is documented for at least three major editors.
