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

### v0.11.0 Decision

The first official non-Cargo channel is a root GitHub Action. It installs the
published crate by default and supports a local path mode for repository smoke
tests. The action is intentionally a thin wrapper over `kml`; it does not embed
Node, Python, or consumer-specific adapter logic.

Release gates SHALL include an action smoke target that installs from the local
checkout and runs `kml check` through the same scripts used by the GitHub
Action. Other channels are documented as deferred until binary artifact naming,
schema metadata, or dedicated ownership is stable.

### Non-Goals

- Node/Python implementation of the linter.
- KatanA-specific adapter.
- Distribution before release verification remains reliable.
