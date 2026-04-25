# Tasks

## Definition Of Ready

- [x] v0.5.0 is released and `source-preserving-document-context` is archived.
- [x] Issue #4 follow-up is closed or no longer blocks v0.6.0.
- [x] Implementation branch is based on latest `origin/main`.
- [x] OpenSpec proposal, design, delta specs, and tasks exist for this change.
- [x] Scope excludes new locales, unsafe fix mode, and workspace-writing MCP tools.

## 0. Planning

- [x] 0.1 Finalize proposal from draft to implementation-ready scope.
- [x] 0.2 Add delta spec for library i18n API.
- [x] 0.3 Add delta spec for CLI localized rule/config output.
- [x] 0.4 Add delta spec for MCP localized metadata/config output.
- [x] 0.5 Add design with fallback policy and API stability decisions.

## 1. Library i18n API

- [x] 1.1 Mark `Locale` as non-exhaustive.
- [x] 1.2 Add localized rule catalog API while preserving canonical English metadata.
- [x] 1.3 Add stable config validation message IDs and structured params.
- [x] 1.4 Add localized config error rendering helper.
- [x] 1.5 Add unit tests for fallback policy, localized catalog, and config error metadata.

## 2. CLI i18n Surface

- [x] 2.1 Route selected locale into `kml rule` and `kml rule <id>`.
- [x] 2.2 Include localized descriptions and canonical English descriptions in JSON rule output.
- [x] 2.3 Use structured localized config errors for validation failures.
- [x] 2.4 Add CLI tests for Japanese rule output and JSON config error metadata.

## 3. MCP i18n Surface

- [x] 3.1 Add optional locale request fields to text and metadata tools.
- [x] 3.2 Localize `check_text` and `fix_text` diagnostics when requested.
- [x] 3.3 Localize `config_validate` errors when requested.
- [x] 3.4 Localize `rule_list` and `rule_get` descriptions when requested.
- [x] 3.5 Add MCP tests for localized rule metadata, diagnostics, and config errors.

## 4. Coverage And Documentation

- [x] 4.1 Add translation coverage gate for supported message IDs.
- [x] 4.2 Add translation coverage gate for active rule descriptions.
- [x] 4.3 Update README with v0.6.0 i18n behavior and `Locale` migration note.
- [x] 4.4 Update MCP documentation with locale request behavior.
- [x] 4.5 Update examples to demonstrate localized catalog usage.

## 5. Release Preparation

- [x] 5.1 Bump crate version to 0.6.0.
- [x] 5.2 Add CHANGELOG entry for v0.6.0.
- [x] 5.3 Run `cargo fmt --all -- --check`.
- [x] 5.4 Run `cargo test --workspace --locked`.
- [x] 5.5 Run `cargo test --test ast_linter --locked`.
- [x] 5.6 Run `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`.
- [x] 5.7 Run `make dogfood`.
- [x] 5.8 Run `make release-check VERSION=v0.6.0`.
- [x] 5.9 Run `git diff --check`.

## Definition Of Done

- [x] Rust API exposes localized catalog/config error helpers without app-specific coupling.
- [x] CLI rule/config output respects selected locale and keeps machine-readable metadata stable.
- [x] MCP read-only tools accept explicit locale and preserve structured fields.
- [x] Translation coverage gates fail on missing supported-locale message IDs or rule descriptions.
- [x] v0.6.0 release readiness gates pass locally and in CI.
