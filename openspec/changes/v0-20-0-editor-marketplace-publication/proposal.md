## Why

VS Code / Zed 拡張は、editor 上で設定反映済みの診断・整形・安全な修正が使える状態になってから公開する必要がある。`v0.20.0` は、`v0.19.0` 系の機能完了と最終 dogfood evidence を前提に、Marketplace 公開だけを扱う release change とする。

## What Changes

- `v0-19-0-editor-capability-completion` の完了 evidence を公開前提条件にする。
- VS Code Marketplace 公開を `VSCE_PAT` と `vsce` で実行する。
- Zed extension registry 公開を `zed-industries/extensions` の merged PR で検証する。
- release workflow / runbook / release verification が editor marketplace の published / failed 状態を説明できるようにする。
- 公開後に VS Code Marketplace と Zed extension registry の両方で `v0.20.0` を確認する。

## Capabilities

### New Capabilities

- `editor-marketplace-publication`: VS Code / Zed Marketplace 公開の前提条件と実行契約。
- `editor-publication-verification`: 公開後の registry 状態確認と失敗時の停止契約。

### Modified Capabilities

- `release-cicd`: editor marketplace publish を release gate / release verification に統合する。

## Impact

- `.github/workflows/release.yml`
- `just/release.just`
- `scripts/release/**`
- `docs/release-runbook.md`
- `docs/release-readiness/**`
- `editors/vscode/package.json`
- `editors/zed/extension.toml`
- `openspec/specs/release-cicd/spec.md`
