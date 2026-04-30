# Tasks

## Definition of Ready

- [ ] 0.1 `v0.18.1` の VS Code extension MVP が完了している
- [ ] 0.2 `editor-lsp-contract` の requirements が満たされている
- [ ] 0.3 Zed の公式 extension API と local development command を確認する
- [ ] 0.4 Zed extension の directory 名を VS Code extension と揃える

## 1. Zed Extension Scaffold

- [ ] 1.1 Zed extension manifest を追加する
- [ ] 1.2 Zed extension source を追加する
- [ ] 1.3 extension が `kml lsp` を起動する最小実装を追加する
- [ ] 1.4 explicit `kml` path setting を追加する
- [ ] 1.5 binary missing 時の setup error を追加する

## 2. LSP Registration

- [ ] 2.1 Markdown file へ `kml lsp` を登録する
- [ ] 2.2 diagnostics が `kml lsp` から表示されることを smoke で確認する
- [ ] 2.3 format / range format の provider 設定を確認する
- [ ] 2.4 safe quick-fix の code action provider 設定を確認する
- [ ] 2.5 extension 側に lint logic を置かないことを確認する

## 3. Schema and Documentation

- [ ] 3.1 Zed の schema mapping docs を published schema contract に合わせて更新する
- [ ] 3.2 Zed extension が必要な範囲と不要な範囲を docs で分ける
- [ ] 3.3 binary path setting と troubleshooting を docs に追加する
- [ ] 3.4 Marketplace 公開は後続 hardening change に送ることを明記する

## 4. Build and CI

- [ ] 4.1 `make zed-extension-check` を追加する
- [ ] 4.2 Zed extension manifest validation を追加する
- [ ] 4.3 package / build check を local で実行できるようにする
- [ ] 4.4 release preflight または CI に Zed extension check を追加する
- [ ] 4.5 `make release-check VERSION=v0.18.2` に Zed extension check を組み込む

## 5. Verification

- [ ] 5.1 `make fmt-check`
- [ ] 5.2 `make lint`
- [ ] 5.3 `make ast-lint`
- [ ] 5.4 `cargo test --workspace --locked`
- [ ] 5.5 `make dogfood`
- [ ] 5.6 `git diff --check`
- [ ] 5.7 `make zed-extension-check`
- [ ] 5.8 `make release-check VERSION=v0.18.2`

## Definition of Done

- [ ] 6.1 Zed extension が `kml lsp` を Markdown language server として起動できる
- [ ] 6.2 diagnostics / format / safe quick-fix が shared LSP contract に従っている
- [ ] 6.3 Zed schema validation docs が published schema contract を参照している
- [ ] 6.4 Zed extension check が local と CI で実行できる
- [ ] 6.5 extension は thin wrapper のままになっている
