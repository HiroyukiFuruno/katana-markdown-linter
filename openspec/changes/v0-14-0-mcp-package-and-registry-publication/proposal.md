# MCP Package And Registry Publication

## Target Version

`v0.14.0`

## Why

`v0.13.0` で MCP 配布方式と公開 gate を決めた後は、実際に利用者が導入できる
配布物と Registry metadata を作る必要がある。

Registry だけを先に公開しても、install path が曖昧なら利用者は迷う。
この change では、選定済み package type を実装し、`server.json` を公開可能な形にし、
release gate と docs で検証してから Registry / Hub 公開へ進む。

## What Changes

- `v0.13.0` で選んだ package type に従って `kml-mcp` 配布物を作る
- `server.json` を実装し、local stdio server としての起動方法を明記する
- package artifact の smoke test を release gate に追加する
- Registry / Hub 公開手順を release runbook に追加する
- 公開後の verification と rollback 判断を文書化する

## Impact

- MCP client 利用者が `kml-mcp` を discovery しやすくなる
- 配布物、Registry metadata、docs、release gate が同じ導入経路を指す
- local stdio MCP の範囲を超える remote support を誤って宣伝しなくなる

## Non-Goals

- 遠隔 MCP 接続（remote MCP transport）の実装
- KatanA 固有 adapter の実装
- directory-wide automatic fix apply の解禁
- wrapper package を primary distribution にすること。ただし `v0.13.0` の選定で必要と判断された場合は例外とする
