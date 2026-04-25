# Changelog

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
