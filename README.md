# katana-markdown-linter

`katana-markdown-linter` is a Rust library-first Markdown lint engine with an optional `kml` CLI.

## Library API

Use the crate directly when embedding linting into another Rust application.

- `lint(content, options)`
- `fix(content, options)`
- `available_rules()`
- `implemented_rules()`
- `missing_rules()`
- `MarkdownLintConfig`

Minimal embedding examples are available under [`examples/`](examples/):

- `embedding.rs`: string checks, file tree checks, string fixes, config loading

## CLI Install

Install the bundled CLI with Cargo:

```bash
cargo install katana-markdown-linter
```

The binary target is `kml`.

## CLI Usage

```bash
kml check
kml fix
kml fmt
kml check --fix
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
kml rule MD013
kml config file
kml config get --output json
kml version
kml fix --config .markdownlint.json README.md
kml init-config
```

When no files are provided, `kml check`, `kml fix`, and `kml fmt` recursively process Markdown files under the current directory. Use `--file` to make single-file intent explicit.

`check` exits with `1` when lint violations are found. `check --fix`, `fix`, and `fmt` apply safe fixes and exit with `1` if violations remain after rewriting. Filesystem or configuration errors exit with `2`.

`--output json` is the preferred JSON output flag. `--format json` remains a compatibility alias.

`--stdin` reads Markdown from standard input. `check --stdin` reports diagnostics against `<stdin>`; `fix --stdin` and `fmt --stdin` write fixed Markdown to stdout.

Directory scans respect gitignore files by default. Use `--no-ignore` to include ignored paths. `--exclude` filters discovered files; explicit files are kept unless `--force-exclude` is also set.

## Rule Map

kml ships lint checks for the active markdownlint rule catalog. The table below
matches the public catalog returned by `available_rules()` and
`kml rule --output json`. Run `kml rule MD013` to inspect one rule with its
upstream documentation URL.

All rows are lint-supported. `Safe fix` only means kml can rewrite at least one
violation form automatically; rules marked `no` still report diagnostics.

| Rule | Safe fix |
| --- | --- |
| `MD001` | no |
| `MD003` | no |
| `MD004` | yes |
| `MD005` | yes |
| `MD007` | yes |
| `MD009` | yes |
| `MD010` | yes |
| `MD011` | yes |
| `MD012` | yes |
| `MD013` | no |
| `MD014` | yes |
| `MD018` | yes |
| `MD019` | yes |
| `MD020` | yes |
| `MD021` | yes |
| `MD022` | yes |
| `MD023` | yes |
| `MD024` | no |
| `MD025` | no |
| `MD026` | yes |
| `MD027` | yes |
| `MD028` | no |
| `MD029` | yes |
| `MD030` | yes |
| `MD031` | yes |
| `MD032` | yes |
| `MD033` | no |
| `MD034` | yes |
| `MD035` | no |
| `MD036` | no |
| `MD037` | yes |
| `MD038` | yes |
| `MD039` | yes |
| `MD040` | yes |
| `MD041` | no |
| `MD042` | no |
| `MD043` | no |
| `MD044` | yes |
| `MD045` | no |
| `MD046` | no |
| `MD047` | yes |
| `MD048` | no |
| `MD049` | yes |
| `MD050` | yes |
| `MD051` | yes |
| `MD052` | no |
| `MD053` | yes |
| `MD054` | yes |
| `MD055` | no |
| `MD056` | no |
| `MD058` | yes |
| `MD059` | no |
| `MD060` | no |

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

```bash
cargo build --bin kml-mcp --features mcp --locked
cargo run --bin kml-mcp --features mcp --locked
```

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
