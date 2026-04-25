# Tasks

## Definition Of Ready

- [x] v0.6.0 is released and archived.
- [x] README Rule Map cleanup is merged or isolated from this change.
- [x] Current performance docs identify `api_fix_large_document` as a hot path.
- [x] rumdl is reviewed as a reference for performance/product direction without copying implementation.

## 1. Planning And Baseline

- [x] 1.1 Update active roadmap so `v0.7.0` is performance, later work covers linter precision, unsafe fix mode, formatter productization, and tool distribution.
- [x] 1.2 Capture a pre-change local performance snapshot for the fix hot path.
- [x] 1.3 Confirm the optimization does not change public API or safe fix policy.

## 2. Fix Application Hot Path

- [x] 2.1 Replace repeated full-content offset scans in `fix::apply` with a per-call line offset index.
- [x] 2.2 Preserve virtual EOF insertion behavior.
- [x] 2.3 Preserve UTF-8 char boundary handling.
- [x] 2.4 Keep overlap resolution and output construction behavior unchanged.

## 3. Tests

- [x] 3.1 Add unit tests for multi-line optimized range mapping.
- [x] 3.2 Add unit tests for Unicode boundary clamping.
- [x] 3.3 Add unit tests for virtual EOF insertion.
- [x] 3.4 Confirm `fix_with_results_matches_fix_output` still passes.

## 4. Documentation And Baseline

- [x] 4.1 Update `docs/performance.md` with v0.7.0 before/after notes.
- [x] 4.2 Refresh `tests/fixtures/perf-baseline.json` after the intentional optimization.
- [x] 4.3 Keep rumdl-inspired cache/watch/distribution ideas in roadmap, not in v0.7.0 scope.

## 5. Release Preparation

- [x] 5.1 Bump crate version to 0.7.0.
- [x] 5.2 Add CHANGELOG entry for v0.7.0.
- [x] 5.3 Run `make release-check VERSION=v0.7.0`.

## Verification

- [x] `cargo fmt --all -- --check` passes.
- [x] `cargo test --workspace --locked` passes.
- [x] `make dogfood` passes.
- [x] `make perf-check` passes.
- [x] `make release-check VERSION=v0.7.0` passes.
- [x] `git diff --check` passes.

## Definition Of Done

- [x] `api_fix_large_document` median improves in local benchmark snapshot or the result is documented if not improved.
- [x] Fixed output and applied fix counts remain compatible.
- [x] Performance baseline and docs reflect the intentional change.
- [x] Roadmap clearly separates performance, linter precision, unsafe fix mode, formatter productization, and distribution/tool expansion.
- [x] v0.7.0 release readiness gates pass locally.
