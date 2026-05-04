# Tasks

## Definition of Ready

- [x] 0.1 `v0.18.1` の VS Code extension MVP が完了している
- [x] 0.2 `v0.18.2` の Zed extension MVP が完了している
- [x] 0.3 VS Code publisher 名と package name を確認する
- [x] 0.4 Zed extension の公開先と package name を確認する
- [x] 0.5 extension icon と metadata の方針を確認する

## 1. Compatibility and Startup Checks

- [x] 1.1 VS Code extension startup で `kml --version` を確認する
- [x] 1.2 Zed extension startup で `kml --version` を確認する
- [x] 1.3 supported CLI version range を extension metadata と docs に書く
- [x] 1.4 unsupported version error を test で固定する
- [x] 1.5 explicit binary path / PATH の default behavior を docs と一致させる

## 2. Package Verification

- [x] 2.1 VS Code extension package の content check を追加する
- [x] 2.2 Zed extension package の content check を追加する
- [x] 2.3 package に local-only files が入らないことを検証する
- [x] 2.4 package validation target を `just editor-extension-check` にまとめる
- [x] 2.5 CI と release preflight に editor extension check を追加する

## 3. Release and Publish Runbook

- [x] 3.1 `scripts/release` に editor extension verification を追加する
- [x] 3.2 `just release-verify` が published / deferred の状態を説明できるようにする
- [x] 3.3 VS Code Marketplace publish 手順を runbook に追加する
- [x] 3.4 Zed extension publish 手順を runbook に追加する
- [x] 3.5 account 未設定時は publish を止める gate を追加する

## 4. Documentation

- [x] 4.1 README の editor section を VS Code / Zed / Neovim に分けて更新する
- [x] 4.2 `docs/editor-integration.md` を setup workflow 別に整理する
- [x] 4.3 schema validation と Markdown diagnostics の違いを docs に書く
- [x] 4.4 binary path setting、unsupported version、missing binary の troubleshooting を追加する
- [x] 4.5 Neovim は docs-only sample として扱い、maintained plugin と誤解されないようにする
- [x] 4.6 `CHANGELOG.md` に `v0.18.3` の editor hardening を追加する

## 5. Verification

- [x] 5.1 `just fmt-check`
- [x] 5.2 `just lint`
- [x] 5.3 `just ast-lint`
- [x] 5.4 `cargo test --workspace --locked`
- [x] 5.5 `just dogfood`
- [x] 5.6 `git diff --check`
- [x] 5.7 `just editor-extension-check`
- [x] 5.8 `just VERSION=v0.18.3 release-check`

## Definition of Done

- [x] 6.1 VS Code / Zed extension package が release gate で検証される
- [x] 6.2 extension と `kml` CLI の compatibility policy が docs と metadata で一致している
- [x] 6.3 post-release verification が editor extension artifact の published / deferred 状態を説明できる
- [x] 6.4 Marketplace / registry publish は account と package verification なしに進まない
- [x] 6.5 Neovim は docs-only LSP sample として整理されている

| 項目 | 配点 | スコア |
| :--- | :--- | :--- |
| 機能実装 | 40 | 40 |
| 品質検証 | 30 | 30 |
| ドキュメント | 20 | 20 |
| リリース準備 | 10 | 10 |
| 合計 | 100 | 100 |
