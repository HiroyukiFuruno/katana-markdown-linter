# Design

## Goals

- Treat i18n as a library/CLI/MCP contract, not only diagnostic text rendering.
- Keep the crate generic and independent from any consuming application.
- Avoid adding new locales before translation coverage can be enforced.
- Preserve strict CLI locale validation while keeping embedding/MCP locale input lenient.

## Decisions

### Locale Support

The v0.6.0 i18n expansion supports English and Japanese only. Additional
language catalogs are deferred until coverage gates can prevent partial
catalog additions.

### Locale API Stability

`Locale` becomes `#[non_exhaustive]` in v0.6.0. This is a semver-minor boundary in the `0.x` series and is intentional because future locale additions should not repeatedly break exhaustive matches. Documentation must tell consumers to add wildcard match arms.

### Fallback Policy

- CLI explicit `--locale` / `-l`: strict. Unsupported values return an error.
- CLI omitted locale: OS locale detection, then English fallback.
- Library language-code helpers: lenient. Unsupported values resolve to English or caller-provided fallback.
- MCP locale parameter: lenient. Unsupported values resolve to English.

### Catalog Representation

Canonical rule metadata remains English. Localized APIs return owned metadata with localized descriptions while preserving stable IDs, docs URLs, lifecycle, and fixability.

The Rust-native catalog remains the storage mechanism for v0.6.0. Data-file catalogs are deferred until additional locales make operational overhead worthwhile.

### Config Validation Localization

`ConfigError` gains stable message IDs and structured params derived from `ConfigErrorKind`. Text rendering remains separate from error classification. CLI and MCP responses can localize messages without losing machine-readable kind, expected, actual, allowed values, rule ID, or property.

### MCP Boundary

`kml-mcp` remains an adapter over public crate APIs. It accepts optional `locale` request fields and returns localized diagnostics/rule metadata/config errors. The core crate remains MCP-free.

## Validation Strategy

- Unit tests cover non-exhaustive-compatible API behavior, localized catalog APIs, and config error message metadata.
- CLI tests cover `kml rule --locale ja` and localized JSON config errors.
- MCP tests cover localized `rule_list`, `rule_get`, `check_text`, and `config_validate`.
- AST lint covers translation coverage for supported message IDs and active rule descriptions.
- Release check remains the final gate.

## Out of Scope

- Adding new locales.
- Unsafe fix mode.
- Workspace-writing MCP tools.
- Moving catalogs to external data files.
