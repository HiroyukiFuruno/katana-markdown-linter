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

## CLI Install

Install the bundled CLI with Cargo:

```bash
cargo install katana-markdown-linter
```

The binary target is `kml`.

## CLI Usage

```bash
kml check README.md
kml check --format json "docs/**/*.md"
kml fix --config .markdownlint.json README.md
kml init-config
```

`check` exits with `1` when lint violations are found. `fix` applies safe fixes and exits with `1` if violations remain after rewriting. Filesystem or configuration errors exit with `2`.

## Configuration

The crate reads and writes markdownlint-compatible JSON and JSONC configuration files:

- `.markdownlint.json`
- `.markdownlint.jsonc`

Use `kml init-config` to create a default `.markdownlint.json`.

## Release Policy

- The library API is the primary contract.
- The CLI is a thin wrapper over the library.
- Release validation should include `cargo fmt`, `cargo test`, `cargo clippy`, and publish dry-run checks before publication.
- `Cargo.toml` package version is the release version source of truth.
- GitHub Releases are created by the Release workflow with a `vX.Y.Z` tag.
- crates.io publication requires the `CARGO_REGISTRY_TOKEN` GitHub secret.
