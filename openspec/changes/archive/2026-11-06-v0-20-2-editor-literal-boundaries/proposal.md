## Why

Editor positions, selection formatting, literal Markdown regions and strict
configuration handling diverged across entrypoints. Those differences could
damage links, alter literal content and apply fixes under invalid settings.

## What Changes

- Share source positions with the editor adapter and format selections in
  document context.
- Derive literal protection from the Markdown parser and parse on demand.
- Share strict configuration preparation across CLI, LSP and MCP.
- Verify with protocol edits, independently written answers and unchanged
  performance and coverage thresholds.
- Update compatible Rust, VS Code and GitHub Actions dependencies.

## Capabilities

### Modified Capabilities

- `editor-safe-actions`: Valid UTF-16 edits and context-aware selection formatting.
- `shared-inline-config-contracts`: Literal source protection and strict preparation.

## Impact

Rust library internals, CLI, LSP, MCP, regression corpora, editor development
dependencies, release workflows and synchronized v0.20.2 distribution metadata.
