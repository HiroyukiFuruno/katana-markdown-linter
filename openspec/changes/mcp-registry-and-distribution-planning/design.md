# Design

## Current Position

`kml-mcp` は Rust binary として配布され、local MCP client から stdio で利用する。
この形は Codex / Claude Code / Antigravity などの desktop / CLI client に向いている。

LLM provider API から直接 MCP として使う場合は、provider が接続できる remote MCP
server が必要になる。local stdio server は hosted API から直接参照できない。

## Registry Reality

Official MCP Registry は server artifact を hosting しない。
`server.json` は実体 package への参照を持つ metadata である。

現時点で検討対象になる package type:

| Package type | Fit for kml | Notes |
| --- | --- | --- |
| npm | Low | Rust binary wrapper が必要で、primary distribution から外れる。 |
| PyPI | Low | Python wrapper が必要で、ownership / install path が不自然。 |
| NuGet | Low | .NET distribution ではない。 |
| Docker/OCI | Medium | GHCR で配布しやすいが、local file access と workspace mount の設計が必要。 |
| MCPB | Medium | Desktop client distribution と相性があるが、artifact format と install UX の調査が必要。 |

## Preferred Direction

Phase 1 は MCPB と GHCR の比較検証に留める。

推奨順:

1. MCPB が `kml-mcp` の local stdio binary 配布に自然か確認する。
2. GHCR image は remote / isolated execution 向けとして検討する。
3. npm / PyPI wrapper は最後の手段とする。

## Public Readiness Gate

Registry / Hub 公開は以下を満たすまで deferred とする。

- `mcp-stdio-smoke` が release gate に含まれている。
- workspace write path が preview / explicit apply に限定されている。
- README / docs が install, safety, client config を説明している。
- rule check / fix coverage の現在地が誤解なく表示されている。
- `server.json` の install path がユーザーに不要な wrapper を強制しない。
- 公開前に security review を実施し、workspace access policy を再確認する。

## Future Remote MCP Option

Provider API から直接使う用途が明確になった場合、remote HTTP MCP transport を別 change
で設計する。local stdio と remote HTTP は安全境界が違うため、同じ change に混ぜない。
