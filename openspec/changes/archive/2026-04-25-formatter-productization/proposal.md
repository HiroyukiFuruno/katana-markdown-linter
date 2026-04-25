## Why

Lint fixes and formatting overlap, but they are not the same product contract.
kml should first improve linter precision and safe fix coverage. Once that
baseline is stronger, formatter behavior can be designed as a deterministic,
idempotent policy rather than a collection of lint rule side effects.

For `v0.10.0`, formatter scope is intentionally narrow: indentation and newline
normalization only. It must not become a general Markdown rewriter.

## What Changes

- Define `kml fmt` as a layout formatter distinct from `kml fix`.
- Add a Rust library entrypoint for deterministic Markdown layout formatting.
- Normalize only layout-safe areas: line endings, final newline, repeated blank lines, structural blank lines, and safe list indentation/marker spacing.
- Define formatter exit codes for editor integration.
- Add idempotence tests and stdin/stdout contract tests.

## Scope

In scope:

- Formatter semantics and API shape for indentation and newline normalization.
- Layout-only formatter rule subset: `MD005`, `MD007`, `MD012`, `MD022`, `MD030`, `MD031`, `MD032`, `MD047`, and `MD058`.
- CRLF/CR to LF normalization.
- Idempotence and stdout behavior.
- Documentation that explains `check`, `fix`, and `fmt`.

Out of scope:

- Unsafe formatter rewrites without opt-in.
- LSP/editor distribution.
- New markdownlint rule implementation.
- Hard wrapping or paragraph reflow.
- Heading style, emphasis style, URL/reference style, table style, or marker style conversion.
- Removing trailing spaces because they can encode Markdown hard line breaks.

## Impact

- Users can distinguish lint-driven fixes from whole-document formatting.
- Editor integrations can use stable formatter behavior later.
- Formatting becomes predictable enough to be used on stdin/stdout without
  accidentally inheriting `check --fix` exit-code behavior.
