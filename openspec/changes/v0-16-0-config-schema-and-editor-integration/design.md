# Design: Config Schema and Editor Integration

## JSON Schema

- Use `schemars` or similar to derive JSON schema from the `MarkdownLintConfig` struct.
- Expose a `kml config schema` command to output the current schema.
- Host the schema at a stable URL (e.g., GitHub Pages or raw GitHub URL) for editor discovery.
- Include rule descriptions and default values in the schema metadata.

## LSP Entrypoint

- Implement a lightweight, stdio-based LSP server that wraps the existing library functions.
- Support `textDocument/didOpen`, `textDocument/didChange`, `textDocument/didSave` for real-time diagnostics.
- Support `textDocument/formatting` using the `fmt` logic.
- Support `textDocument/codeAction` to apply safe fixes.
- The LSP server should be part of the `kml` binary or a separate `kml-lsp` binary (to be decided).

## Editor Integration

- Document how to configure `json.schemas` in VS Code.
- Document how to use `kml` with `null-ls` or `efm-langserver` for Neovim.
- Explore creating a thin VS Code extension that bundles the schema and optionally the LSP.

## Configuration Validation

- Update `kml config file` and `kml-mcp`'s `config_validate` to use the JSON schema for validation instead of just basic JSON parsing.
- Provide clear error messages with line/column information when validation fails.
