# Tasks

## Definition of Ready

- [ ] `v0.15.0` remote transport work is completed and released.
- [ ] Requirements for JSON schema (rule descriptions, types) are stable in the library.
- [ ] Use cases for LSP diagnostics and formatting are prioritized.

## 1. JSON Schema Implementation

- [ ] 1.1 Add `schemars` (or equivalent) to generate JSON schema from `MarkdownLintConfig`.
- [ ] 1.2 Implement `kml config schema` command.
- [ ] 1.3 Add automated test to verify schema correctness against example configs.
- [ ] 1.4 Document the stable schema URL.

## 2. LSP Entrypoint

- [ ] 2.1 Design the LSP server architecture (stdio-based).
- [ ] 2.2 Implement basic LSP lifecycle (initialize, shutdown).
- [ ] 2.3 Implement diagnostic reporting on file open/change.
- [ ] 2.4 Implement document formatting via LSP.
- [ ] 2.5 Implement code actions for safe fixes.

## 3. Editor Integration & Docs

- [ ] 3.1 Create VS Code configuration guide for schema and LSP.
- [ ] 3.2 Create Neovim/Zed integration guides.
- [ ] 3.3 Update README with editor support section.

## 4. Validation Hardening

- [ ] 4.1 Update CLI config validation to use the JSON schema.
- [ ] 4.2 Update MCP `config_validate` to use the JSON schema.
- [ ] 4.3 Add integration tests for schema-based validation.

## Verification

- [ ] `kml config schema` outputs valid JSON schema.
- [ ] LSP server responds to initialize and formatting requests.
- [ ] Diagnostics are correctly reported over LSP.
- [ ] `make check` passes.
- [ ] `make ast-lint` passes.

## Definition of Done

- [ ] JSON schema is available and used for configuration validation.
- [ ] LSP entrypoint provides diagnostics and formatting.
- [ ] Editor integration is documented for at least three major editors.
