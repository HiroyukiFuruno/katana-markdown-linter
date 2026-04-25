## Why

Issue #4 reports that embedding consumers currently need to know kml's concrete
`Locale` enum and duplicate fallback logic when they receive UI locale strings
such as `en`, `en-US`, `ja`, `ja-JP`, `fr`, or `zh-CN`.

The CLI already resolves locale for diagnostic rendering, but the library API
does not expose a stable helper for generic language code input. Rule metadata
also exposes English descriptions only, so settings UIs and MCP rule metadata
tools cannot reuse kml's existing localized rule messages without manually
constructing diagnostic-like parameters.

This is a small additive API gap and should be handled as v0.4.3 rather than
waiting for the broader v0.6.0 i18n expansion.

## What Changes

- Add a public locale resolver that accepts arbitrary language or locale code
  strings and returns the best supported kml locale.

- Add a resolver variant that lets callers specify their own fallback locale.

- Add a public helper for localized rule descriptions by rule id, fallback
  description, and language code string.

- Add a `RuleMeta` convenience method that returns the localized description for
  that rule using the same resolver.

- Re-export the relevant locale and localized metadata helpers from the crate
  root so embedding users do not need to depend on internal module layout.

- Document that downstream consumers should call resolver APIs instead of
  exhaustively matching `Locale` for fallback behavior.

## Impact

- Rust embedding users can pass UI language codes directly to kml without
  duplicating fallback policy.

- CLI and MCP rule metadata paths can reuse the same localized rule metadata
  behavior when exposing catalog output.

- Existing `RuleMeta.description` remains English for source compatibility.

- Existing `Locale` enum is not marked `#[non_exhaustive]` in v0.4.3 because
  doing so can break downstream exhaustive matches in a patch release.

## User Decisions

- v0.4.3 is the patch release target for issue #4.

- v0.6.0 should become the broader i18n expansion line, but this change should
  stay narrow and additive.

## Non-Goals

- Adding new supported locales beyond English and Japanese.

- Translating every CLI command surface beyond the existing diagnostic catalog.

- Changing CLI behavior for explicit unsupported `--locale` values.

- Marking `Locale` as `#[non_exhaustive]` in v0.4.3.

- Introducing ICU, Fluent, gettext, or runtime translation dependencies.
