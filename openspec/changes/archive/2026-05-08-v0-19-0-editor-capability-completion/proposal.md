## Why

現行の VS Code / Zed 拡張は `kml lsp` を起動できるが、project config を反映した診断・安全な修正を提供できない。linter の editor integration として公開するには、コード上の表示と修正が CLI と同じ設定解決に従う必要がある。

## What Changes

- LSP が Markdown document ごとに `.markdownlint.json` / `.markdownlint.jsonc` を探索・解決する。
- editor 上の診断、安全な修正候補、整形リクエストが config 解決結果と config error を尊重する。
- VS Code / Zed の両方で、設定反映済み診断と安全な修正候補を検証する。
- Marketplace 公開前に必要な最終 dogfood を `v0.19.0` 系の完了条件として固定する。
- Marketplace 公開処理は含めない。公開は `v0-20-0-editor-marketplace-publication` に送る。

## Capabilities

### New Capabilities

- `editor-config-aware-lsp`: editor LSP が CLI と同じ project config 解決を使う契約。
- `editor-safe-actions`: editor 上の診断、整形、安全な修正候補が config と安全性を尊重する契約。
- `editor-dogfood-verification`: 実リポジトリで editor integration を最終検証する契約。

### Modified Capabilities

- なし

## Impact

- `src/lsp/**`
- `src/cli/workflow/common.rs` または共有 config 解決層
- `editors/vscode/**`
- `editors/zed/**`
- `tests/**`
- `just/**`
- `docs/release-readiness/**`
