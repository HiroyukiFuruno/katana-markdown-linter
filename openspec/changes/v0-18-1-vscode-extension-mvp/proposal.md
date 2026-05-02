# VS Code Extension MVP

## Target Version

`v0.18.1`

## Why

`kml lsp` と published config schema が揃うと、VS Code では専用拡張機能（extension）を
薄い起動ラッパーとして実装できる。

最初の editor target は VS Code に絞り、lint logic を extension 側へ移さず、
`kml` CLI / LSP を共有エンジンとして使う。

## What Changes

- VS Code extension package を repository に追加する
- extension は Markdown file で `kml lsp` を stdio 起動する
- extension は `.markdownlint.json` / `.markdownlint.jsonc` に published schema を関連付ける
- extension は diagnostics、format、range format、safe quick-fix を LSP 経由で公開する
- extension smoke test を追加し、`kml lsp` と extension package の接続を検証する
- README / docs に VS Code extension MVP の install と設定を追加する

## Capabilities

### New Capabilities

- `vscode-extension`: VS Code extension package、activation、configuration、schema association を扱う
- `editor-lsp-contract`: editor wrapper が依存する `kml lsp` の最小 contract を扱う

### Modified Capabilities

なし。

## Impact

- `editors/vscode/**` または `extensions/vscode/**`
- `package.json` / VS Code extension manifest
- VS Code extension source and tests
- `src/lsp/**`
- `tests/cli_lsp_contract.rs`
- `Justfile`
- `.github/workflows/**`
- `README.md`
- `docs/editor-integration.md`
- `docs/quality-gates.md`
- `CHANGELOG.md`
