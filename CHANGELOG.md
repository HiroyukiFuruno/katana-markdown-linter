# Changelog

## v0.12.13

- Splits `src/cli/workflow.rs` (1197 size-score) into focused sub-modules:
  `workflow/common.rs`, `workflow/check.rs`, `workflow/fmt.rs`, and
  `workflow/config_cmd.rs`, each scoring ≤ 400 in the internal quality gate.
- Eliminates intermediate `Vec` allocation in `md059.rs::normalize_link_text`
  by replacing `collect::<Vec<_>>().join(" ")` with a direct char-push loop.
- Updates `tests/ast_linter.rs` Windows-path compatibility: path separators are
  normalized before string matching to fix two test failures on Windows CI.
- Updates coverage baseline from 873 to 880 to account for the workflow module
  split (cosmetic line-count increase with no functional coverage regression).

## v0.12.12

- Fixes MD003 false positive: diagnostic now points to the heading text line
  instead of the setext underline, matching the markdownlint reference
  implementation behavior.
- Fixes MD046 false positive: 4-space-indented list items (using `-`, `*`, `+`,
  or ordered markers) are no longer misidentified as indented code blocks.
- Splits `src/cli.rs` (2399 lines) into focused sub-modules: `args.rs`,
  `input.rs`, `reporter.rs`, and `workflow.rs`, each scoring ≤ 200 in the
  internal quality gate.
- Splits `src/upstream.rs` (1196 lines) into `upstream/document.rs`,
  `upstream/fixture.rs`, and `upstream/drift.rs`.
- Eliminates per-call `String` clones in severity map construction by switching
  from `HashMap<String, …>` to `HashMap<&str, …>`.
- Reduces intermediate `Vec<&str>` allocations in `heading_style.rs`,
  `style.rs`, and `heading_duplicates.rs` by working directly with `ctx.lines()`
  slices.
- Removes `md-broken-link` from the default evaluation flow to eliminate
  classification noise; the rule implementation is retained for future use.
- Extends `tests/ast_linter.rs` and `scripts/ci/internal-quality.py` to cover
  `tests/` and `build.rs` in addition to `src/`.
- Refreshes performance baseline after optimizations; all benchmarks remain
  within the 1.40× strict regression gate.

## v0.12.11

- No functional code changes. Release prep completed to advance patch version and
  maintain release cadence for the current quality hardening planning iteration.

## v0.12.10

- No functional code changes. Release prep completed to advance patch version and
  maintain release cadence for upcoming MCP distribution work.

## v0.12.9

- Adds a public confidence corpus that exercises `check`, `fix`, and `fmt`
  convergence on committed Markdown with links, images, inline HTML, fenced
  code, tables, reference definitions, and mixed Japanese/English text.
- Adds an optional KatanA external dogfood runner for `docs/**/*.md` and
  `assets/**/*.md`, with source no-write checks, classified diagnostics, and
  command timing evidence.
- Fixes CLI path-aware linting so internal local-link diagnostics resolve
  relative links from the checked file path and ignore non-local schemes such as
  `mailto:`.
- Fixes an `MD028` false positive for adjacent GFM alert blocks such as
  `[!NOTE]`, `[!TIP]`, `[!IMPORTANT]`, `[!WARNING]`, and `[!CAUTION]`.
- Wires public confidence evidence into CI, release preflight, and
  `make release-check` without requiring a private sibling checkout.

## v0.12.8

- Defines a 100-point stable candidate score across precision correctness, safe
  command behavior, performance stability, release reproducibility, and
  evidence quality.
- Records hard blockers that prevent stable acceptance even when the numeric
  score is high.
- Captures the final stable-candidate score report and known limitations before
  allowing v0.13.0 distribution planning to begin.
- Promotes the stable-candidate score to 100/100 by sharing inline HTML
  attribute context across MD033, MD034, and MD051 and validating nested quoted
  attributes.
- Refreshes performance evidence after the shared inline HTML context work, with
  measured cases staying within 0.90x-1.08x of the committed baseline.
- Keeps MCP Registry, Hub, package artifact, and remote transport work frozen
  until the stable score passes and user acceptance is recorded.

## v0.12.7

- Improves post-migration performance by making shared inline-code range lookup
  use the nearest indexed span instead of scanning every inline-code token.
- Adds parser-heavy convergence coverage for the `check` -> `check --fix` ->
  `fix` -> `fmt` -> `check` -> repeated `fmt` workflow.
- Expands performance coverage with table-heavy linting plus parser-heavy fix
  and format cases, then refreshes the committed baseline after correctness
  gates pass.
- Records a stable-score dry run so v0.12.8 can make the final stable
  acceptance judgment from measured evidence.

## v0.12.6

- Migrates inline-context rules beyond the link family so MD033, MD037, MD038,
  MD039, MD044, MD049, and MD050 rely on shared `DocumentContext` inline-code
  or link token evidence instead of isolated backtick or bracket scans.
- Expands document-level regression coverage to include MD037 and MD038, plus
  long and unclosed code-span boundaries for link, proper-name, emphasis, and
  strong-style rules.
- Records the context-sensitive rule migration inventory for v0.12.6 and keeps
  remaining broader AST work assigned to the stable-line follow-up releases.
- Refreshes the performance baseline after measuring the parser/context cost of
  the migration, with v0.12.7 reserved for convergence and performance cleanup.

## v0.12.5

- Adds a source-preserving inline token layer for inline code spans, inline
  links, images, autolinks, reference links, and reference definitions.
- Migrates `MD034`, `MD051`, `MD052`, `MD054`, and `MD059` to shared
  `DocumentContext` token evidence, with `MD053` aligned to the shared
  reference definition index.
- Expands rule-local and document-level regression coverage for nested
  brackets, link titles, images, inline code spans, reference definitions,
  CRLF, Unicode, and unclosed markers.
- Extends performance benchmarking with link-heavy, inline-code-heavy,
  reference-heavy, and inline token index construction cases.
- Syncs the `v0.12.5` parser readiness OpenSpec requirements while keeping
  `v0.13.0` distribution expansion frozen until stable-line acceptance.

## v0.12.4

- Keeps `kml check --fix --output json` identified as the `check` workflow while
  still reporting applied fixes and remaining diagnostics.
- Validates stdin config for `check`, `fix`, and `fmt` before producing output,
  including stable JSON config error metadata for editor and automation use.
- Makes `fix --stdin --output json` report applied fixes, remaining diagnostics,
  changed state, and a non-zero exit code when safe fixes cannot resolve all
  issues.
- Adds integration coverage for the `check` no-write contract, safe-only `fix`
  behavior, formatter-only `fmt` behavior, JSON command identity, and stdin
  config validation.
- Extends performance benchmarking and the committed baseline with API
  formatting plus CLI `fix` and `fmt` representative paths.
- Records the `v0.12.4` core command hardening OpenSpec work before returning
  to MCP Registry and distribution planning.

## v0.12.3

- Fixes `MD034` so every bare URL on the same line is reported and fixed, while
  trailing prose delimiters such as a closing parenthesis stay outside the fix
  range.
- Fixes `MD059` so each prohibited Markdown link text on a line is reported with
  a text-specific range, and longer inline code spans are still ignored.
- Speeds up large-document lint and fix paths by caching code-line membership in
  `DocumentContext` and dispatching lint evaluation through the static rule
  registry.
- Adds rule-local, document-level, and fix-idempotence regression tests for the
  precision fixes.
- Records the `v0.12.3` issue inventory, performance before/after numbers, and
  test hardening evidence in OpenSpec before release.

## v0.12.2

- Adds Windows to the normal CI matrix with locked `cargo check`, `cargo fmt`,
  and workspace test coverage before release.
- Replaces the normal CI hand-written Cargo cache with `Swatinem/rust-cache`
  so CI, release preflight, and release workflows use the same cache family.
- Adds an AST lint guard that fails when Windows CI coverage or Rust cache
  strategy drifts out of the intended release gates.
- Keeps GitHub Release and crates.io publication on the single Ubuntu release
  runner while treating Windows compatibility as pre-release CI responsibility.
- Records the `v0.12.2` false-positive and CI hardening inventory so remaining
  portability and parser-depth work is tracked outside the patch release.

## v0.12.1

- Fixes a false positive where `MD029` shared ordered-list numbering across
  nested indentation levels.
- Fixes a false positive where `MD034` treated URLs inside HTML attributes as
  bare URLs.
- Fixes a false positive where `MD014` treated LaTeX display math delimiters
  (`$$`) as command prompts.
- Fixes a false positive where `MD046` treated indented Mermaid diagram lines
  inside fenced code blocks as indented code blocks.
- Adds a regression guard so `MD055` does not treat HTML link separators or
  Mermaid edge labels inside fenced code blocks as Markdown tables.
- Fixes `MD056` and `MD058` to evaluate parsed Markdown table blocks instead of
  pipe-like text in HTML, prose, or fenced code blocks.
- Consolidates fenced code block context across heading, list, whitespace, link,
  image, HTML, and fence rules so nested shorter fences are not misclassified.
- Fixes false positives where `MD059` inspected HTML anchor text or treated
  descriptive Markdown link text containing `link` as non-descriptive.
- Adds file-level false-positive regression coverage for mixed Markdown
  documents with HTML, tables, math, nested lists, reference links, and fenced
  Mermaid/code examples.
- Aligns `MD009`, `MD010`, `MD013`, `MD014`, and `MD044` context handling with
  upstream configuration defaults for code blocks, code spans, headings, tables,
  hard-break spaces, and long unbreakable tokens.
- Updates GitHub Action examples and distribution docs to the current release line.
- Archives the completed MCP workspace tools OpenSpec change.

## v0.12.0

- Promotes `kml-mcp` from a text-only prototype to a workspace-scoped MCP adapter.
- Adds `check_file`, `check_directory`, `fix_file_preview`, and explicit `fix_file_apply` tools.
- Enforces workspace-root path boundaries, relative paths, symlink rejection, gitignore-aware directory scans, and non-UTF-8 file errors.
- Documents Codex, Claude Code, and Antigravity stdio configuration examples.

## v0.11.1

- Clarifies the README rule map states so implemented checks and conservative safe-fix subsets are not presented as unfinished work.
- Replaces unsafe fix `Not planned` states with `Needs triage` or `Not applicable` to better reflect the current safety policy.
- Updates dependency lockfile entries with `make update`.

## v0.11.0

- Adds an official root GitHub Action for running `kml` in CI from release tags.
- Adds shared action install/run scripts with crates.io and local-path install modes.
- Adds `make action-smoke` plus CI and release workflow smoke coverage for the action channel.
- Documents distribution channel status, official support policy, and deferred wrapper channels.

## v0.10.0

- Adds a dedicated layout formatter API with `format_markdown()`, `FormatOptions`, and `FormatResult`.
- Changes `kml fmt` from a `fix` alias into an indentation/newline formatter with editor-friendly exit code `0` on successful formatting.
- Normalizes CRLF/CR line endings, final newlines, repeated blank lines, structural blank lines, and safe list indentation/list-marker spacing.
- Keeps semantic/style rewrites out of formatter scope, including paragraph reflow, heading/emphasis/link/table style conversion, trailing-space removal, and unsafe fixes.
- Adds formatter idempotence, stdin contract, and CLI exit-code coverage.

## v0.9.0

- Adds fix safety metadata to public diagnostics so consumers can distinguish `safe` and `unsafe` candidates.
- Keeps default `fix`, `fmt`, and `check --fix` behavior safe-only.
- Adds `--unsafe` CLI mode with `[Y/n]` confirmation for interactive runs and `--unsafe --yes` for non-interactive automation.
- Adds an unsafe `MD036` candidate that converts emphasis-only section labels into ATX headings when explicitly opted in.
- Extends JSON and dashboard reporting with safe/unsafe fix visibility.

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
