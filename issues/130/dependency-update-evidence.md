# Dependency update evidence for Issue 130

Issue: <https://github.com/HiroyukiFuruno/katana-markdown-linter/issues/130>

upstream release: pulldown-cmark 0.13.5, katana-ast-lint 0.5.3, ignore 0.4.34, globset 0.4.21, unicode-width 0.2.3 are available from crates.io; taiki-e/install-action v2.87.33 is pinned to its official tag commit.

api migration note: The source adapter uses pulldown-cmark offset events behind DocumentContext. Public source ranges and single-pass fix APIs retain their contracts. Optional MCP dependencies remain registry dependencies; no sibling repository or path/git release dependency is introduced. Fix iteration is shared internally and reports cycles or an exhausted budget rather than returning an unstable result.

verification evidence: `cargo upgrade --dry-run --incompatible` found all 13 direct packages current. Shared syntax/fix regressions, architecture contracts, upstream golden comparisons, 258 reviewed document answers over three fix runs, and strict performance checks have passed locally. Full release-check, CI, public artifact verification, and cleanup evidence are tracked with their actual completion state in the OpenSpec task ledger.

The historical performance baseline and its 1.4 ratio threshold remain unchanged.

Editor dependency verification: ESLint 10.13.0, @eslint/js 10.0.1, typescript-eslint 8.72.1, TypeScript 6.0.3, @types/vscode 1.140.0, and @vscode/test-electron 3.1.0 are current compatible releases. TypeScript 7 is outside the parser peer range; languageclient 11 would raise the declared editor minimum. Mocha/glob were removed after testing their latest releases against the Node 16 host: the same three smoke assertions now use built-in Node assertions through the official extension host runner. Actual VS Code 1.75.0 tests and npm audit (zero advisories) pass.
