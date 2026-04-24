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
- Release validation should include `cargo fmt`, `cargo test`, `cargo clippy`, and publish dry-run checks before publication.
- `Cargo.toml` package version is the release version source of truth.
- GitHub Releases are created by the Release workflow with a `vX.Y.Z` tag.
- crates.io publication requires the `CARGO_REGISTRY_TOKEN` GitHub secret.
