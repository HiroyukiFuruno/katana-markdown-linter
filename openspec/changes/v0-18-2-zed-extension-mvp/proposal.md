# Zed Extension MVP

## Target Version

`v0.18.2`

## Why

VS Code extension MVP で `kml lsp` を editor wrapper から起動する契約が固まる。
次は Zed を second target とし、Zed 固有の extension 境界だけを検証する。

Zed 側でも lint logic は持たず、`kml lsp` を共有エンジンとして使う。

## What Changes

- Zed extension package を repository に追加する
- extension は Markdown file に対して `kml lsp` を起動する
- extension は `.markdownlint.json` / `.markdownlint.jsonc` の schema 利用手順を docs に固定する
- Zed extension build / package / smoke check を追加する
- VS Code extension と同じ editor-facing behavior を Zed で確認する

## Capabilities

### New Capabilities

- `zed-extension`: Zed extension package、language server registration、configuration、smoke check を扱う

### Modified Capabilities

なし。

## Impact

- `editors/zed/**` または `extensions/zed/**`
- Zed extension manifest and source
- `src/lsp/**`
- `tests/cli_lsp_contract.rs`
- `Justfile`
- `.github/workflows/**`
- `README.md`
- `docs/editor-integration.md`
- `docs/quality-gates.md`
- `CHANGELOG.md`
