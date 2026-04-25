# Tasks

## Definition of Ready

- [x] `safe-fix-coverage-continuous-expansion` and `unsafe-fix-mode-and-confirmation` are archived.
- [x] `check`, `fix`, and `fmt` responsibility split is accepted.
- [x] Default formatter behavior does not require unsafe fix policy.
- [x] `v0.10.0` formatter scope is limited to indentation and newline/layout normalization.
- [x] Out-of-scope rewrites are explicit: paragraph reflow, heading style conversion, emphasis/URL style conversion, table style conversion, and trailing-space removal.

## 1. Contract

- [x] 1.1 Define formatter API separate from `fix`.
- [x] 1.2 Define CLI `fmt` semantics as layout formatting, not `fix` alias.
- [x] 1.3 Define formatter exit code behavior: success is `0`, filesystem/config errors are `2`, unresolved non-formatting lint diagnostics do not produce `1`.
- [x] 1.4 Define stdin/stdout behavior for editor integration.
- [x] 1.5 Document formatter rule subset: `MD005`, `MD007`, `MD012`, `MD022`, `MD030`, `MD031`, `MD032`, `MD047`, `MD058`, plus CRLF/CR to LF normalization.

## 2. Implementation

- [x] 2.1 Add library formatter entrypoint and result/options types.
- [x] 2.2 Make `kml fmt` call formatter entrypoint instead of `check --fix` behavior.
- [x] 2.3 Keep `kml fix` and `kml check --fix` behavior unchanged.
- [x] 2.4 Ensure `fmt --stdin` writes only formatted Markdown to stdout on success.
- [x] 2.5 Ensure `fmt` never applies unsafe fix candidates by default.

## 3. Tests And Docs

- [x] 3.1 Add formatter idempotence tests.
- [x] 3.2 Add formatter CRLF/final-newline/blank-line tests.
- [x] 3.3 Add CLI `fmt` exit-code test for unresolved non-formatting diagnostics.
- [x] 3.4 Add CLI `fmt --stdin` stdout contract test.
- [x] 3.5 Update README to compare `check`, `fix`, and `fmt`.
- [x] 3.6 Update examples or public API surface tests to include formatter API.

## Verification

- [x] `cargo fmt --all -- --check` passes.
- [x] `cargo test --workspace --locked` passes.
- [x] `make check` passes.
- [x] `make dogfood` passes.
- [x] `make release-check VERSION=v0.10.0` passes.
- [x] `git diff --check` passes.

## Definition of Done

- [x] Formatter behavior is deterministic and idempotent.
- [x] Formatter contract is separate from lint fix contract.
- [x] Formatter scope is limited to indentation and newline/layout normalization.
- [x] Editor-friendly stdout and exit code behavior is documented.
- [x] Unsafe fixes are not part of default formatter behavior.
