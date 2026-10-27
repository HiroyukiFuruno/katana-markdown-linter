# v0.19.6 dependency update

Issue: https://github.com/HiroyukiFuruno/katana-markdown-linter/issues/127

upstream release: GitHub API release/tag checks on 2026-10-27 confirmed checkout v7.0.1, rust-cache v2.9.3, install-action v2.87.32, setup-python/setup-node v7.1.0, upload-artifact v7.0.2, download-artifact v8.0.2, and PyPI publish v1.14.3. Workflow references use the resolved commit SHA. Root action.yml retains deliberate stable Rust tracking.

api migration note: rmcp 3.4.1 to 3.8.0 requires no product source changes. zed_extension_api 0.3.0 to 0.7.0 retains the extension's used APIs and passes wasm32-wasip1 compilation. The Zed API embeds version 0.7.0 and uses edition 2024 internally; the existing stable Rust toolchain supports it. Marketplace publication remains deferred. install-action now receives explicit tool inputs instead of alias-tag defaults.

verification evidence: Root and Zed cargo outdated report all dependencies up to date. just zed-extension-check passed. Full just VERSION=v0.19.6 release-check passed (exit 0); log: /tmp/kml-v0.19.6-release-check.log. This includes all-feature tests, coverage, MCP stdio/remote, packages, wrappers, editors, publish dry-run and installed CLI checks. Publication evidence will be recorded before Issue closure. Existing exact-reference AST guards are updated, not relaxed. Coverage passed at 802 uncovered lines against the existing 881 baseline.

Required CI follow-up: run 40445711340 detected a real MD043 compatibility gap against markdownlint commit 5dd44fe44481cfbc8c6fee955719773fa17dc5d5. Its headings type/default is string[]|null/null; KML previously conflated null and an empty array. The repair aligns metadata, validation, schema and behavior with regression tests, without drift exceptions. Initial local verification above must be repeated after the repair.

Final repair verification: full release-check, final all-feature tests, fmt/Clippy/AST checks and coverage all passed. Final coverage is 802 <= the unchanged 881 baseline. Live upstream drift passed without exceptions. Final logs: /tmp/kml-v0.19.6-final-release-check.log, /tmp/kml-v0.19.6-final-tests.log, /tmp/kml-v0.19.6-final-lint.log, /tmp/kml-v0.19.6-final-coverage.log, /tmp/kml-v0.19.6-md043-drift.log. Independent repair re-review reports no open P0/P1/P2 findings.

## Publication and cleanup follow-up

PR #128 merged as c325648464a5afab2a3435966c32a1394aa0b0ef. Release run 40448337956 published v0.19.6 to GitHub Release, crates.io, npm and PyPI; all publication jobs passed. Its cleanup job failed because a failed origin/HEAD lookup was forced to succeed by `|| true`, returning an empty default branch name.

The maintenance change repairs default-branch detection with actual temporary Git repository regression coverage. Remote branch deletion becomes opt-in, matching the user's cleanup policy. The maintenance PR must pass Draft review and required checks before merging; only the failed cleanup job of run 40448337956 is then rerun. The existing release tag and packages remain untouched.

`just VERSION=v0.19.6 release-verify` passed: tag and GitHub Release both target c325648464a5afab2a3435966c32a1394aa0b0ef, all registries report 0.19.6, binary/MCP/wrapper smoke tests pass, and Homebrew formulas verify. Log: /tmp/kml-v0.19.6-release-verify.log. Final cleanup/Issue closure evidence is recorded in Issue #127 and the session task ledger.

Cleanup repair verification: actual remote HEAD is checked after fetch and must resolve to a fetched commit; stale/missing local origin/HEAD is ignored, invalid remote HEAD fails closed, and remote deletion is opt-in. Four real Git regression cases pass (master/main without local origin/HEAD, nested default branch, invalid remote HEAD with both candidates, and stale local HEAD). Shell syntax, formatting, and diff checks pass.
