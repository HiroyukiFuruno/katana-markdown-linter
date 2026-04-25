## Why

`rumdl` demonstrates that a Markdown linter becomes easier to adopt when it is
available through multiple ecosystems and editor/automation workflows. kml is a
Rust library-first project, but the CLI should still be easy to install and use
outside Cargo once the lint/fix/format contracts are stable.

## What Changes

- Evaluate distribution channels: Homebrew, npm wrapper, pip/uv wrapper, standalone GitHub artifacts, pre-commit, and GitHub Action.

- Define which channels are official and which are documented as examples.

- Add schema/config/editor integration plan when it benefits users.

- Keep core crate independent from distribution wrappers.

## Scope

In scope:

- Distribution and integration planning.
- Release workflow changes needed for selected official channels.
- Documentation for supported channels.

Out of scope:

- Rewriting the core linter.
- Adding formatter semantics.
- Consumer application specific adapters.

## Impact

- kml becomes easier to adopt in CI and editor workflows.
- Release flow gains additional artifact responsibilities only after the core quality contract is stable.
