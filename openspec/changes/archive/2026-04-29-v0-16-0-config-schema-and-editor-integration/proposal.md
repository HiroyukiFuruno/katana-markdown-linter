# Config Schema and Editor Integration

## Target Version

`v0.16.0`

## Why

After expanding MCP distribution and remote transport, the next barrier to adoption is ease of configuration and editor feedback.

Currently, users must manually edit `.markdownlint.json` without auto-completion or validation. Providing a versioned JSON schema will allow editors like VS Code and Zed to provide real-time validation and documentation for rule parameters.

Furthermore, while `kml fmt --stdin` and MCP tools exist, a dedicated LSP (Language Server Protocol) entrypoint will provide a more standard way for editors to integrate linting, fixing, and formatting without requiring MCP support.

## What Changes

- Implement JSON schema generation for `MarkdownLintConfig`.
- Publish versioned JSON schema for `.markdownlint.json`.
- Design and implement a basic LSP entrypoint for `kml`.
- Add editor integration documentation for VS Code, Zed, and Neovim.
- Wire schema validation into `kml config` and `kml-mcp` validation tools.

## Impact

- Improved developer experience with auto-completion and validation for configuration.
- Standardized editor integration via LSP.
- Reduced friction for new users through better tooling support.

## Non-Goals

- Implementing a full-blown LSP server with complex features (focused on diagnostics and formatting first).
- Support for non-standard configuration formats.
- Rewriting the core linter to be LSP-native (LSP will be a wrapper).
