## Why

Issue #4 shows that localization is not just a CLI presentation concern.
Embedding consumers, MCP tools, settings screens, rule catalogs, and future
editor integrations need a consistent i18n boundary broader than diagnostic
message rendering.

v0.4.3 should close the immediate API gap with additive locale resolver and
localized rule metadata helpers. v0.6.0 should then treat i18n as a product
surface and define the next durable contract.

## Draft Scope

v0.6.0 should investigate and plan:

- locale support policy beyond English and Japanese

- whether `Locale` should become non-exhaustive in a semver-minor release

- localized rule catalog output for CLI and MCP

- localized config validation errors with stable error ids and parameters

- translation coverage tooling for rule descriptions, config errors, CLI text,
  MCP metadata, and docs snippets

- fallback policy for explicit user locale vs OS locale vs API-provided locale

- whether message catalogs should stay Rust-native or move to data files

- public API stability rules for adding future locales

## Candidate Deliverables

- A detailed design for kml-wide i18n boundaries across library, CLI, and MCP

- A locale support matrix and fallback policy

- A translation catalog coverage gate

- Localized `kml rule` / MCP `rule_list` output if product value is confirmed

- A migration note for consumers currently matching `Locale` exhaustively

## Out of Scope For The Draft

- Implementing new locales immediately

- Rewriting the current message catalog before v0.4.3 is released

- Coupling i18n behavior to any specific consuming application

- Blocking v0.5.0 DocumentContext / AST work

## Open Questions For Kickoff

- Which additional locales are worth supporting first?

- Should CLI explicit unsupported locale remain a hard error for all commands?

- Should library resolver behavior stay lenient while CLI behavior stays strict?

- Does MCP need localized metadata by default, or should clients request locale
  explicitly?

- Do rule descriptions need per-rule translation files, or is a single catalog
  sufficient for the next minor release?

## Readiness Notes

When the draft is promoted to implementation planning, create or expand:

- `design.md` with final API and catalog architecture

- `tasks.md` with DoR, DoD, and rule-by-rule / surface-by-surface progress

- delta specs for library i18n API, CLI localized catalog output, and MCP
  metadata localization if those surfaces are included
