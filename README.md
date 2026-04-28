# katana-markdown-linter

`katana-markdown-linter` is a Rust library-first Markdown lint engine with an optional `kml` CLI.

## Library API

Use the crate directly when embedding linting into another Rust application.

- `lint(content, options)`
- `fix(content, options)`
- `available_rules()`
- `localized_available_rules(language_code)`
- `implemented_rules()`
- `missing_rules()`
- `rule_catalog()`
- `localized_rule_catalog(language_code)`
- `resolve_locale_code(language_code)`
- `resolve_locale_code_or(language_code, fallback)`
- `localized_rule_description(rule_id, fallback_description, language_code)`
- `supported_locales()`
- `MarkdownLintConfig`
- `MarkdownLintConfig::to_lint_options()`

`available_rules()` returns canonical English metadata. For user-facing rule
catalogs, call `localized_available_rules(language_code)`,
`localized_rule_catalog(language_code)`, `RuleMeta::localized_description(...)`,
or `localized_rule_description(...)` so applications can pass UI language codes
without reimplementing kml's fallback policy.

`Locale` is `#[non_exhaustive]` from v0.6.0. Consumers that match on `Locale`
should include a wildcard arm and prefer `resolve_locale_code(...)` or
`resolve_locale_code_or(...)` for UI language strings.

Minimal embedding examples are available under [`examples/`](examples/):

- `embedding.rs`: string checks, file tree checks, string fixes, config loading and config-to-options conversion

## CLI Install

Install the bundled CLI with Cargo:

~~~bash
cargo install katana-markdown-linter
~~~

The binary target is `kml`.

## GitHub Actions

Use the repository action to run `kml` in CI without writing install steps:

~~~yaml
- uses: actions/checkout@v5
- uses: HiroyukiFuruno/katana-markdown-linter@v0.14.0
  with: { version: "0.14.0", command: check, paths: "README.md\ndocs", config: .markdownlint.json }
~~~

Pin the action tag and `version` together for reproducible runs. The action
installs `kml` from crates.io by default. Repository-local smoke tests can set
`install-source: path` and `path: .`.

`extra-args` accepts one argument per line, so flags that take values must use
separate lines.

## CLI Usage

~~~bash
kml check
kml check --locale en
kml check -l ja
kml fix
kml fmt
kml check --fix
kml fix --unsafe --yes README.md
kml check README.md
kml check --file README.md
kml check --output json "docs/**/*.md"
kml check --format json "docs/**/*.md"
kml check --stdin
kml fix --stdin
kml check --include "**/*.md" --exclude "target/**"
kml check --no-ignore --force-exclude --exclude "vendor/**" vendor/README.md
kml check --statistics --quiet
kml fix --diff README.md
kml rule
kml rule --locale ja
kml rule MD013
kml rule MD013 --locale ja --output json
kml config file
kml config get --output json
kml version
kml fix --config .markdownlint.json README.md
kml init-config
~~~

When no files are provided, `kml check`, `kml fix`, and `kml fmt` recursively process Markdown files under the current directory. Use `--file` to make single-file intent explicit.

`check` reports diagnostics and exits with `1` when lint violations are found.
`fix` and `check --fix` apply safe lint-driven fixes and exit with `1` if
violations remain after rewriting. `fmt` is a layout formatter for indentation
and newline normalization; it exits with `0` after successful formatting even
when unrelated lint diagnostics would still be reported by `check`. Filesystem
or configuration errors exit with `2`.

`fmt` currently normalizes CRLF/CR line endings to LF, final newlines, repeated
blank lines, blank lines around headings/fences/lists/tables, and safe list
indentation/list-marker spacing. It does not reflow paragraphs, change heading
or emphasis style, change URL/table style, remove trailing spaces, or apply
unsafe fixes by default.

Unsafe fixes require explicit opt-in. Interactive use prompts with `[Y/n]`;
non-interactive use must pass `--unsafe --yes`.

`--output json` is the preferred JSON output flag. `--format json` remains a compatibility alias.

`--stdin` reads Markdown from standard input. `check --stdin` reports diagnostics against `<stdin>`; `fix --stdin` writes fixed Markdown to stdout; `fmt --stdin` writes formatted Markdown only to stdout.

Directory scans respect gitignore files by default. Use `--no-ignore` to include ignored paths. `--exclude` filters discovered files; explicit files are kept unless `--force-exclude` is also set.

`--locale <locale>` and `-l <locale>` select user-facing CLI message locale.
Supported values currently resolve to English (`en`, `en-US`) or Japanese
(`ja`, `ja-JP`). When omitted, `kml` reads OS locale environment variables and
falls back to English if the locale is unavailable or unsupported. Explicit
unsupported locales fail with a CLI error. `--local` is accepted as a
backward-compatible alias for v0.4.0 users.

`kml rule` and `kml rule <id>` also honor `--locale`. Text output uses localized
rule descriptions, and JSON output includes both `description` (localized) and
`english_description` (canonical English).

## Rule Map

kml ships lint checks for the active markdownlint rule catalog. The rule IDs
below match the public catalog returned by `available_rules()` and
`kml rule --output json`; state columns describe fixture-backed implementation
status. Run `kml rule MD013` to inspect one rule with its upstream documentation
URL.

The short version:

| Capability | Status |
| --- | --- |
| Check coverage | `Implemented` for all 53 active rules |
| Safe fix coverage | Safe fix available for 38 rules; diagnostic-only for 15 rules |
| Unsafe fix coverage | Explicit opt-in unsafe subset for 1 rule; remaining rules are either not applicable or need triage |
| Deleted upstream IDs | 7 historical IDs shown as `Deleted` with `-` fix states |

Safe fixes are intentionally conservative. `Implemented subset` means kml
rewrites fixture-locked, low-risk violation forms for that rule. `Diagnostic
only` means checks are implemented, but safe fixes are not provided.
`Needs triage` means an unsafe fix candidate has not been evaluated yet.
`Not applicable` means no separate unsafe candidate is currently identified
beyond the safe-fix policy. `Deleted` rows are historical markdownlint IDs that
are not part of the active upstream rule catalog.

<details>
<summary>Full fixture-backed rule matrix</summary>

| Rule | Check | Fix (safe) | Fix (unsafe) |
| --- | --- | --- | --- |
| `MD001` | Implemented | Diagnostic only | Needs triage |
| `MD002` | Deleted | - | - |
| `MD003` | Implemented | Diagnostic only | Needs triage |
| `MD004` | Implemented | Implemented subset | Not applicable |
| `MD005` | Implemented | Implemented subset | Not applicable |
| `MD006` | Deleted | - | - |
| `MD007` | Implemented | Implemented subset | Not applicable |
| `MD008` | Deleted | - | - |
| `MD009` | Implemented | Implemented subset | Not applicable |
| `MD010` | Implemented | Implemented subset | Not applicable |
| `MD011` | Implemented | Implemented subset | Not applicable |
| `MD012` | Implemented | Implemented subset | Not applicable |
| `MD013` | Implemented | Diagnostic only | Needs triage |
| `MD014` | Implemented | Implemented subset | Not applicable |
| `MD015` | Deleted | - | - |
| `MD016` | Deleted | - | - |
| `MD017` | Deleted | - | - |
| `MD018` | Implemented | Implemented subset | Not applicable |
| `MD019` | Implemented | Implemented subset | Not applicable |
| `MD020` | Implemented | Implemented subset | Not applicable |
| `MD021` | Implemented | Implemented subset | Not applicable |
| `MD022` | Implemented | Implemented subset | Not applicable |
| `MD023` | Implemented | Implemented subset | Not applicable |
| `MD024` | Implemented | Diagnostic only | Needs triage |
| `MD025` | Implemented | Implemented subset | Not applicable |
| `MD026` | Implemented | Implemented subset | Not applicable |
| `MD027` | Implemented | Implemented subset | Not applicable |
| `MD028` | Implemented | Diagnostic only | Needs triage |
| `MD029` | Implemented | Implemented subset | Not applicable |
| `MD030` | Implemented | Implemented subset | Not applicable |
| `MD031` | Implemented | Implemented subset | Not applicable |
| `MD032` | Implemented | Implemented subset | Not applicable |
| `MD033` | Implemented | Diagnostic only | Needs triage |
| `MD034` | Implemented | Implemented subset | Not applicable |
| `MD035` | Implemented | Implemented subset | Not applicable |
| `MD036` | Implemented | Diagnostic only | Implemented subset |
| `MD037` | Implemented | Implemented subset | Not applicable |
| `MD038` | Implemented | Implemented subset | Not applicable |
| `MD039` | Implemented | Implemented subset | Not applicable |
| `MD040` | Implemented | Implemented subset | Not applicable |
| `MD041` | Implemented | Diagnostic only | Needs triage |
| `MD042` | Implemented | Diagnostic only | Needs triage |
| `MD043` | Implemented | Diagnostic only | Needs triage |
| `MD044` | Implemented | Implemented subset | Not applicable |
| `MD045` | Implemented | Diagnostic only | Needs triage |
| `MD046` | Implemented | Implemented subset | Not applicable |
| `MD047` | Implemented | Implemented subset | Not applicable |
| `MD048` | Implemented | Implemented subset | Not applicable |
| `MD049` | Implemented | Implemented subset | Not applicable |
| `MD050` | Implemented | Implemented subset | Not applicable |
| `MD051` | Implemented | Implemented subset | Not applicable |
| `MD052` | Implemented | Implemented subset | Not applicable |
| `MD053` | Implemented | Implemented subset | Not applicable |
| `MD054` | Implemented | Implemented subset | Not applicable |
| `MD055` | Implemented | Implemented subset | Not applicable |
| `MD056` | Implemented | Diagnostic only | Needs triage |
| `MD057` | Deleted | - | - |
| `MD058` | Implemented | Implemented subset | Not applicable |
| `MD059` | Implemented | Diagnostic only | Needs triage |
| `MD060` | Implemented | Implemented subset | Not applicable |

</details>

## Configuration

The crate reads and writes markdownlint-compatible JSON and JSONC configuration files:

- `.markdownlint.json`
- `.markdownlint.jsonc`

Use `kml init-config` to create a default `.markdownlint.json`.

## Quality Gates

Use `make check` for the default local gate. It runs format, Clippy, AST lint, and tests.

Use `make dogfood` to run `kml` against this repository's Markdown documentation without modifying files. Use `make dogfood-fix` only when you intend to apply safe Markdown fixes. Archived OpenSpec documents are excluded from the default dogfood targets; run `make dogfood-archive` to check them explicitly.

Use `make examples` to compile the public Rust embedding examples.

Quality gate details, CI required checks, coverage modes, and release readiness are documented in [`docs/quality-gates.md`](docs/quality-gates.md).

Distribution channel status and deferral notes are documented in
[`docs/distribution.md`](docs/distribution.md).

## MCP Server

`kml-mcp` is an optional MCP server for agents and editors that want structured
library access over stdio while keeping the core crate independent from MCP
dependencies, with official MCP Registry and `.mcpb` bundle support.

Build or install it with the `mcp` feature:

~~~bash
cargo build --bin kml-mcp --features mcp --locked
cargo install katana-markdown-linter --locked --features mcp --bin kml-mcp
~~~

The server exposes text, config, rule metadata, and workspace-safe file tools:

- `check_text`
- `fix_text`
- `config_validate`
- `rule_list`
- `rule_get`
- `check_file`
- `check_directory`
- `fix_file_preview`
- `fix_file_apply`

Workspace paths must stay under the configured `--workspace-root`. File writes
are limited to `fix_file_apply` and require `apply: true`; no directory apply
tool is exposed.

Run `make mcp-stdio-smoke` to exercise the installed MCP server through
JSON-RPC stdio calls.

See [MCP server documentation](docs/mcp-server.md) and the earlier
[MCP integration evaluation](docs/mcp-integration-evaluation.md).

## Release Policy

- The library API is the primary contract.
- The CLI is a thin wrapper over the library.
- `Cargo.toml` package version is the release version source of truth.
- Run `make release-check VERSION=vX.Y.Z` before publication.
- Run `make release-github VERSION=vX.Y.Z` to create or update only the GitHub Release.
- Run `make release VERSION=vX.Y.Z` only when crates.io publication is intended.
- Run `make release-verify VERSION=vX.Y.Z` after publication to compare the tag target, GitHub Release target, and crates.io version.
- GitHub Releases require a signed annotated `vX.Y.Z` tag that GitHub reports as `Verified`.
- `make release` stops before dispatch when the requested version already exists on crates.io.
- crates.io publication requires the `CARGO_REGISTRY_TOKEN` GitHub secret.
