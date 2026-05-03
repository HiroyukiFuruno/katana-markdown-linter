# Tasks

## Definition of Ready

- [x] 0.1 `v0.18.0` の schema publication change が完了している
- [x] 0.2 `kml lsp` の diagnostics / format / quick-fix contract が現行 test で確認できる
- [x] 0.3 VS Code extension の directory 名を決める
- [x] 0.4 VS Code extension の publisher / package name は MVP では公開しない前提にする

## 1. Extension Test Scaffold

- [x] 1.1 VS Code extension package scaffold を追加する
- [x] 1.2 TypeScript compile check を追加する
- [x] 1.3 binary missing 時の error test を追加する
- [x] 1.4 explicit `kml` path 設定の test を追加する
- [x] 1.5 LSP initialize smoke test を追加する

## 2. LSP Client Implementation

- [x] 2.1 extension activation を Markdown document と config file に限定する
- [x] 2.2 configuration reader を実装する
- [x] 2.3 Language Client factory を実装する
- [x] 2.4 `kml lsp` を stdio 起動する
- [x] 2.5 output channel に起動失敗理由を出す
- [x] 2.6 extension 側に lint logic を置かないことを test または static check で確認する

## 3. Schema Association

- [x] 3.1 `.markdownlint.json` の schema association を追加する
- [x] 3.2 `.markdownlint.jsonc` の schema association を追加する
- [x] 3.3 published schema URL を extension manifest または configuration で参照する
- [x] 3.4 manual schema mapping docs を残す

## 4. Build and CI

- [x] 4.1 `just vscode-extension-check` を追加する
- [x] 4.2 release preflight または CI に extension check を追加する
- [x] 4.3 extension package の file list が不要物を含まないことを検証する
- [x] 4.4 `just VERSION=v0.18.1 release-check` に extension check を組み込む

## 5. Documentation

- [x] 5.1 README に VS Code extension MVP の install / setup を追加する
- [x] 5.2 `docs/editor-integration.md` に VS Code extension と manual schema mapping の違いを書く
- [x] 5.3 binary path setting と troubleshooting を docs に追加する
- [x] 5.4 Marketplace 公開は後続 hardening change に送ることを明記する
- [x] 5.5 `CHANGELOG.md` に `v0.18.1` の editor MVP を追加する

## 6. Verification

- [x] 6.1 `just fmt-check`
- [x] 6.2 `just lint`
- [x] 6.3 `just ast-lint`
- [x] 6.4 `cargo test --workspace --locked`
- [x] 6.5 `just dogfood`
- [x] 6.6 `git diff --check`
- [x] 6.7 `just vscode-extension-check`
- [x] 6.8 `just VERSION=v0.18.1 release-check`

## Definition of Done

- [x] 7.1 VS Code extension が `kml lsp` を起動できる
- [x] 7.2 Markdown diagnostics / format / safe quick-fix が LSP 経由で使える
- [x] 7.3 `.markdownlint.json` / `.markdownlint.jsonc` が published schema に関連付く
- [x] 7.4 extension package check が local と CI で実行できる
- [x] 7.5 lint logic は Rust 側に残り、extension は thin wrapper のままになっている
