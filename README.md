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

`check` exits with `1` when lint violations are found. `check --fix`, `fix`, and `fmt` apply safe fixes and exit with `1` if violations remain after rewriting. Filesystem or configuration errors exit with `2`.

Unsafe fixes require explicit opt-in. Interactive use prompts with `[Y/n]`;
non-interactive use must pass `--unsafe --yes`.

`--output json` is the preferred JSON output flag. `--format json` remains a compatibility alias.

`--stdin` reads Markdown from standard input. `check --stdin` reports diagnostics against `<stdin>`; `fix --stdin` and `fmt --stdin` write fixed Markdown to stdout.

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
| Check coverage | `Supported` for all 53 active rules |
| Safe fix coverage | `Partial` for 38 rules; diagnostic-only for 15 rules |
| Unsafe fix coverage | `Partial` for 1 rule; explicit opt-in only |
| Deleted upstream IDs | 7 historical IDs shown as `Deleted` with `-` fix states |

Safe fixes are intentionally conservative. `Partial` means kml rewrites at least
one fixture-locked, low-risk violation form for that rule. `Not implemented`
means the rule is currently diagnostic-only. `Not planned` means unsafe rewriting
is outside the default safe-fix contract. `Deleted` rows are historical
markdownlint IDs that are not part of the active upstream rule catalog.

<details>
<summary>Full fixture-backed rule matrix</summary>

| Rule | Check | Fix (safe) | Fix (unsafe) |
| --- | --- | --- | --- |
| `MD001` | Supported | Not implemented | Not planned |
| `MD002` | Deleted | - | - |
| `MD003` | Supported | Not implemented | Not planned |
| `MD004` | Supported | Partial | Not planned |
| `MD005` | Supported | Partial | Not planned |
| `MD006` | Deleted | - | - |
| `MD007` | Supported | Partial | Not planned |
| `MD008` | Deleted | - | - |
| `MD009` | Supported | Partial | Not planned |
| `MD010` | Supported | Partial | Not planned |
| `MD011` | Supported | Partial | Not planned |
| `MD012` | Supported | Partial | Not planned |
| `MD013` | Supported | Not implemented | Not planned |
| `MD014` | Supported | Partial | Not planned |
| `MD015` | Deleted | - | - |
| `MD016` | Deleted | - | - |
| `MD017` | Deleted | - | - |
| `MD018` | Supported | Partial | Not planned |
| `MD019` | Supported | Partial | Not planned |
| `MD020` | Supported | Partial | Not planned |
| `MD021` | Supported | Partial | Not planned |
| `MD022` | Supported | Partial | Not planned |
| `MD023` | Supported | Partial | Not planned |
| `MD024` | Supported | Not implemented | Not planned |
| `MD025` | Supported | Partial | Not planned |
| `MD026` | Supported | Partial | Not planned |
| `MD027` | Supported | Partial | Not planned |
| `MD028` | Supported | Not implemented | Not planned |
| `MD029` | Supported | Partial | Not planned |
| `MD030` | Supported | Partial | Not planned |
| `MD031` | Supported | Partial | Not planned |
| `MD032` | Supported | Partial | Not planned |
| `MD033` | Supported | Not implemented | Not planned |
| `MD034` | Supported | Partial | Not planned |
| `MD035` | Supported | Partial | Not planned |
| `MD036` | Supported | Not implemented | Partial |
| `MD037` | Supported | Partial | Not planned |
| `MD038` | Supported | Partial | Not planned |
| `MD039` | Supported | Partial | Not planned |
| `MD040` | Supported | Partial | Not planned |
| `MD041` | Supported | Not implemented | Not planned |
| `MD042` | Supported | Not implemented | Not planned |
| `MD043` | Supported | Not implemented | Not planned |
| `MD044` | Supported | Partial | Not planned |
| `MD045` | Supported | Not implemented | Not planned |
| `MD046` | Supported | Not implemented | Not planned |
| `MD047` | Supported | Partial | Not planned |
| `MD048` | Supported | Partial | Not planned |
| `MD049` | Supported | Partial | Not planned |
| `MD050` | Supported | Partial | Not planned |
| `MD051` | Supported | Partial | Not planned |
| `MD052` | Supported | Not implemented | Not planned |
| `MD053` | Supported | Partial | Not planned |
| `MD054` | Supported | Partial | Not planned |
| `MD055` | Supported | Partial | Not planned |
| `MD056` | Supported | Not implemented | Not planned |
| `MD057` | Deleted | - | - |
| `MD058` | Supported | Partial | Not planned |
| `MD059` | Supported | Not implemented | Not planned |
| `MD060` | Supported | Partial | Not implemented |

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

MCP integration has been evaluated separately in
[`docs/mcp-integration-evaluation.md`](docs/mcp-integration-evaluation.md).
The current recommendation is an optional `kml-mcp` prototype that keeps the
core crate independent from MCP dependencies.

## MCP Server

`kml-mcp` is an optional, experimental MCP server for agents and editors that
want to call the library without shelling out to `kml`.

Build or run it with the `mcp` feature:

~~~bash
cargo build --bin kml-mcp --features mcp --locked
cargo run --bin kml-mcp --features mcp --locked
~~~

The prototype exposes read-only, text-first tools:

- `check_text`
- `fix_text`
- `config_validate`
- `rule_list`
- `rule_get`

`fix_text` returns fixed content only; it does not write files. File read/write
MCP tools are intentionally absent until workspace allowlist and dry-run
policies are implemented.

See [MCP server documentation](docs/mcp-server.md).

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
