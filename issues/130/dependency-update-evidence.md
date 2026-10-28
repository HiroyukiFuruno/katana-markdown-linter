# Dependency update evidence for Issue 130

Issue: <https://github.com/HiroyukiFuruno/katana-markdown-linter/issues/130>

upstream release: pulldown-cmark 0.13.5, katana-ast-lint 0.5.3, ignore 0.4.34, globset 0.4.21, unicode-width 0.2.3 are available from crates.io; taiki-e/install-action v2.87.33 is pinned to its official tag commit.

api migration note: The source adapter uses pulldown-cmark offset events behind DocumentContext. Public source ranges and single-pass fix APIs retain their contracts. Optional MCP dependencies remain registry dependencies; no sibling repository or path/git release dependency is introduced. Fix iteration is shared internally and reports cycles or an exhausted budget rather than returning an unstable result.

verification evidence: `cargo upgrade --dry-run --incompatible` found all 13 direct packages current. Shared syntax/fix regressions, architecture contracts, upstream golden comparisons, 258 reviewed document answers over three fix runs, and strict performance checks have passed locally. Full release-check, CI, public artifact verification, and cleanup evidence are tracked with their actual completion state in the OpenSpec task ledger.

The historical performance baseline and its 1.4 ratio threshold remain unchanged.

Editor dependency verification: ESLint 10.13.0, @eslint/js 10.0.1, typescript-eslint 8.72.1, TypeScript 6.0.3, @types/vscode 1.140.0, and @vscode/test-electron 3.1.0 are current compatible releases. TypeScript 7 is outside the parser peer range; languageclient 11 would raise the declared editor minimum. Mocha/glob were removed after testing their latest releases against the Node 16 host: the same three smoke assertions now use built-in Node assertions through the official extension host runner. Actual VS Code 1.75.0 tests and npm audit (zero advisories) pass.

## Merged release evidence

Source HEAD `a024276c05f9c4dba3727df015a51193e88d1fd7` was merged as `1277353eee9cb5a8903cfc8f405a56ad661b0db6` (PR [#131](https://github.com/HiroyukiFuruno/katana-markdown-linter/pull/131)). The PR re-review reported no findings; both P2 threads received replies and are resolved. CI runs [40753342786](https://github.com/HiroyukiFuruno/katana-markdown-linter/actions/runs/40753342786) and [40753342777](https://github.com/HiroyukiFuruno/katana-markdown-linter/actions/runs/40753342777) passed.

`just VERSION=v0.20.0 release-check` exited 0. Its blocking coverage gate passed at 804 uncovered lines against the unchanged 881-line baseline. The current document-answer report records 258 reviewed documents (200 public, 58 original), zero mismatches or answer diagnostics, zero answer-fix changes, and three successful fix passes; the measured performance report remains below the 1.4 threshold. Report hashes and the historical checkpoint are retained in `verification-summary.json`. Publication and `just VERSION=v0.20.0 release-verify` are now complete. The [Release workflow40754506735](https://github.com/HiroyukiFuruno/katana-markdown-linter/actions/runs/40754506735) succeeded; the signed tag, [GitHub Release](https://github.com/HiroyukiFuruno/katana-markdown-linter/releases/tag/v0.20.0), crates.io/npm/PyPI0.20.0, current-platform CLI/MCP binaries, actual npm/PyPI wrapper launches, and Homebrew formulas passed verification. Marketplace publication remains in its independent v0.21.0 plan.
