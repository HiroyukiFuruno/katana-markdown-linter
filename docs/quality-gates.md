# Quality Gates

## Local Targets

| Target | Responsibility | Blocking |
| --- | --- | --- |
| `make fmt-check` | Verify rustfmt output is committed | Yes |
| `make lint` | Run Clippy with zero warnings | Yes |
| `make ast-lint` | Verify repository-specific invariants | Yes |
| `make test` | Run workspace unit and integration tests | Yes |
| `make check` | Run `fmt-check`, `lint`, `ast-lint`, and `test` | Yes |
| `make coverage` | Report uncovered lines against the current test suite | No |
| `make coverage-blocking` | Fail when uncovered lines exceed `scripts/ci/coverage-baseline.txt` | Yes |
| `make upstream-drift` | Compare local rule catalog with upstream markdownlint docs | Yes when upstream docs are provided |
| `make release-check` | Run local release preflight gates except live upstream clone | Yes |

`make lint` is intentionally limited to Clippy. Repository-specific checks belong in `make ast-lint` so Rust style warnings and project invariants can be triaged independently.

## AST Lint Invariants

`make ast-lint` protects invariants that normal compiler checks do not cover:

- source code must not contain lazy macros such as `todo!`, `unimplemented!`, or `dbg!`
- CLI directory traversal must use the parallel `ignore` walker and respect gitignore controls
- fixture matrix entries must match the active rule catalog and keep the expected schema fields
- release workflow must require an existing annotated signed tag
- upstream drift checking must be wired through `make upstream-drift` and release workflows
- public library API and rule catalog entrypoints must remain explicit

These checks are intentionally repo-specific. KatanA UI-specific lint rules are not imported into this crate.

## CI Required Checks

Branch protection for `main` currently requires:

- `Test and Build (macos-latest)` -> `.github/workflows/test-and-build.yml`, `make fmt-check`, `make lint`, `make ast-lint`, `cargo test --workspace`
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

The local release check runs formatting, Clippy, AST lint, tests, coverage regression, version verification, dry-run publish, and install smoke checks. The GitHub release workflows additionally clone upstream markdownlint and run `make upstream-drift` against the default branch docs.

## Failure Recovery

- `make lint` fails: fix the Clippy warning or adjust the code so the warning is unnecessary.
- `make ast-lint` fails: read the named invariant and update the fixture, workflow, public API, or traversal implementation intentionally.
- `make coverage-blocking` fails: add tests for the new uncovered paths, or update the baseline only when the uncovered increase is intentional and documented in review.
- `make upstream-drift` fails: inspect new, removed, deprecated, or changed markdownlint rules and update local rule/catalog/config metadata before releasing.
- CI required checks are pending or missing: verify workflow job names still match the branch protection required check names.
