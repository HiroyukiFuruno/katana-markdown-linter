## Design

### Reference Points From rumdl

Useful ideas:

- Cargo, Homebrew, npm, pip/uv, standalone binaries
- pre-commit hook repository
- GitHub Action wrapper
- config schema for editor autocomplete
- LSP/editor entrypoint

kml should choose channels based on maintenance cost and user value, not parity
for its own sake.

### Proposed Ordering

1. GitHub Action wrapper and pre-commit docs.
2. Homebrew tap if release artifacts are stable.
3. npm/pip wrappers only after binary artifact naming is stable.
4. Editor/LSP work after formatter semantics are stable.

### Non-Goals

- Node/Python implementation of the linter.
- KatanA-specific adapter.
- Distribution before release verification remains reliable.
