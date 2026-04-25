# Changelog

## v0.8.1

- Improves `MD048` code fence style checks by evaluating parsed fenced code blocks and honoring `style = consistent`, `backtick`, and `tilde`.
- Adds conservative `MD048` safe fixes that rewrite fence markers only when block-local collision guards pass.
- Adds fixture and golden coverage for `MD048` fix output and collision-safe diagnostic-only behavior.

## v0.8.0

- Adds safe fixes for `MD035` horizontal rule style and `MD055` table pipe style.
- Improves `MD035` precision by ignoring YAML front matter delimiters when selecting the consistent horizontal rule style.
- Improves `MD055` precision by evaluating parsed table blocks instead of pipe-like text inside fenced code.
- Adds fix idempotence coverage to the rule fixture harness.
- Documents diagnostic-only rule fix feasibility so later unsafe and formatter work remains separate.

## v0.7.0

- Optimizes safe fix application by replacing repeated full-document range scans with a per-apply line offset index.
- Preserves multi-line fix ranges, virtual EOF insertion, UTF-8 boundary clamping, and overlap resolution behavior.
- Refreshes performance baselines and documents the v0.7.0 fix range snapshot.
- Reorganizes the roadmap so linter precision and safe fix coverage come before unsafe fix mode, formatter productization, and tool distribution expansion.
- Adds OpenSpec planning for safe fix coverage, formatter productization, and rumdl-inspired distribution/editor expansion.

## v0.6.0

- Promotes i18n to a library, CLI, and MCP product surface while keeping English and Japanese as the supported locale set.
- Adds localized rule catalog APIs with canonical English metadata preserved.
- Makes `Locale` non-exhaustive so future locale additions can be handled without repeating exhaustive-match breakage.
- Localizes `kml rule` text/JSON output and config validation errors with stable message IDs and parameters.
- Adds optional MCP `locale` request support for diagnostics, config validation errors, and rule metadata.
- Adds translation coverage gates for supported message IDs and active rule descriptions.

## v0.5.0

- Adds source-preserving `DocumentContext` for shared line, heading, code block, reference, table, and lazy AST structure.
- Adds context-based rule evaluation while keeping existing `lint` and `fix` API compatibility.
- Migrates `MD001` and `MD060` to context-backed evaluation and safe source-range fixes.
- Expands `MD060` table column style check/fix coverage, including aligned, compact, and tight table styles.
- Improves fix application by selecting non-overlapping edits before constructing the output.
- Adds v0.5.0 performance snapshots for context construction and migrated rule paths.
- Fixes an `MD003` false positive where standalone horizontal rules or front matter delimiters were treated as setext headings.

## v0.4.3

- Adds locale resolver helpers for embedding consumers: `resolve_locale_code()` and `resolve_locale_code_or()`.
- Adds localized rule metadata helpers via `localized_rule_description()` and `RuleMeta::localized_description()`.
- Documents that consumers should use resolver helpers for UI language codes instead of duplicating kml fallback policy.
- Keeps `Locale` source-compatible in the v0.4 patch line while recording broader i18n expansion for v0.6.0.
- Closes issue #4.

## v0.4.2

- Adds `MarkdownLintConfig::to_lint_options()` so embedding applications can load `.markdownlint.json` and run `lint` without duplicating CLI conversion logic.
- Routes CLI config handling through the same public conversion API to keep embedded and CLI behavior aligned.
- Updates the embedding example and README to show config-to-options conversion.

## v0.4.1

- Fixes the locale CLI option spelling by adding `--locale` as the canonical long flag.
- Keeps `--local` as a backward-compatible alias for v0.4.0 users.
- Updates repository dogfood and README examples to use `--locale`.

## v0.4.0

- Adds localized `kml check` diagnostics with `--local` / `-l`, English and Japanese output, OS locale fallback, and stable JSON `message_id` / `message_params`.
- Expands safe fix coverage for `MD005` nested sibling indentation drift and `MD030` list marker spacing, including configured single-line spacing.
- Adds locked golden cases for `MD005` and `MD030`, plus edge coverage for CRLF trailing spaces, Unicode line length, fenced list markers, and table column style boundaries.
- Updates the rule coverage dashboard to derive golden status from the locked baseline and show safe fix counts separately from manual-required candidates.
- Hardens release operations and local dogfood behavior so locale drift does not affect Markdown quality gates.

## v0.3.0

- Expands safe automatic fixes for markdownlint-compatible rules including trailing spaces, hard tabs, reversed links, command prompts, closed ATX heading spacing, blockquote spacing, bare URLs, emphasis/strong style, link fragments, duplicate link definitions, link style, and table spacing.
- Passes rule configuration properties into runtime evaluation so setting-dependent rules can avoid guessing and only fix when enough configuration is available.
- Hardens default-safe behavior for proper names and link-style fixes to avoid speculative edits.
- Adds fixture matrix checks for active rules, fixability, documented rule parameters, Markdown summary consistency, and fix before/after outputs.
- Adds cross-tool benchmark fix validation and repository dogfood baselines to verify that fixes converge without modifying source fixtures.
- Adds an optional experimental MCP server prototype and shared Serena project configuration.

## v0.2.0

- Adds signed annotated tag verification to the release flow.
- Improves CLI directory traversal, explicit file handling, gitignore behavior, and AST lint gates.
- Cleans release packaging inputs so generated or build artifact directories are not included.
- Adds planning artifacts for the markdownlint parity expansion that later became the v0.3.0 quality and fix coverage work.

## v0.1.0

- Initial public release of `katana-markdown-linter`.
- Provides a Rust library API for markdownlint-compatible `check`, `fix`, and config helpers.
- Provides the `kml` CLI with `check`, `fix`, `init-config`, `--config`, and `--format json`.
- Adds upstream markdownlint drift tracking for rule document changes.
