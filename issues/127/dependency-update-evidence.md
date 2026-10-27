# v0.19.6 dependency update

Issue: https://github.com/HiroyukiFuruno/katana-markdown-linter/issues/127

upstream release: GitHub API release/tag checks on 2026-10-27 confirmed checkout v7.0.1, rust-cache v2.9.3, install-action v2.87.32, setup-python/setup-node v7.1.0, upload-artifact v7.0.2, download-artifact v8.0.2, and PyPI publish v1.14.3. Workflow references use the resolved commit SHA. Root action.yml retains deliberate stable Rust tracking.

api migration note: rmcp 3.4.1 to 3.8.0 requires no product source changes. zed_extension_api 0.3.0 to 0.7.0 retains the extension's used APIs and passes wasm32-wasip1 compilation. The Zed API embeds version 0.7.0 and uses edition 2024 internally; the existing stable Rust toolchain supports it. Marketplace publication remains deferred. install-action now receives explicit tool inputs instead of alias-tag defaults.

verification evidence: Root and Zed cargo outdated report all dependencies up to date. just zed-extension-check passed. Full just VERSION=v0.19.6 release-check passed (exit 0); log: /tmp/kml-v0.19.6-release-check.log. This includes all-feature tests, coverage, MCP stdio/remote, packages, wrappers, editors, publish dry-run and installed CLI checks. Publication evidence will be recorded before Issue closure. Existing exact-reference AST guards are updated, not relaxed. Coverage passed at 802 uncovered lines against the existing 881 baseline.
