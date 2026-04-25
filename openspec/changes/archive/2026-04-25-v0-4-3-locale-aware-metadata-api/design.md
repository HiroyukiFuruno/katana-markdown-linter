## Design

The v0.4.3 API should expose stable helpers over the existing locale and message
catalog implementation without changing the CLI resolution contract.

## Public API

Add the following public API at the crate root:

- `resolve_locale_code(language_code: &str) -> Locale`

- `resolve_locale_code_or(language_code: &str, fallback: Locale) -> Locale`

- `localized_rule_description(rule_id: &str, fallback_description: &str, language_code: &str) -> String`

Add a convenience method:

- `RuleMeta::localized_description(&self, language_code: &str) -> String`

`resolve_locale_code` returns English when the input is blank or unsupported.
`resolve_locale_code_or` returns the provided fallback when the input is blank or
unsupported. Both helpers normalize case, `_` vs `-`, charset suffixes, and
primary-language fallback in the same way as existing `Locale::parse`.

## Locale Compatibility

Do not change `Locale::resolve(explicit)` CLI semantics:

- explicit unsupported CLI locale remains an error

- OS locale fallback remains English when unsupported

- parser support for `en`, `en-US`, `ja`, `ja-JP`, and underscore variants is
  preserved

The new resolver helpers are for embedding consumers that want lenient fallback
instead of CLI-style errors.

## Rule Description Localization

The implementation should reuse existing catalog rendering rather than
duplicating translations:

- use `rule.generic` with `rule_id`, empty `rule_name`, and fallback description

- use the resolved locale from the language code helper

- return the fallback description unchanged for English

- return the existing Japanese rule message for supported rule ids in Japanese

- return a safe fallback string for unknown rule ids

This keeps rule metadata localization consistent with diagnostic localization
without requiring callers to construct `MessageParams` manually.

## Documentation

README and embedding examples should show the API boundary:

- `available_rules()` remains English metadata

- `RuleMeta::localized_description("ja-JP")` is the localized presentation helper

- downstream consumers should prefer resolver helpers for user language codes

- downstream consumers should not rely on exhaustive `Locale` matches for
  fallback policy

## Tests

Add unit tests that cover:

- `resolve_locale_code` accepts `en`, `en-US`, `ja`, `ja-JP`, underscore variants,
  and charset suffixes

- unsupported locale codes fall back to English or the caller-provided fallback

- localized rule descriptions return Japanese for known rule ids

- unknown rule ids preserve a useful fallback

- `RuleMeta::localized_description` matches the standalone helper

- public API surface AST lint includes the new additive exports

## Release Notes

The v0.4.3 changelog entry should mention issue #4 and call out that this is an
additive embedding API improvement.
