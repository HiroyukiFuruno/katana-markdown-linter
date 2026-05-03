# Tasks

## Definition of Ready

- [x] 0.1 `v0.18.1` の VS Code extension MVP が完了している
- [x] 0.2 `editor-lsp-contract` の requirements が満たされている
- [x] 0.3 Zed の公式 extension API と local development command を確認する
- [x] 0.4 Zed extension の directory 名を VS Code extension と揃える

## 1. Zed Extension Scaffold

- [x] 1.1 Zed extension manifest を追加する
- [x] 1.2 Zed extension source を追加する
- [x] 1.3 extension が `kml lsp` を起動する最小実装を追加する
- [x] 1.4 explicit `kml` path setting を追加する
- [x] 1.5 binary missing 時の setup error を追加する

## 2. LSP Registration

- [x] 2.1 Markdown file へ `kml lsp` を登録する
- [x] 2.2 diagnostics が `kml lsp` から表示されることを smoke で確認する
- [x] 2.3 format / range format の provider 設定を確認する
- [x] 2.4 safe quick-fix の code action provider 設定を確認する
- [x] 2.5 extension 側に lint logic を置かないことを確認する

## 3. Schema and Documentation

- [x] 3.1 Zed の schema mapping docs を published schema contract に合わせて更新する
- [x] 3.2 Zed extension が必要な範囲と不要な範囲を docs で分ける
- [x] 3.3 binary path setting と troubleshooting を docs に追加する
- [x] 3.4 Marketplace 公開は後続 hardening change に送ることを明記する

## 4. Build and CI

- [x] 4.1 `just zed-extension-check` を追加する
- [x] 4.2 Zed extension manifest validation を追加する
- [x] 4.3 package / build check を local で実行できるようにする
- [x] 4.4 release preflight または CI に Zed extension check を追加する
- [x] 4.5 `just VERSION=v0.18.2 release-check` に Zed extension check を組み込む

## 5. Verification

- [x] 5.1 `just fmt-check`
- [x] 5.2 `just lint`
- [x] 5.3 `just ast-lint`
- [x] 5.4 `cargo test --workspace --locked`
- [x] 5.5 `just dogfood`
- [x] 5.6 `git diff --check`
- [x] 5.7 `just zed-extension-check`
- [x] 5.8 `just VERSION=v0.18.2 release-check`

## Definition of Done

- [x] 6.1 Zed extension が `kml lsp` を Markdown language server として起動できる
- [x] 6.2 diagnostics / format / safe quick-fix が shared LSP contract に従っている
- [x] 6.3 Zed schema validation docs が published schema contract を参照している
- [x] 6.4 Zed extension check が local と CI で実行できる
- [x] 6.5 extension は thin wrapper のままになっている

## 品質評価スコア

| 項目 | 最大 | 現在 | 根拠 |
| --- | ---: | ---: | --- |
| Zed Scaffold | 20 | 20 | Zed extension manifest, source, configuration を追加した。 |
| LSP contract | 20 | 20 | `kml lsp` を Zed から起動し、diagnostics / format / codeAction を接続した。 |
| Documentation | 20 | 20 | README と `editor-integration.md` に Zed extension のセットアップ手順を追加した。 |
| Version Sync | 20 | 20 | 全ての構成要素で `0.18.2` への同期を完了した。 |
| CI/Verification | 20 | 20 | `just zed-extension-check` を追加し、release gate をパスした。 |
| 合計 | 100 | 100 | Zed extension MVP 実装完了。 |
