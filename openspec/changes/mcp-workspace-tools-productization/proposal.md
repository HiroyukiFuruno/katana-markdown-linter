## Why

`kml-mcp` は native Rust の experimental MCP adapter として text-first tools を持つが、実運用で使うには workspace file/directory 対象、path safety、config loading、dry-run/fix policy、client 設定例が不足している。

KatanA や他の Rust 組み込み利用者とは独立した共通 linter として、MCP server も core crate から分離した adapter のまま実用化する。

## What Changes

- `kml-mcp` に workspace-scoped file/directory check tools を追加する
- destructive fix は default off とし、dry-run diff または explicit apply に分ける
- path traversal / symbolic path / ignored file の扱いを明確にする
- `.markdownlint.json` / `.markdownlint.jsonc` config loading を MCP tool contract に含める
- Codex / ClaudeCode / Antigravity の client 設定例を docs に追加する

## Impact

- MCP 経由で `kml` を repository linter として使いやすくなる
- core library は MCP を知らないまま維持される
- file mutation の安全性を tool contract と tests で固定できる
