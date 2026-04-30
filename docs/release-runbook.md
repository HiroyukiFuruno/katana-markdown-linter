# Release Runbook

## Purpose

This runbook covers the release checks needed before publishing
`katana-markdown-linter` to crates.io and GitHub Releases.

## Preflight Checklist

Confirm `Cargo.toml` metadata is still correct:

- `license = "MIT"`
- `readme = "README.md"`
- `repository` points at the GitHub repository
- `description`, `keywords`, and `categories` are still accurate

Confirm package contents are limited to source, manifest, README, license, and other intentional files.

Run local validation:

~~~bash
make release-check VERSION=vX.Y.Z
~~~

For MCP distribution changes, the release check includes:

- `make mcpb-smoke VERSION=vX.Y.Z`
- `make server-json-validate VERSION=vX.Y.Z`

For binary distribution changes, the release check includes:

- `make binary-smoke VERSION=vX.Y.Z`
- `make homebrew-formula-check VERSION=vX.Y.Z`
- `make wrapper-smoke VERSION=vX.Y.Z`
- `make wrapper-publish-gate`

If validating upstream drift locally, clone upstream docs and run:

~~~bash
KML_UPSTREAM_MARKDOWNLINT_DOC_DIR=/path/to/markdownlint/doc make upstream-drift
~~~

Confirm the installed binary path:

~~~bash
cargo install --path . --bin kml
~~~

## CI/CD Release Flow

The Release workflow is defined in `.github/workflows/release.yml`.

Required sequence:

- Confirm `Cargo.toml` `package.version` is the intended version.
- Confirm `CHANGELOG.md` has a `## vX.Y.Z` section.
- Create and push a GitHub-verified signed annotated tag with `make release-tag VERSION=vX.Y.Z`.
- Dispatch the intended release command.
- Verify external state after publication with `make release-verify VERSION=vX.Y.Z`.

Release command responsibilities:

- `make release-github VERSION=vX.Y.Z` creates or updates the GitHub Release only.
- `make release VERSION=vX.Y.Z` creates or updates the GitHub Release and publishes to crates.io.

The workflow validates:

- Cargo version equals release version.
- Existing tag is an annotated signed tag that GitHub reports as `Verified`.
- `make fmt-check`
- `cargo test --all-features --locked`
- `make examples`
- `make mcp-build`
- `make mcp-remote-build`
- `make mcp-remote-smoke`
- `make mcpb-smoke`
- `make server-json-validate`
- `make action-smoke`
- standalone `kml` archive build and archive smoke for each supported target
- Homebrew formula rendering from release archive checksums
- upstream markdownlint drift gate
- `make lint`
- `cargo publish --dry-run --locked --allow-dirty`
- `cargo install --path . --locked --bin kml`

Normal pull request CI validates Windows with locked workspace build and test
commands before release. The Release workflow remains a single Ubuntu runner so
GitHub Release creation and crates.io publication cannot execute from multiple
OS jobs.

Normal CI, release preflight, and release publication all use
`Swatinem/rust-cache`. Each workflow uses an explicit `shared-key` to keep the
cache family and feature scope visible in logs.

The workflow creates or updates:

- GitHub Release
- `.crate` package artifact
- `.sha256` checksum
- `kml-vX.Y.Z-x86_64-unknown-linux-gnu.tar.gz` and checksum
- `kml-vX.Y.Z-x86_64-apple-darwin.tar.gz` and checksum
- `kml-vX.Y.Z-aarch64-apple-darwin.tar.gz` and checksum
- `kml-vX.Y.Z-x86_64-pc-windows-msvc.zip` and checksum
- `.mcpb` package artifact for `kml-mcp`
- `.mcpb.sha256` checksum
- rendered MCP Registry `server.json`

The `x86_64-apple-darwin` artifact uses GitHub Actions `macos-15-intel`
because the older `macos-13` image is no longer supported.

The root `action.yml` is the official GitHub Action channel from `v0.11.0`.
Release preflight must keep `make action-smoke` passing before publishing a tag.

Tag push flow is also supported. Pushing `vX.Y.Z` runs the same gates and creates or updates the GitHub Release, but it does not publish to crates.io. Use manual dispatch with `publish_crate: true` when crates.io publication is intended.

Manual GitHub Actions dispatch is still available, but the local `make`
targets are preferred because they create or verify the signed tag and check the
`CARGO_REGISTRY_TOKEN` secret before dispatch.

## MCPB And Registry Publication

Build and validate the MCPB artifact locally:

~~~bash
make mcpb-smoke VERSION=vX.Y.Z
make server-json-validate VERSION=vX.Y.Z
~~~

`make mcpb-smoke` builds `target/mcpb/katana-markdown-linter-X.Y.Z.mcpb`,
writes a SHA-256 checksum beside it, extracts the bundle, and runs the bundled
`kml-mcp` binary through the stdio smoke test.

`make server-json-validate` renders `target/mcpb/server.json` from the
repository `server.json` and the computed MCPB checksum. Publish the rendered
file, not the source template.

The Release workflow uploads the MCPB artifact, its checksum, and the rendered
`server.json` to the GitHub Release before publishing Registry metadata. It
authenticates with GitHub OIDC:

~~~bash
mcp-publisher login github-oidc
mcp-publisher publish target/mcpb/server.json
~~~

The MCP Registry version and MCPB artifact are immutable release outputs. If a
published artifact or checksum is wrong, bump to the next patch version instead
of replacing the published meaning of the same version.

## Homebrew Tap Update

The release workflow generates `target/homebrew/kml.rb` from the verified
binary assets. Apply that file to the `homebrew-katana` tap in a separate
branch after the GitHub Release exists:

~~~bash
cd /Users/hiroyuki_furuno/works/private/homebrew-katana
git switch -c release/kml-vX.Y.Z
mkdir -p Formula
cp /Users/hiroyuki_furuno/works/private/katana-markdown-linter/target/homebrew/kml.rb Formula/kml.rb
brew audit --strict --online Formula/kml.rb
brew test Formula/kml.rb
~~~

Push and review the tap change separately from the core release PR. Do not push
a tap update before the referenced release assets and checksums exist.

## Wrapper Publication

The npm and Python wrappers are thin launchers over GitHub Release binary
archives. They are not official install channels until all of these are true:

- npm and PyPI maintainer accounts exist for the package owner.
- The GitHub repository has a `pypi` environment for PyPI publication.
- npm trusted publishing is configured for `release.yml`.
- PyPI trusted publishing is configured for `release.yml` with the `pypi` environment.
- The release workflow is manually dispatched with wrapper publish flags enabled.
- `make wrapper-smoke VERSION=vX.Y.Z` succeeds against the release archive shape.

Current `v0.17.0` release notes and public documentation keep wrapper
publication deferred when these trusted publisher settings are absent.

Use these trusted publisher settings:

| Registry | Project or package | Owner | Repository | Workflow filename | Environment |
| --- | --- | --- | --- | --- | --- |
| npm | `katana-markdown-linter` | `HiroyukiFuruno` | `katana-markdown-linter` | `release.yml` | Leave empty unless npm is configured with a GitHub environment |
| PyPI | `katana-markdown-linter` | `HiroyukiFuruno` | `katana-markdown-linter` | `release.yml` | `pypi` |

The PyPI project name must match `wrappers/python/pyproject.toml`. Change the
wrapper metadata first if the package name is changed before publication.

## Required Secrets

- `CARGO_REGISTRY_TOKEN`: crates.io API token used only when manual dispatch sets `publish_crate: true`.

GitHub OIDC Registry authentication does not require a dedicated secret, but the
Release workflow must keep `id-token: write`.

For the first npm wrapper publication, npm can require a traditional publish
token because trusted publishing is configured from an existing package. If the
package does not exist yet, create a short-lived granular token with publish
permission, store it as `NPM_TOKEN`, publish the first wrapper version, then
configure npm trusted publishing and revoke the token.

## Quality Gates and Branch Protection

See `docs/quality-gates.md` for the authoritative mapping between local `make` targets, CI required checks, and branch protection.

Current `main` branch protection requires:

- `Test and Build (macos-latest)`
- `Test and Build (ubuntu-latest)`
- `Test and Build (windows-latest)`

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
- Do not rewrite a tag unless no GitHub Release exists, the version is not published on crates.io, and the only failure is the workflow run itself.
- `make release-tag` refuses to overwrite a remote tag whose target differs from the local tag.

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
- `make release` fails fast when the requested version already exists on crates.io.
- Use `make release-verify VERSION=vX.Y.Z` to compare the local tag target, GitHub Release title and target, and crates.io version after retry.

### MCP Registry publish failed

- Verify the GitHub Release contains the `.mcpb`, `.mcpb.sha256`, and rendered
  `server.json` assets.
- Re-run `make server-json-validate VERSION=vX.Y.Z` and inspect the rendered
  `fileSha256`.
- If the Registry version was not accepted, fix authentication or metadata and
  rerun the workflow with the same version.
- If the Registry version was accepted with a bad artifact reference, do not
  reuse the same version for changed content; bump `Cargo.toml` version.

### MCPB smoke fails

- Re-run `make mcpb-smoke VERSION=vX.Y.Z`.
- Inspect `mcpb/manifest.json`, `scripts/release/package-mcpb.sh`, and
  `scripts/ci/mcpb-smoke.py`.
- Keep the manifest aligned with local stdio execution; do not claim remote MCP
  transport.

### MCP remote smoke fails

- Re-run `make mcp-remote-smoke`.
- Inspect `src/bin/kml-mcp-remote.rs`, `src/bin/kml_mcp/remote/`, and
  `scripts/ci/mcp-remote-smoke.py`.
- Keep remote `tools/list` text-only unless workspace authentication, tenant
  boundary, explicit apply, and audit logging are implemented together.

### Release notes generation fails

- Add a non-empty `## vX.Y.Z` section to `CHANGELOG.md`.
- Re-run `scripts/release/release-notes.sh X.Y.Z` before dispatching the release workflow.

### Installed binary is missing or renamed

- Verify `[[bin]] name = "kml"` remains unchanged.
- Re-run the install check with `cargo install --path . --bin kml`.

### GitHub Action smoke fails

- Re-run `make action-smoke`.
- Inspect `action.yml`, `scripts/action/install-kml.sh`, and `scripts/action/run-kml.sh`.
- Keep the action scripts generic; do not add repository-specific lint policy outside action inputs.

### Incorrect files were packaged

- Update the `include` list in `Cargo.toml`.
- Re-run `cargo package` and the dry-run publish check.
