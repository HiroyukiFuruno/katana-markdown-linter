# Dependency update evidence for Issue 133

Issue: https://github.com/HiroyukiFuruno/katana-markdown-linter/issues/133

upstream release: `cargo upgrade --dry-run --incompatible` reports all 13 direct Rust packages current. Root and Zed `cargo update --dry-run --verbose` resolve zero compatible updates. All release dependencies remain registry packages. matchit0.8.4 is constrained by published axum0.8.9; it cannot be independently moved to0.8.6 without changing the upstream contract.

api migration note: No dependency API migration is needed. The source adapter and configuration changes preserve public APIs and source coordinates. VSCode runtime languageclient8.1.0 and the ^1.75.0 minimum remain; languageclient11 would raise the runtime minimum. Node18 types are retained for the existing host contract. TypeScript7 remains outside the current parser peer range; TS6.0.3 is retained.

compatible updates: VSCode @types/vscode1.140.0 ->1.144.0 in the lockfile. taiki-e/install-action v2.87.33 ->v2.88.0 is pinned to official commit86f7a0ffb25f40114606501a311ebcfaf882ea32 in all three workflows. Its release adds similarity-rs support without changing the existing just installer invocation. All other pinned actions and rust-toolchain master match current upstream releases/revisions. Python setuptools>=68 accepts the current84.2.1 backend; the wrapper has no runtime dependencies. The npm wrapper has no dependency graph.

verification evidence: npm audit reports0 advisories. Rust/editor/package release-check and workspace check passed; coverage806<=881 and strict performance20/20 passed. Publication remains pending in OpenSpec until the actual result is collected. No baseline, threshold, coverage gate, or editor minimum is reduced for these updates.

Final verification: all release-check and workspace checks pass; strict20/20 with30 iterations,9samples,warmup5 at the unchanged1.4 threshold. Official install-action latest release was rechecked as v2.88.0 before the PR update.
