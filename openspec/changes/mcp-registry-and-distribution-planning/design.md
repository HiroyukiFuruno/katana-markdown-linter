# Design

## Current Position

`kml-mcp` は Rust binary として配布され、ローカル MCP client から標準入出力
（stdio）で利用する。この形は Codex、Claude Code、Antigravity のような
デスクトップまたは CLI の client に向いている。

ただし、この distribution planning は安定版受け入れ後の作業である。
`v0.12.8` の stable score が 90 点未満、hard blocker が 1 件以上、ユーザー受け入れが未記録、または `v0.12.9` の public confidence gate が未完了の場合は、この change を開始しない。

一方で、LLM provider API から直接呼ぶ場合は、provider が接続できる遠隔 MCP
サーバー（remote MCP server）が必要になる。local stdio server は hosted API
から直接参照できない。

## Package Type Evaluation

この change では、実装前に package type を評価して第一候補を決める。

| Package type | 初期評価 | 判断軸 |
| --- | --- | --- |
| MCPB | 有力 | local stdio binary を desktop client に配る形と合うか |
| OCI image | 有力 | GHCR で配れるが workspace mount と write policy の説明が必要 |
| npm wrapper | 低 | Rust binary への wrapper が必要で primary distribution とずれる |
| PyPI wrapper | 低 | Python distribution ではなく ownership が不自然 |
| NuGet | 低 | .NET distribution ではない |

判断結果は `docs/mcp-server.md` か配布文書に残す。
「調査したが選ばなかった理由」も future maintainer が追えるように記録する。

## Registry Metadata Shape

`server.json` は Registry metadata として扱う。server artifact の実体は、選定した
package registry または release artifact を参照する。

草案で決める項目:

- server name と display name
- package type と install command
- binary name `kml-mcp`
- required feature `mcp`
- workspace root の指定方法
- destructive write が default off であること
- documentation URL
- security / workspace policy への導線

## Public Readiness Gate

Registry / Hub 公開は、少なくとも以下を満たすまで deferred とする。

- `v0.12.8` で stable score 90 点以上、hard blocker 0 件、ユーザー受け入れが記録されている
- `v0.12.9` で public confidence score 90 点以上、release-blocking issue 0 件が記録されている
- `make mcp-stdio-smoke` が release gate に含まれている
- file write は preview と explicit apply に分かれている
- README / docs が install、safety、client config を説明している
- rule check / fix coverage の現在地が誤解なく表示されている
- `server.json` の install path が不要な wrapper を強制しない
- 公開前 security review で workspace access policy を再確認している

## Follow-Up Split

この change の結果、後続 change を次のように分ける。

- `v0-14-0-mcp-package-and-registry-publication`: 選定済み配布物と Registry 公開
- `v0-15-0-remote-mcp-transport`: 遠隔 MCP 接続が必要になった場合の transport

local stdio と remote transport は安全境界が違うため、同じ実装 change に混ぜない。
