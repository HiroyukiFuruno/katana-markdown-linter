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

Confirm the requested version follows the published release line:

~~~bash
just VERSION=vX.Y.Z release-target-check
~~~

The check rejects suspicious jumps such as releasing `v0.18.7` when the latest
stable release is `v0.17.6`. Continue the patch line with `v0.17.7`, or start
the new minor line with `v0.18.0`.

Run local validation:

~~~bash
just VERSION=vX.Y.Z release-check
~~~

For MCP distribution changes, the release check includes:

- `just VERSION=vX.Y.Z mcpb-smoke`
- `just VERSION=vX.Y.Z server-json-validate`
- `just VERSION=vX.Y.Z mcp-binary-smoke`

For binary distribution changes, the release check includes:

- `just VERSION=vX.Y.Z binary-smoke`
- `just VERSION=vX.Y.Z mcp-binary-smoke`
- [ ] Run `just VERSION=vX.Y.Z editor-extension-check`
- `just VERSION=vX.Y.Z homebrew-formula-check`
- `just VERSION=v0.18.5 release-partial-publish-test`
- `just VERSION=vX.Y.Z wrapper-smoke`
- `just npm-package-check`
- `just npm-publish-target-check`
- `just pypi-package-check`
- `just wrapper-publish-gate`
- `just editor-publish-gate`

If validating upstream drift locally, clone upstream docs and run:

~~~bash
KML_UPSTREAM_MARKDOWNLINT_DOC_DIR=/path/to/markdownlint/doc just upstream-drift
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
- Merge a release PR from `release/vX.Y.Z`, or dispatch the intended local release command.
- Verify GitHub Release, crates.io, npm, PyPI, wrapper launch, and Homebrew formula state after publication with `just VERSION=vX.Y.Z release-verify`.

Release command responsibilities:

- `just VERSION=vX.Y.Z release-github` creates or updates the GitHub Release only.
- `just VERSION=vX.Y.Z release` creates or updates the GitHub Release and publishes to crates.io, npm, and PyPI.

Release PR merge responsibilities:

- A merged `release/vX.Y.Z` pull request creates the signed release tag.
- The same workflow run creates or updates the GitHub Release.
- The same workflow run publishes crates.io, npm, and PyPI so wrapper packages do not lag behind the crate.

The workflow validates:

- Cargo version equals release version.
- release target follows the published stable release line.
- Existing tag is an annotated signed tag that GitHub reports as `Verified`.
- `just fmt-check`
- `cargo test --all-features --locked`
- `just examples`
- `just mcp-build`
- `just mcp-remote-build`
- `just mcp-remote-smoke`
- `just mcpb-smoke`
- `just server-json-validate`
- `just action-smoke`
- `just npm-package-check`
- `just npm-publish-target-check`
- `just pypi-package-check`
- standalone `kml` archive build and archive smoke for each supported target
- Homebrew formula rendering from release archive checksums
- upstream markdownlint drift gate
- `just lint`
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
- matching `kml-mcp-vX.Y.Z-<target>` archives and checksums
- matching `kml-mcp-remote-vX.Y.Z-<target>` archives and checksums
- `.mcpb` package artifact for `kml-mcp`
- `.mcpb.sha256` checksum
- rendered MCP Registry `server.json`

The `x86_64-apple-darwin` artifact uses GitHub Actions `macos-15-intel`
because the older `macos-13` image is no longer supported.

The root `action.yml` is the official GitHub Action channel from `v0.11.0`.
Release preflight must keep `just action-smoke` passing before publishing a tag.

Tag push does not drive release publication. Use a `release/vX.Y.Z` pull
request for automatic release, or use `just VERSION=vX.Y.Z release-github` /
`just VERSION=vX.Y.Z release` for manual dispatch.

Manual GitHub Actions dispatch is still available, but the local `just`
recipes are preferred because they create or verify the signed tag and check the
`CARGO_REGISTRY_TOKEN` secret before dispatch.

## MCPB And Registry Publication

Build and validate the MCPB artifact locally:

~~~bash
just VERSION=vX.Y.Z mcpb-smoke
just VERSION=vX.Y.Z server-json-validate
~~~

`just mcpb-smoke` builds `target/mcpb/katana-markdown-linter-X.Y.Z.mcpb`,
writes a SHA-256 checksum beside it, extracts the bundle, and runs the bundled
`kml-mcp` binary through the stdio smoke test.

`just server-json-validate` renders `target/mcpb/server.json` from the
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

The release workflow generates both `target/homebrew/kml.rb` and
`target/homebrew/kml@X.Y.Z.rb` from the verified binary assets. After the
GitHub Release exists, the workflow updates `HiroyukiFuruno/homebrew-katana`
with `HOMEBREW_KATANA_GIT_TOKEN`.

Configure this repository secret before publishing:

~~~bash
gh secret set HOMEBREW_KATANA_GIT_TOKEN --body "<token-with-homebrew-katana-write-access>"
~~~

Do not add a `github.token` fallback. The tap update must fail when the
dedicated token is missing. `just VERSION=vX.Y.Z release-verify` compares the
generated formulae with the actual tap files. It also checks the consistency
against `target/release-verify-state.json` generated by
`release-target-check`.

- `Formula/kml.rb`
- `Formula/kml@X.Y.Z.rb`

If either file still points at an older release, verification fails.

## Wrapper Publication

The npm and Python wrappers are thin launchers over GitHub Release binary
archives. They are official install channels only when registry state and
wrapper launch verification pass for the release version.

Keep these conditions true before publishing a wrapper version:

- npm and PyPI maintainer accounts exist for the package owner.
- The GitHub repository has a `pypi` environment for PyPI publication.
- npm trusted publishing is configured for `release.yml`.
- PyPI trusted publishing is configured for `release.yml` with the `pypi` environment.
- The release workflow is either triggered by a merged `release/vX.Y.Z` pull request or dispatched with wrapper publish flags enabled.
- `just VERSION=vX.Y.Z wrapper-smoke` succeeds against the release archive shape.
- `just npm-package-check` confirms the npm README, metadata, and tarball file list.
- `just npm-publish-target-check` confirms the npm version is not already published. If npm returns `Unpublished on`, treat it as a soft signal and continue to publish attempt.
- `just pypi-package-check` confirms the PyPI README, metadata, source distribution, wheel, and wheel metadata.
- `just VERSION=vX.Y.Z wrapper-smoke` verifies `kml`, `kml-mcp`, and `kml-mcp-remote` through local wrapper launchers.
- `npx --yes katana-markdown-linter@X.Y.Z kml-mcp --workspace-root <path>` is the documented npm stdio MCP form.
- `bunx --package katana-markdown-linter@X.Y.Z kml-mcp --workspace-root <path>` is documented only while it passes smoke in the release environment.
- `uvx --from katana-markdown-linter==X.Y.Z kml-mcp --workspace-root <path>` is the documented PyPI stdio MCP form.

Use these trusted publisher settings:

| Registry | Project or package | Owner | Repository | Workflow filename | Environment |
| --- | --- | --- | --- | --- | --- |
| npm | `katana-markdown-linter` | `HiroyukiFuruno` | `katana-markdown-linter` | `release.yml` | Leave empty unless npm is configured with a GitHub environment |
| PyPI | `katana-markdown-linter` | `HiroyukiFuruno` | `katana-markdown-linter` | `release.yml` | `pypi` |

## Editor Extension Publication

Editor extensions for VS Code and Zed are thin wrappers around `kml linter`. They follow the same versioning as the core linter but may have independent publication schedules.

### Marketplace Prerequisites

When publication is enabled, the following metadata must be exactly as specified:

| Editor | Field | Expected Value |
| --- | --- | --- |
| VS Code | `publisher` | `HiroyukiFuruno` |
| VS Code | `name` | `vscode-katana-markdown-linter` |
| Zed | `id` | `katana-markdown-linter` |
| Zed | `authors` | Must contain `Hiroyuki Furuno` |

Keep these conditions true before publishing an editor extension:

- `just editor-extension-check` passes for both `editors/vscode` and `editors/zed`.
- `just editor-publish-gate` confirms whether publication is enabled or deferred and verifies the above metadata. Local checks always defer publication.
- Release verification (`just release-verify`) reports `published` or `deferred` status based on `PUBLISH_VSCODE_EXTENSION` and `PUBLISH_ZED_EXTENSION` flags.

**Stop Route:** If any of the above prerequisites are not met when publication is enabled, the release process will exit with an error. Fix the metadata or disable publication before proceeding.

If an extension publication is deferred, it will not block the core linter release. However, once enabled, publication must succeed before the release can be considered complete.

The PyPI project name must match `wrappers/python/pyproject.toml`. Change the
wrapper metadata first if the package name is changed before publication.
After wrapper publication, `just VERSION=vX.Y.Z release-verify` checks the npm
registry version, PyPI JSON version, `npx` launcher output, `uvx` launcher
output, MCP binary release assets, and MCP wrapper JSON-RPC smoke.

## Required Secrets

- `CARGO_REGISTRY_TOKEN`: crates.io API token used only when manual dispatch sets `publish_crate: true`.

GitHub OIDC Registry authentication does not require a dedicated secret, but the
Release workflow must keep `id-token: write`.

## Quality Gates and Branch Protection

See `docs/quality-gates.md` for the authoritative mapping between local `just` recipes, CI required checks, and branch protection.

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
- `just release-tag` refuses to overwrite a remote tag whose target differs from the local tag.

### Upstream drift gate fails

- Inspect the reported markdownlint rule drift.
- Update local rule metadata, config properties, fixture matrix, or allowlist intentionally.
- Re-run `just upstream-drift` with the same upstream docs checkout.

### Coverage blocking gate fails

- Add tests for the newly uncovered paths.
- If the gap is intentional, update `scripts/ci/coverage-baseline.txt` in the same review and explain why the baseline increased.

### GitHub Release was created but crates.io publish failed

- Check whether the version exists on crates.io.
- If it was not published, fix the token or package issue and re-run with the same version.
- If it was partially published, do not reuse the same version for changed content; bump `Cargo.toml` version.
- `just release` fails fast when the requested version already exists on crates.io.
- Use `just VERSION=vX.Y.Z release-verify` to compare the local tag target, GitHub Release title and target, crates.io version, npm version, PyPI version, wrapper launch output, and Homebrew formula evidence after retry.

### Accidental version was published

Use the accidental release recovery plan before changing package metadata by
hand:

~~~bash
just BAD_VERSION=v0.18.7 release-recovery-plan
~~~

The plan only uses non-destructive registry actions by default:

- yank the crates.io version
- deprecate the npm version
- move the npm `latest` dist-tag back to the replacement version
- mark the GitHub Release as an accidental prerelease
- list the PyPI yank and Homebrew tap rollback steps that still require manual review

To run the executable steps, set the confirmation environment variable to the
bad tag:

~~~bash
KML_RELEASE_RECOVERY_CONFIRM=v0.18.7 just BAD_VERSION=v0.18.7 release-recover
~~~

Do not unpublish npm, delete PyPI files, delete GitHub assets, or remove tags as
the first recovery action. Published versions are already observable by users;
preserve evidence and publish a corrected version instead.

### MCP Registry publish failed

- Verify the GitHub Release contains the `.mcpb`, `.mcpb.sha256`, and rendered
  `server.json` assets.
- Re-run `just VERSION=vX.Y.Z server-json-validate` and inspect the rendered
  `fileSha256`.
- If the Registry version was not accepted, fix authentication or metadata and
  rerun the workflow with the same version.
- If the Registry version was accepted with a bad artifact reference, do not
  reuse the same version for changed content; bump `Cargo.toml` version.

### MCPB smoke fails

- Re-run `just VERSION=vX.Y.Z mcpb-smoke`.
- Inspect `mcpb/manifest.json`, `scripts/release/package-mcpb.sh`, and
  `scripts/ci/mcpb-smoke.py`.
- Keep the manifest aligned with local stdio execution; do not claim remote MCP
  transport.

### MCP wrapper smoke fails

- Re-run `just VERSION=vX.Y.Z wrapper-smoke`.
- Inspect `wrappers/npm/lib/installer.js`, `wrappers/python/src/katana_markdown_linter/installer.py`, and `scripts/release/smoke-wrappers.sh`.
- Keep wrapper output off stdout for `kml-mcp`; stdout is reserved for MCP JSON-RPC messages.
- Confirm `kml`, `kml-mcp`, and `kml-mcp-remote` caches are separated by version, target, and executable role.

### MCP remote smoke fails

- Re-run `just mcp-remote-smoke`.
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

- Re-run `just action-smoke`.
- Inspect `action.yml`, `scripts/action/install-kml.sh`, and `scripts/action/run-kml.sh`.
- Keep the action scripts generic; do not add repository-specific lint policy outside action inputs.

### npm package check fails

- Re-run `just npm-package-check`.
- Inspect `wrappers/npm/package.json`, `wrappers/npm/README.md`, and
  `scripts/release/verify-npm-package.js`.
- Keep the npm wrapper dependency-free unless a runtime dependency is justified
  by launcher behavior.

### npm publish target check fails

- Re-run `just npm-publish-target-check`.
- If the version is already published, do not publish the same npm version again.
- If npm reports `Unpublished on`, `npm-publish-target-check` may still pass so a manual publish can continue (authentication/2FA may be the next failure).
- If the registry check fails for another reason, fix registry access before running `npm publish`.

### Incorrect files were packaged

- Update the `include` list in `Cargo.toml`.
- Re-run `cargo package` and the dry-run publish check.
