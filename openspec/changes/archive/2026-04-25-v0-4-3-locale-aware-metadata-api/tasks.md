## Definition of Ready

- [x] Issue #4 has been reviewed and referenced in the implementation notes
- [x] v0.4.2 is the latest published release and v0.4.3 is available as the next patch version
- [x] The current implementation branch is based on `origin/main` and isolated from unrelated v0.5.0 work
- [x] Public API additions are confirmed additive and source-compatible
- [x] v0.6.0 broader i18n work is split into a separate draft change

## 1. Public Locale Resolver API

- [x] 1.1 Add `resolve_locale_code(language_code: &str) -> Locale`
- [x] 1.2 Add `resolve_locale_code_or(language_code: &str, fallback: Locale) -> Locale`
- [x] 1.3 Re-export `Locale`, `LocaleError`, and resolver helpers from the crate root
- [x] 1.4 Preserve existing `Locale::resolve(explicit)` CLI error behavior
- [x] 1.5 Add tests for primary language, region, underscore, charset suffix, blank input, and unsupported locale codes

## 2. Localized Rule Metadata API

- [x] 2.1 Add `localized_rule_description(rule_id, fallback_description, language_code)`
- [x] 2.2 Add `RuleMeta::localized_description(&self, language_code)`
- [x] 2.3 Reuse the existing diagnostic message catalog instead of duplicating translations
- [x] 2.4 Keep `RuleMeta.description` as the English canonical description
- [x] 2.5 Add tests for known Japanese rule descriptions, English fallback, and unknown rule ids

## 3. Documentation And Examples

- [x] 3.1 Update README library API list with locale resolver and localized metadata helpers
- [x] 3.2 Update `examples/embedding.rs` or add a small example section showing localized rule descriptions
- [x] 3.3 Document that consumers should use resolver helpers instead of implementing locale fallback matches
- [x] 3.4 Document why `Locale` is not made non-exhaustive in v0.4.3
- [x] 3.5 Add a v0.4.3 changelog entry referencing issue #4

## 4. Release Preparation

- [x] 4.1 Bump crate version to 0.4.3
- [x] 4.2 Ensure Cargo.lock reflects the version bump
- [x] 4.3 Run local release preflight for a patch release
- [x] 4.4 Confirm issue #4 acceptance criteria are covered before release

## Verification

- [x] `cargo fmt --all -- --check`
- [x] `cargo test --workspace --locked`
- [x] `cargo test --test ast_linter --locked`
- [x] `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- [x] `make dogfood`
- [x] `make release-check VERSION=v0.4.3`
- [x] `git diff --check`

## Definition of Done

- [x] Consumers can pass arbitrary language or locale code strings and receive kml's supported fallback consistently
- [x] Consumers can render localized rule descriptions without constructing diagnostic message parameters
- [x] CLI explicit unsupported locale behavior remains unchanged
- [x] Existing public fields and existing API behavior remain source-compatible
- [x] Documentation explains the resolver API and v0.4.3 enum compatibility decision
- [x] Issue #4 can be closed after v0.4.3 release
