# VS Code Extension MVP Design

## Context

`kml lsp` は stdio の Language Server Protocol server として、
diagnostics、formatting、range formatting、code actions を返せる。
`docs/editor-integration.md` は VS Code の schema mapping を説明しているが、
Markdown file に `kml lsp` を attach する専用 extension はまだない。

この change では VS Code extension を最初の editor product surface とする。
extension は editor との接続だけを担当し、lint engine は `kml` binary に残す。

## Goals / Non-Goals

**Goals:**

- VS Code extension を repository 内に追加する
- extension が installed `kml` binary を見つけ、`kml lsp` を起動する
- Markdown diagnostics / format / safe quick-fix が VS Code から動く
- config file は published schema に関連付ける
- extension package と LSP 接続を smoke test できるようにする

**Non-Goals:**

- extension へ Rust binary や lint engine を bundle すること
- VS Code Marketplace への公開自動化
- Zed extension の実装
- Neovim plugin の実装
- `kml lsp` protocol の大規模な新機能追加

## Decisions

### D-1: extension は thin wrapper にする

extension は `kml lsp` を起動し、VS Code の Language Client と接続するだけにする。
rule、formatter、config validation の本体は Rust 側へ残す。

これにより CLI、LSP、npm / PyPI wrapper、future editor extensions が同じ実装を共有できる。

### D-2: 初期 binary discovery は `PATH` と明示設定に限定する

MVP では extension が binary download を行わない。
`kml` が `PATH` にある場合はそれを使い、必要に応じて user setting で absolute path を指定できるようにする。

自動 download、npx fallback、uvx fallback は security / latency / offline behavior が絡むため、
`v0.18.3` の hardening 候補に送る。

### D-3: extension code は TypeScript で小さく保つ

VS Code extension は TypeScript で実装する。
entrypoint、configuration reader、language client factory、schema association helper を責務で分ける。
`any` / `unknown` の濫用は避け、extension API の型で表現できない境界は専用型に閉じ込める。

### D-4: schema association は extension と docs の両方で扱う

extension は `.markdownlint.json` / `.markdownlint.jsonc` に published schema を関連付ける。
docs には extension を使わない manual mapping も残す。
これにより extension 未導入の利用者も config validation を使える。

### D-5: smoke test は UI 操作ではなく protocol と package contract を確認する

MVP の test は次を確認する。

- extension package が build できる
- extension manifest が Markdown と config file に activation できる
- test workspace で `kml lsp` が initialize できる
- diagnostics / format / quick-fix の protocol contract が既存 Rust test と一致する

## Risks / Trade-offs

- `kml` が未 install の場合に extension が無言で動かない
  - 明示的な error message と binary path setting を用意する
- extension package manager が Rust release flow とずれる
  - `Justfile` に editor-specific check target を追加し、release gate に含める
- schema association が user settings と競合する
  - extension の既定値は上書きしすぎず、manual setting を docs に残す
- marketplace 公開まで含めると scope が広がる
  - MVP は local package と smoke test までに限定する

## Migration Plan

1. VS Code extension scaffold を追加する
2. TypeScript test を先に追加し、binary missing / binary path / LSP launch の期待を固定する
3. Language Client 起動 logic を実装する
4. schema association と configuration を追加する
5. Justfile / CI に extension build と smoke target を追加する
6. README / editor integration docs を更新する

## Open Questions

- extension directory は `editors/vscode` と `extensions/vscode` のどちらにするか implementation 開始時に決める
- Marketplace 公開用 publisher 名は `v0.18.3` までに確定する
