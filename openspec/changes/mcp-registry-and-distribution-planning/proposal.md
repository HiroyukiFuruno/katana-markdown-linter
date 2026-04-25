# MCP Registry And Distribution Planning

## Summary

`kml-mcp` は 0.12.0 で native Rust stdio server として成立する。
一方で、MCP Registry / Hub への登録は discovery を広げる効果があるが、
現在の check / fix 精度と機能セットでは公開を急がない。

この change では公開作業そのものは行わず、公式 MCP Registry と周辺 Hub へ
登録できる状態にするための配布方式、検証条件、公開判断基準を定義する。

## Motivation

- MCP Registry は metadata registry であり、server artifact 自体は別 package
  registry に置く必要がある。

- 公式 MCP Registry は preview で、下流 aggregator が定期的に参照する前提である。

- crates.io package は現時点の公式 MCP Registry package type では直接扱えない。

- `kml-mcp` は local stdio server なので、LLM provider API の remote MCP connector
  から直接利用するには remote transport か wrapper distribution が必要になる。

## Scope

- 公式 MCP Registry 登録に必要な `server.json` 方針を決める。
- package type 候補を比較し、`kml-mcp` に合う配布方式を選ぶ。
- 公開前に必要な品質基準を定義する。
- Registry / Hub 公開を future work として明確に deferred にする。

## Non-Goals

- この change では MCP Registry へ publish しない。
- この change では Docker image / MCPB artifact / npm wrapper を実装しない。
- この change では remote HTTP MCP transport を実装しない。
- この change では KatanA 固有の adapter を追加しない。

## References

- [Official MCP Registry quickstart](https://modelcontextprotocol.io/registry/quickstart)
- [Supported package types](https://modelcontextprotocol.io/registry/package-types)
- [Registry aggregators](https://modelcontextprotocol.io/registry/registry-aggregators)
