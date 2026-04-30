# Zed Extension MVP Design

## Context

`v0.18.1` で VS Code extension が `kml lsp` を thin wrapper として起動する。
Zed も同じ `kml lsp` contract に乗せたいが、extension format と language server registration は
VS Code と異なる。

この change は Zed 固有の boundary を検証する。
共有 engine と LSP behavior は Rust 側へ残し、Zed extension は起動と登録だけを担当する。

## Goals / Non-Goals

**Goals:**

- Zed extension scaffold を repository に追加する
- Zed extension から `kml lsp` を起動する
- Markdown file に diagnostics / format / safe quick-fix を接続する
- schema-backed config editing の docs を Zed 向けに更新する
- Zed extension build / smoke check を local と CI で実行できるようにする

**Non-Goals:**

- Zed extension へ lint logic を実装すること
- Zed marketplace 公開自動化
- VS Code extension の設計変更
- `kml lsp` の editor-specific fork
- Neovim plugin 実装

## Decisions

### D-1: Zed extension は VS Code extension の LSP contract を再利用する

Zed extension は `kml lsp` の initialize / diagnostics / formatting / codeAction contract に依存する。
Zed 固有の処理は language server registration と configuration に限定する。

### D-2: implementation 開始時に Zed の公式 extension API を確認する

Zed extension API は editor 側の仕様に依存する。
この change の実装開始時に、公式 docs で manifest、language server registration、
local development command を確認してから scaffold を確定する。

### D-3: binary discovery は VS Code MVP と同じ方針にする

MVP は installed `kml` を使う。
`PATH` と明示 path setting だけを対象にし、binary download、npx fallback、uvx fallback は
hardening change に送る。

### D-4: Zed schema support は docs と extension の責務を分ける

Zed の JSON schema mapping は docs に明記する。
extension が schema association を直接管理できる場合は追加するが、
MVP の必須条件は Markdown LSP 接続とする。

### D-5: smoke check は package boundary と LSP 起動を重視する

Zed extension の UI 操作自動化は MVP の必須にしない。
代わりに extension package check、manifest validation、`kml lsp` launch contract を確認する。

## Risks / Trade-offs

- Zed extension API の変更で scaffold が壊れる
  - implementation 開始時に公式 docs を確認し、tasks に検証結果を残す
- Zed の schema association が extension から直接設定できない
  - docs-based config を MVP の受け入れ条件に含める
- VS Code と Zed で configuration key がずれる
  - user-facing docs では editor ごとの設定名を分け、Rust 側 contract は共有する
- marketplace 公開を含めると scope が広がる
  - MVP は local package と smoke check までに限定する

## Migration Plan

1. Zed official docs で extension scaffold と language server registration を確認する
2. Zed extension scaffold を追加する
3. `kml` binary path 設定と LSP 起動を実装する
4. package / manifest / launch smoke check を追加する
5. README / editor integration docs を Zed extension 向けに更新する
6. CI と release gate に Zed extension check を追加する

## Open Questions

- Zed extension directory は VS Code extension と同じ parent directory に揃える
- Zed package の公開名と公開先は `v0.18.3` で確定する
