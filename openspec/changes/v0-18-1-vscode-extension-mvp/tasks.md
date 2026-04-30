# Tasks

## Definition of Ready

- [ ] 0.1 `v0.18.0` の schema publication change が完了している
- [ ] 0.2 `kml lsp` の diagnostics / format / quick-fix contract が現行 test で確認できる
- [ ] 0.3 VS Code extension の directory 名を決める
- [ ] 0.4 VS Code extension の publisher / package name は MVP では公開しない前提にする

## 1. Extension Test Scaffold

- [ ] 1.1 VS Code extension package scaffold を追加する
- [ ] 1.2 TypeScript compile check を追加する
- [ ] 1.3 binary missing 時の error test を追加する
- [ ] 1.4 explicit `kml` path 設定の test を追加する
- [ ] 1.5 LSP initialize smoke test を追加する

## 2. LSP Client Implementation

- [ ] 2.1 extension activation を Markdown document と config file に限定する
- [ ] 2.2 configuration reader を実装する
- [ ] 2.3 Language Client factory を実装する
- [ ] 2.4 `kml lsp` を stdio 起動する
- [ ] 2.5 output channel に起動失敗理由を出す
- [ ] 2.6 extension 側に lint logic を置かないことを test または static check で確認する

## 3. Schema Association

- [ ] 3.1 `.markdownlint.json` の schema association を追加する
- [ ] 3.2 `.markdownlint.jsonc` の schema association を追加する
- [ ] 3.3 published schema URL を extension manifest または configuration で参照する
- [ ] 3.4 manual schema mapping docs を残す

## 4. Build and CI

- [ ] 4.1 `make vscode-extension-check` を追加する
- [ ] 4.2 release preflight または CI に extension check を追加する
- [ ] 4.3 extension package の file list が不要物を含まないことを検証する
- [ ] 4.4 `make release-check VERSION=v0.18.1` に extension check を組み込む

## 5. Documentation

- [ ] 5.1 README に VS Code extension MVP の install / setup を追加する
- [ ] 5.2 `docs/editor-integration.md` に VS Code extension と manual schema mapping の違いを書く
- [ ] 5.3 binary path setting と troubleshooting を docs に追加する
- [ ] 5.4 Marketplace 公開は後続 hardening change に送ることを明記する
- [ ] 5.5 `CHANGELOG.md` に `v0.18.1` の editor MVP を追加する

## 6. Verification

- [ ] 6.1 `make fmt-check`
- [ ] 6.2 `make lint`
- [ ] 6.3 `make ast-lint`
- [ ] 6.4 `cargo test --workspace --locked`
- [ ] 6.5 `make dogfood`
- [ ] 6.6 `git diff --check`
- [ ] 6.7 `make vscode-extension-check`
- [ ] 6.8 `make release-check VERSION=v0.18.1`

## Definition of Done

- [ ] 7.1 VS Code extension が `kml lsp` を起動できる
- [ ] 7.2 Markdown diagnostics / format / safe quick-fix が LSP 経由で使える
- [ ] 7.3 `.markdownlint.json` / `.markdownlint.jsonc` が published schema に関連付く
- [ ] 7.4 extension package check が local と CI で実行できる
- [ ] 7.5 lint logic は Rust 側に残り、extension は thin wrapper のままになっている
