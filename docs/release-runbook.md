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
   - `make release-check VERSION=vX.Y.Z`
4. If validating upstream drift locally, clone upstream docs and run:
   - `KML_UPSTREAM_MARKDOWNLINT_DOC_DIR=/path/to/markdownlint/doc make upstream-drift`
5. Confirm the installed binary path:
   - `cargo install --path . --bin kml`

## CI/CD Release Flow

The Release workflow is defined in `.github/workflows/release.yml`.

1. Confirm `Cargo.toml` `package.version` is the intended version.
2. Confirm `CHANGELOG.md` has a `## vX.Y.Z` section.
3. Run the workflow manually from GitHub Actions with:
   - `version`: `X.Y.Z` or `vX.Y.Z`
   - `publish_crate`: `false` for GitHub Release only, `true` for GitHub Release plus crates.io
4. The workflow validates:
   - Cargo version equals release version
   - `make fmt-check`
   - `cargo test --all-features --locked`
   - upstream markdownlint drift gate
   - `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
   - `cargo publish --dry-run --locked --allow-dirty`
   - `cargo install --path . --locked --bin kml`
5. The workflow creates or updates:
   - Git tag `vX.Y.Z`
   - GitHub Release
   - `.crate` package artifact
   - `.sha256` checksum

Tag push flow is also supported. Pushing `vX.Y.Z` runs the same gates and creates or updates the GitHub Release, but it does not publish to crates.io. Use manual dispatch with `publish_crate: true` when crates.io publication is intended.

## Required Secrets

- `CARGO_REGISTRY_TOKEN`: crates.io API token used only when manual dispatch sets `publish_crate: true`.

## Quality Gates and Branch Protection

See `docs/quality-gates.md` for the authoritative mapping between local `make` targets, CI required checks, and branch protection.

Current `main` branch protection requires:

- `Test and Build (macos-latest)`
- `Test and Build (ubuntu-latest)`

If workflow job names are changed, update branch protection in the same change. Direct pushes to `main` are blocked for non-admin users; admin bypasses should be treated as exceptions and verified by CI immediately after push.

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

### Release workflow fails before GitHub Release creation

- Fix the failed quality gate.
- Re-run the workflow with the same version.

### Upstream drift gate fails

- Inspect the reported markdownlint rule drift.
- Update local rule metadata, config properties, fixture matrix, or allowlist intentionally.
- Re-run `make upstream-drift` with the same upstream docs checkout.

### Coverage blocking gate fails

- Add tests for the newly uncovered paths.
- If the gap is intentional, update `scripts/ci/coverage-baseline.txt` in the same review and explain why the baseline increased.

### GitHub Release was created but crates.io publish failed

- Check whether the version exists on crates.io.
- If it was not published, fix the token or package issue and re-run with the same version.
- If it was partially published, do not reuse the same version for changed content; bump `Cargo.toml` version.

### Installed binary is missing or renamed

- Verify `[[bin]] name = "kml"` remains unchanged.
- Re-run the install check with `cargo install --path . --bin kml`.

### Incorrect files were packaged

- Update the `include` list in `Cargo.toml`.
- Re-run `cargo package` and the dry-run publish check.
