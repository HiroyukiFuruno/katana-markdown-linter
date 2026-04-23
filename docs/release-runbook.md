# Release Runbook

## Purpose

This runbook covers the release checks needed before publishing `katana-markdown-linter` to crates.io.

## Preflight Checklist

1. Confirm `Cargo.toml` metadata is still correct:
   - `license = "MIT"`
   - `readme = "README.md"`
   - `repository = "https://github.com/HiroyukiFuruno/katana-markdown-linter"`
   - `description`, `keywords`, and `categories` are still accurate
2. Confirm package contents are limited to source, manifest, README, license, and other intentional files.
3. Run local validation:
   - `cargo fmt --all --check`
   - `cargo test --all-features`
   - `cargo clippy --all-targets --all-features -- -D warnings`
   - `cargo package --locked --allow-dirty`
   - `cargo publish --dry-run --locked --allow-dirty`
4. Confirm the installed binary path:
   - `cargo install --path . --bin kml`

## Publish Failure Recovery

### `cargo package` fails

- Fix the reported manifest or packaging issue.
- Re-run `cargo package --locked --allow-dirty`.

### `cargo publish --dry-run` fails

- Fix the packaging or dependency issue.
- Re-run the full preflight sequence before attempting publish again.

### `cargo publish` is rejected by crates.io

- If the version was not accepted, bump `version` in `Cargo.toml`.
- Re-run `cargo publish --dry-run` before the next publish attempt.

### Installed binary is missing or renamed

- Verify `[[bin]] name = "kml"` remains unchanged.
- Re-run the install check with `cargo install --path . --bin kml`.

### Incorrect files were packaged

- Update the `include` list in `Cargo.toml`.
- Re-run `cargo package` and the dry-run publish check.
