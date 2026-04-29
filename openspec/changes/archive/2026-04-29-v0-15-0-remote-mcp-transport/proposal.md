# Remote MCP Transport

## Target Version

`v0.15.0`

## Why

`kml-mcp` は local stdio MCP server として成立している。
これはローカルの agent や editor が repository を lint する用途に合う。

一方で、LLM provider API から直接 MCP server を呼ぶ用途では、標準入出力
（stdio）ではなく、provider が到達できる遠隔接続（remote transport）が必要になる。
local workspace への読み書き権限も安全境界が変わるため、Registry 配布や stdio server
とは別 change で設計する。

## What Changes

- 遠隔 MCP 接続を提供するかどうかの product boundary を決める
- remote server の transport、auth、workspace access policy を設計する
- text-only remote と workspace-backed remote の違いを明確にする
- core crate は MCP-free のまま維持する
- local stdio server の tool contract と互換にできる範囲を定義する

## Impact

- API-hosted LLM から `kml` を使う条件を誤解なく説明できる
- remote deployment が必要な場合でも、local workspace write policy を雑に流用しない
- hosted service、self-hosted server、library embedding の使い分けが明確になる

## Non-Goals

- `v0.14.0` の Registry / package publication
- directory-wide automatic fix apply
- anonymous public write-capable hosted service
- KatanA 固有 adapter
