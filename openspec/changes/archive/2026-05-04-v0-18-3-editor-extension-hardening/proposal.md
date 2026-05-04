# Editor Extension Hardening

## Target Version

`v0.18.3`

## Why

VS Code と Zed の extension MVP が揃った後は、利用者が install し、
release ごとに検証できる状態へ仕上げる必要がある。

この change では editor extension を product surface として扱い、
package、docs、smoke tests、release verification をまとめて固める。

## What Changes

- VS Code / Zed extension の package verification を release gate に固定する
- extension と `kml` CLI version compatibility を明文化する
- install docs、troubleshooting、binary path 設定を整理する
- Marketplace / registry 公開準備に必要な metadata を追加する
- release verification が extension package と `kml lsp` launch を確認する
- Neovim は plugin 実装ではなく docs-only sample として整理する

## Capabilities

### New Capabilities

- `editor-extension-release`: editor extension package、compatibility、release verification、公開準備を扱う

### Modified Capabilities

- `release-readiness`: release gate に editor extension verification を追加する

## Impact

- `editors/vscode/**` or `extensions/vscode/**`
- `editors/zed/**` or `extensions/zed/**`
- `src/lsp/**`
- `tests/cli_lsp_contract.rs`
- `Justfile`
- `.github/workflows/**`
- `scripts/release/**`
- `README.md`
- `docs/editor-integration.md`
- `docs/quality-gates.md`
- `docs/release-runbook.md`
- `CHANGELOG.md`
