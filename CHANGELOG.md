# Changelog

## v0.17.7

- Clarifies that unsafe fixes can be used with both `kml fix` and
  `kml check --fix` through explicit `--unsafe --yes` opt-in.
- Adds `kml check --fix --unsafe --yes` to the README CLI examples.
- Adds CLI contract coverage proving `check --fix --unsafe --yes` applies an
  unsafe `MD036` fix while keeping JSON command identity as `check`.
- Updates `kml check --help` so the unsafe fix opt-in is discoverable from the
  command-specific help.

## v0.17.6

- Treats bare `kml` as global help, keeping `kml help`, `kml --help`,
  `kml -h`, command-specific help, and all version aliases on the help path.
- Adds localized help output, including `--locale ja help`,
  `kml check --help --locale ja`, and help text that explains `--locale`
  changes diagnostic and help language.
- Accepts official markdownlint config aliases such as `first-line-h1`,
  `first-line-heading`, `no-duplicate-heading`, and `no-inline-html`, plus
  `integer|integer[]` rule properties such as `MD022.lines_below`.
- Stops before linting when configuration validation fails and reports how to
  fix the config or explicitly continue with `--ignore-config-errors`.

## v0.17.5

- Adds global and command-specific CLI help for `kml help`, `kml --help`,
  `kml -h`, and `kml <command> --help`.
- Adds the `kml -v` version alias alongside `kml version`, `kml --version`,
  and `kml -V`.
- Updates the release flow so Homebrew tap formulae are published from verified
  release assets and checked during post-release verification.

## v0.17.4

- Fixes npm and PyPI wrapper binary caches so cached `kml` binaries are scoped
  by package version and target platform.
- Extends wrapper smoke coverage with stale-cache regression checks.
- Hardens post-release verification so it fetches a missing local release tag
  and checks release asset names without a pipe that can stall.

## v0.17.3

- Adds a full PyPI wrapper README and project metadata so the PyPI project page
  shows install, `uvx`, supported platform, and wrapper contract details.
- Adds `make pypi-package-check` to verify the PyPI README, metadata, source
  distribution, wheel, and wheel long description before publication.
- Fixes the Release workflow so merging a `release/vX.Y.Z` pull request
  publishes GitHub Release, crates.io, npm, and PyPI from the same run.
- Removes tag-push release execution to prevent partial publication before npm
  and PyPI wrapper jobs run.
- Carries the npm README and metadata closeout into a fresh patch version after
  `v0.17.2` was already published to GitHub Releases and crates.io.

## v0.17.2

- Adds an npm package README so the registry page shows install, `npx`, and
  basic `kml check` examples.
- Adds npm search and support metadata while keeping the wrapper dependency
  surface empty.
- Adds an npm package check that verifies required metadata and tarball contents
  before release publication.
- Carries the `v0.17.1` npm trusted publishing closeout into a fresh patch
  release so npm publication can be verified with `npm view` and `npx`.

## v0.17.1

- Promotes the npm and PyPI wrappers to official install channels after public
  registry and wrapper launch verification.
- Removes npm wrapper publication from the temporary token path and relies on
  GitHub Actions trusted publishing for the normal release workflow.
- Extends `make release-verify` to check npm, PyPI, wrapper launch output, and
  Homebrew formula evidence in addition to GitHub Release and crates.io state.
- Updates distribution docs, release runbook, and quality gate docs to match the
  published wrapper and Homebrew closeout flow.

## v0.17.0

- Adds standalone `kml` binary archives for Linux x86_64, macOS x86_64,
  macOS arm64, and Windows x86_64, each with a neighboring SHA-256 checksum.
- Extends local release checks and the Release workflow with shared binary
  packaging, archive extraction smoke tests, Homebrew formula rendering, and
  post-release binary asset verification.
- Adds thin npm and Python wrapper sources plus local wrapper smoke coverage.
  Wrapper publication remained deferred until registry publisher setup and
  explicit workflow publish flags were ready.
- Documents the binary archive install path, the separated Homebrew tap update
  flow, and the wrapper publication state without listing unpublished wrappers
  as official install channels.

## v0.16.2

- Adds document-level answer fixture evaluation for `kml check --fix`,
  comparing fixed output byte-for-byte against reviewed answer files and
  rejecting answer fixtures with second-pass fix changes.
- Locks the release gate to a 250-file corpus: 200 public repository Markdown
  samples plus 50 original composite regression documents, using the default
  rule set with line length disabled as non-fix noise.
- Fixes `MD013` line-length evaluation so Unicode at the configured boundary
  cannot panic during default-rule document evaluation.
- Wires `make document-answer-fix` into local release checks and GitHub release
  workflows while keeping the `v0.17.0` distribution expansion deferred.

## v0.16.1

- Expands supported locales to match KatanA: English, Japanese, Simplified
  Chinese, Traditional Chinese, Korean, Portuguese, French, German, Spanish,
  and Italian.
- Localizes rule descriptions for Rust APIs, `kml rule`, and MCP
  `rule_list` / `rule_get` while preserving canonical English descriptions in
  structured responses.
- Adds localized rule Markdown documentation for `rule_doc_get` and coverage
  gates that reject missing translations or English-copy localized content.
- Moves locale text into JSON catalogs and adds KatanA-style AST checks for
  locale catalog consistency, generated rule-doc labels, and rule-doc titles.

## v0.16.0

- Adds `kml config schema`, exposing a generated JSON Schema for
  `.markdownlint.json` and `.markdownlint.jsonc` with rule descriptions,
  property types, enum values, and defaults.
- Adds `kml lsp`, a stdio Language Server Protocol entrypoint for Markdown
  diagnostics, document formatting, range formatting, and safe quick fixes.
- Documents editor integration for VS Code, Zed, and Neovim, and wires
  configuration validation through the same schema-derived rule metadata used
  by the CLI and MCP validation tool.

## v0.15.1

- Fixes `MD046` false positives where continuation prose under ordered or nested
  list items was mistaken for indented code and rewritten into fenced code.
- Fixes `MD052` false positives for changelog headings such as
  `## [0.1.2] - ...` when shortcut reference syntax is enabled.
- Splits shared list-continuation and `MD052` shortcut-reference helpers so the
  regression coverage stays focused and maintainable.

## v0.15.0

- Adds the self-hosted `kml-mcp-remote` binary for MCP Streamable HTTP clients.
  The remote server is intentionally text-only in this release and exposes
  `check_text`, `fix_text`, `config_validate`, `rule_list`, `rule_get`, and
  `rule_doc_get`.
- Requires bearer authentication by default for the remote server and adds
  request body, timeout, and concurrency limits. Anonymous read-only operation
  is available only through an explicit environment opt-in and still exposes no
  workspace tools.
- Adds `make mcp-remote-build` and `make mcp-remote-smoke`, wiring the remote
  smoke into release checks, release preflight, and release publication gates.
- Documents the boundary between local stdio MCP, the MCPB/Registry package,
  and self-hosted remote transport. Workspace-backed remote file access remains
  deferred until authentication, tenant boundary, and audit requirements are
  implemented.

## v0.14.0

- Adds the `katana-markdown-linter-0.14.0.mcpb` release artifact for the
  `kml-mcp` stdio server, including the bundled binary, MCPB manifest, SHA-256
  checksum, and artifact smoke test.
- Adds repository `server.json` metadata and release-time rendering so MCP
  Registry publication points at the GitHub Release MCPB artifact with its
  computed `fileSha256` value.
- Wires MCPB smoke testing and MCP Registry metadata validation into
  `make release-check`, release preflight, and the Release workflow.
- Documents MCPB publication, Registry publication, retry policy, and
  post-publication verification while keeping remote MCP transport out of this
  release.

## v0.13.0

- Completes MCP Registry and hub distribution planning without publishing
  registry metadata yet. The selected package targets are MCPB from GitHub
  Releases and an OCI image on GHCR, with npm and PyPI wrappers deferred.
- Adds `docs/mcp-distribution-plan.md` and extends `docs/mcp-server.md` with a
  `server.json` draft plus a registry security checklist for workspace-safe
  stdio use.
- Updates the OpenSpec roadmap and `v0.13.0` task ledger so publication work is
  explicitly handed off to `v0.14.0`, while remote MCP transport remains split
  into `v0.15.0`.
- Extends release preflight triggers to include README, docs, and OpenSpec
  changes so required checks run for documentation-only release planning PRs.

## v0.12.21

- Excludes reserved and normally ignored directories from recursive CLI input by
  default, including `node_modules`, VCS metadata, build outputs, and cache
  directories. `--include-reserved` restores reserved paths, while
  `--include-ignored` allows explicitly requested ignored directories such as
  `.agents`.
- Adds JSON fix details for `fix --output json` and `check --fix --output json`
  so automated reviews can map each applied rewrite back to its rule and source
  range.
- Completes a KatanA 524-file check/fix sweep with every check result, every fix
  result, and all 82 fix hunks reviewed. The sweep found and fixed an `MD007`
  bad-fix case for unordered child lists under ordered parents, then re-ran the
  review with release-blocking check/fix findings at zero.
- Closes the remaining diagnostic-only rule map by marking author-intent cases
  as manual-required in README, fixture metadata, generated dashboard, and fix
  feasibility docs.
- Records external KatanA dogfood evidence with release-blocking issues at zero.

## v0.12.20

- Adds an internal cache for exported rule metadata so `available_rules()` keeps
  returning an owned `Vec<RuleMeta>` without rebuilding it from the structured
  catalog on every call.
- Fixes the `api_rule_catalog` strict performance regression found after
  `v0.12.19`; the case moved from 0.013 ms / 1.47x over baseline to about
  0.006 ms in the refreshed baseline.
- Refreshes `tests/fixtures/perf-baseline.json` after the catalog hot-path fix
  and documents the `v0.12.20` performance snapshot, public-confidence result,
  and optional cross-tool skipped evidence.

## v0.12.19

- Adds a safe fix to `MD003` (`heading-style`) that converts setext H1/H2
  headings to ATX H1/H2 headings while preserving the heading text and source
  replacement range.
- Keeps front matter delimiters, standalone horizontal rules, and code blocks
  outside the `MD003` safe-fix scope.
- Keeps `MD028` (`no-blanks-blockquote`) diagnostic-only because the official
  repairs require choosing whether adjacent blockquotes are separate quotes or
  one continuous quote. The fixture matrix and rule map now mark that case as
  requiring manual intent.

## v0.12.18

- Fixes `MD029` (`ordered-list-marker`) so nested unordered-list items do not
  reset parent ordered-list numbering, and adds regression tests for both false
  positives and fix behavior.
- Adds a safe fix to `MD056` (`table-column-count`) that pads short table rows
  with empty cells while preserving each row's leading/trailing pipe style.
- Keeps `MD056` diagnostic-only for overflow rows and rows marked unsafe,
  preventing data-loss risk while making short-row repair safe.

## v0.12.17

- Adds `FixDetail { rule_id, range, applied }` as a new public type and extends
  `FixResult` with a `details: Vec<FixDetail>` field. `fix_with_results` and
  `fix_with_results_including_unsafe` now populate `details` with one entry per
  candidate fix, recording whether it was applied or skipped due to a range
  conflict. `fix()` accumulates details across all passes. Existing
  `applied_fixes` and `content` fields are unchanged (Issue #43).
- Adds unit and integration tests that lock in the behaviour of MD051
  `github_heading_slug` for emoji and CJK characters: emoji are silently removed
  (matching GitHub's anchor generation), CJK characters are preserved, and no
  false positives are emitted for links to emoji-mixed or CJK headings.

## v0.12.16

- Adds a safe fix to MD046 (`code-block-style`): indented code blocks are now
  auto-converted to fenced blocks when a file contains mixed fenced and indented
  styles. Emits one diagnostic per indented block group (split at blank lines)
  with a `fix_info` that strips 4 leading spaces and wraps the block with
  triple-backtick fences. Adds MD046 to `is_safe_fix_rule` allowlist.
- Extends MD034 (`no-bare-urls`) to detect `ftp://`, `ftps://`, and `mailto:`
  bare URLs in addition to the existing `http://` and `https://` schemes.
  Existing suppression (code spans, inline links, reference definitions, HTML
  attributes) applies equally to all new schemes.
- Replaces O(n) linear scan in `MD034::is_ignored_url` for `inline_code_spans`,
  `inline_links`, and `reference_definitions` lookups with `partition_point`
  binary search, reducing per-URL ignore check from O(n) to O(log n + k) where
  k is the number of spans on the same line (same technique as v0.12.14).

## v0.12.15

- Removes the unused `_line_index: usize` parameter from `scan::inside_code_span`
  (dead after the v0.12.14 `partition_point` rewrite) and updates all three call
  sites (`html.rs`, `links/mod.rs`, `links/autolink.rs`), eliminating the
  leading-underscore suppression workaround.
- Replaces the defensive `.get(idx).copied().unwrap_or(false)` chain in all four
  inline extractors with direct `code_line_flags[idx]` indexing; the invariant is
  guaranteed by `build_code_line_flags(lines.len(), ...)`.
- Adds a safe fix to MD052 (`reference-links-images`): collapsed references such
  as `[ref][]` and `![alt][]` can now be auto-fixed by deleting the trailing `[]`,
  producing `[ref]` and `![alt]` respectively. Sets `is_fixable = true` in the
  rule catalog so CLI `--fix`, MCP `apply_fix`, and API surfaces advertise the
  new fix capability.
- Adds MD052 to `is_safe_fix_rule` allowlist so the safe fix is applied in the
  default `kml fix` pass without requiring unsafe-fix opt-in.
- Updates MD054 fixture `inline_collapsed_reference_when_disabled` to explicitly
  disable MD052 (`"MD052": false`) so the fixture tests MD054 behavior in
  isolation without conflict from the new MD052 fix.

## v0.12.14

- Replaces O(b) `line_in_blocks()` linear scan in all four inline extractors
  (`extract_inline_code_spans`, `extract_inline_html_elements`,
  `extract_inline_links`, `extract_reference_definitions`) with the pre-built
  `code_line_flags` boolean index already held by `DocumentContext`, eliminating
  per-line `code_blocks.iter().any()` traversal.
- Replaces O(s) `inside_code_span()` linear scan with a `partition_point`
  binary search on the sorted code-span list, reducing per-character-position
  cost from O(s) to O(log s) in the HTML-element and link parsers.
- Removes the `"`".repeat(marker_len)` String allocation in the inline code-span
  scanner, replacing it with a zero-allocation `find_closing_marker` byte scan.
- Updates coverage baseline from 880 to 881 to account for the new
  `find_closing_marker` helper (net line increase with no precision regression).
- Refreshes performance baseline: `context_inline_token_index_large_document`
  drops from 10.7 ms to ~0.9 ms (≈12×); inline-code-heavy and link-heavy
  corpus benchmarks improve by 3–7× with all ratios ≤ 1.40×.

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
