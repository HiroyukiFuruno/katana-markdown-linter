# Quality Gates

## Local Targets

| Target | Responsibility | Blocking |
| --- | --- | --- |
| `make fmt-check` | Verify rustfmt output is committed | Yes |
| `make lint` | Run Clippy with zero warnings | Yes |
| `make ast-lint` | Verify repository-specific invariants | Yes |
| `make test` | Run workspace unit and integration tests | Yes |
| `make dogfood` | Run `kml` against maintained Markdown and fail on new diagnostics | Yes |
| `make check` | Run `fmt-check`, `lint`, `ast-lint`, `test`, and `dogfood` | Yes |
| `make coverage` | Report uncovered lines against the current test suite | No |
| `make coverage-blocking` | Fail when uncovered lines exceed `scripts/ci/coverage-baseline.txt` | Yes |
| `make upstream-drift` | Compare local rule catalog with upstream markdownlint docs | Yes when upstream docs are provided |
| `make upstream-golden` | Compare selected `kml` diagnostics and fixes with the locked upstream oracle baseline | Yes when golden compatibility is in scope |
| `make upstream-golden-live` | Run the live upstream markdownlint oracle against the golden corpus | No, manual update check |
| `make rule-dashboard` | Regenerate `docs/rule-coverage-dashboard.md` | No, generation helper |
| `make bench-cross-tools` | Compare `kml` CLI timing with optional `mado` and `rumdl` binaries | No, manual performance probe |
| `make release-check` | Run local release preflight gates except live upstream clone | Yes |
| `make release-verify` | Verify published tag, GitHub Release, and crates.io state | Yes after publication |

`make lint` is intentionally limited to Clippy. Repository-specific checks belong in `make ast-lint` so Rust style warnings and project invariants can be triaged independently.

## AST Lint Invariants

`make ast-lint` protects invariants that normal compiler checks do not cover:

- source code must not contain lazy macros such as `todo!`, `unimplemented!`, or `dbg!`
- CLI directory traversal must use the parallel `ignore` walker and respect gitignore controls
- fixture matrix entries must match the active rule catalog and keep the expected schema fields
- release workflow must require an existing annotated signed tag that GitHub reports as Verified
- release retry helpers must refuse remote tag overwrites and already-published crates.io versions
- upstream drift checking must be wired through `make upstream-drift` and release workflows
- public library API and rule catalog entrypoints must remain explicit

These checks are intentionally repo-specific. KatanA UI-specific lint rules are not imported into this crate.

## Manual Performance Probes

`make bench-cross-tools` and its narrower variants are intentionally excluded
from `make check`, `release-check`, and required CI. They can depend on
optional third-party binaries and local CPU timing, so they are useful for
investigation but unsuitable as a merge gate.

Use these targets when comparing CLI behavior or speed against peer tools:

- `make bench-cross-tools`
- `make bench-cross-tools-default`
- `make bench-cross-tools-common`
- `make bench-cross-tools-fix`

## CI Required Checks

Branch protection for `main` currently requires:

- `Test and Build (macos-latest)` -> `.github/workflows/test-and-build.yml`, `make fmt-check`, `make lint`, `make ast-lint`, `cargo test --workspace`, `make dogfood`
- `Test and Build (ubuntu-latest)` -> `.github/workflows/test-and-build.yml`, same checks plus non-blocking `make coverage`

If required check names or workflow job names change, update branch protection in the same change. Otherwise the repository can either block valid merges or allow merges without the intended gate.

## PR And Direct Push

Normal development should merge through pull requests so required checks run before `main` changes. Direct pushes to `main` are blocked for non-admins. Admin bypasses may still push, but GitHub reports bypassed rules for PR and required-check requirements; treat that as an exception path only.

## Coverage Modes

`make coverage` is report mode. It prints uncovered lines and exits successfully so developers can inspect gaps without blocking routine work.

`make coverage-blocking` is regression mode. It compares uncovered lines to `scripts/ci/coverage-baseline.txt` and fails only when the count increases. Lower the baseline in the same commit when coverage improves.

## Release Readiness

Before dispatching a release workflow, run:

```bash
make release-check VERSION=vX.Y.Z
```

The local release check runs formatting, Clippy, AST lint, tests, dogfood,
coverage regression, example builds, optional MCP build, version verification,
dry-run publish, and install smoke checks. The GitHub release workflows
additionally clone upstream markdownlint and run `make upstream-drift` against
the default branch docs.

Use `make release-tag VERSION=vX.Y.Z` before dispatching a release. It creates
or verifies a signed annotated tag and then requires GitHub to report the tag as
`Verified`.

Use `make release-github VERSION=vX.Y.Z` for GitHub Release-only publication.
Use `make release VERSION=vX.Y.Z` only when crates.io publication is intended.
`make release` fails before dispatch when the requested version already exists on crates.io.

After publication, run:

```bash
make release-verify VERSION=vX.Y.Z
```

That command compares the local tag target, GitHub Release target, GitHub tag
verification state, and crates.io version.

Run `make upstream-golden` before changing rule behavior or fix behavior. It is deterministic and does not require network access. Run `make upstream-golden-live` only when refreshing the upstream oracle or investigating compatibility drift.

## Failure Recovery

- `make lint` fails: fix the Clippy warning or adjust the code so the warning is unnecessary.
- `make ast-lint` fails: read the named invariant and update the fixture, workflow, public API, or traversal implementation intentionally.
- `make coverage-blocking` fails: add tests for the new uncovered paths, or update the baseline only when the uncovered increase is intentional and documented in review.
- `make dogfood` fails: fix the Markdown change, or run `make dogfood-refresh-baseline` only when the diagnostic change is intentional.
- `make upstream-drift` fails: inspect new, removed, deprecated, or changed markdownlint rules and update local rule/catalog/config metadata before releasing.
- CI required checks are pending or missing: verify workflow job names still match the branch protection required check names.
- Release tag verification fails: recreate the tag only when no GitHub Release exists, the version is not published on crates.io, and the failed workflow did not publish artifacts.
- `make release` fails because the version is already on crates.io: bump `Cargo.toml` and create a new changelog section instead of retrying the same version.
