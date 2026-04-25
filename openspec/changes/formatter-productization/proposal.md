## Why

Lint fixes and formatting overlap, but they are not the same product contract.
kml should first improve linter precision and safe fix coverage. Once that
baseline is stronger, formatter behavior can be designed as a deterministic,
idempotent policy rather than a collection of lint rule side effects.

## What Changes

- Define `kml fmt` as formatter policy distinct from `kml fix`.
- Decide which rewrites belong to formatter-only behavior.
- Define formatter exit codes for editor integration.
- Add idempotence tests and stdin/stdout contract tests.

## Scope

In scope:

- Formatter semantics and API shape.
- Idempotence and stdout behavior.
- Documentation that explains `check`, `fix`, and `fmt`.

Out of scope:

- Unsafe formatter rewrites without opt-in.
- LSP/editor distribution.
- New rule implementation.

## Impact

- Users can distinguish lint-driven fixes from whole-document formatting.
- Editor integrations can use stable formatter behavior later.
