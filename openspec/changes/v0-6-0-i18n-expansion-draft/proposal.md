## Why

Issue #4 shows that localization is not just a CLI presentation concern.
Embedding consumers, MCP tools, settings screens, rule catalogs, and future
editor integrations need a consistent i18n boundary broader than diagnostic
message rendering.

v0.4.3 should close the immediate API gap with additive locale resolver and
localized rule metadata helpers. v0.6.0 should then treat i18n as a product
surface and define the next durable contract.

## Scope

v0.6.0 promotes i18n from diagnostic rendering to a repository-wide product
surface while keeping the crate generic and application-independent.

- Keep English and Japanese as the only supported locales in this release.

- Make `Locale` non-exhaustive in the v0.6.0 semver-minor line so future
  locale additions do not repeatedly break consumers.

- Provide localized rule catalog output for Rust API, CLI `kml rule`, and MCP
  `rule_list` / `rule_get`.

- Localize config validation errors with stable message IDs and structured
  parameters.

- Add translation coverage gates for supported message IDs and active rule
  descriptions.

- Preserve the existing fallback split: CLI explicit unsupported locale remains
  a hard error, OS locale fallback is English, and library/MCP language-code
  helpers are lenient.

- Keep catalogs Rust-native for v0.6.0; moving to data files remains a later
  implementation detail once additional locales create enough pressure.

- Document the public API stability rule for future locale additions.

## Deliverables

- A detailed design for kml-wide i18n boundaries across library, CLI, and MCP.

- A locale support matrix and fallback policy.

- A translation catalog coverage gate.

- Localized Rust API, `kml rule`, and MCP `rule_list` / `rule_get` output.

- Localized config validation errors in CLI JSON/text and MCP responses.

- A migration note for consumers currently matching `Locale` exhaustively.

## Out of Scope

- Implementing new locales immediately.

- Rewriting the current message catalog before v0.4.3 is released

- Coupling i18n behavior to any specific consuming application.

- Unsafe fix mode and workspace-writing MCP productization.

## Resolved Kickoff Decisions

- Additional locales are deferred; v0.6.0 hardens en/ja first.

- CLI explicit unsupported locale remains a hard error for all commands.

- Library resolver behavior stays lenient while CLI behavior stays strict.

- MCP responses default to English and accept an explicit `locale` parameter.

- A single Rust-native catalog is sufficient for v0.6.0.

## Readiness Notes

This change is ready for implementation when:

- `design.md` defines final API and catalog architecture.

- `tasks.md` includes DoR, DoD, and surface-by-surface progress.

- Delta specs cover library i18n API, CLI localized catalog output, MCP
  metadata localization, and coverage gates.
