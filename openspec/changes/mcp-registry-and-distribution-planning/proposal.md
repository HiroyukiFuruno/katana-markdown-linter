# MCP Registry And Distribution Planning

## Target Version

`v0.13.0`

## Why

`kml-mcp` は `v0.12.0` で、ローカル標準入出力サーバー（stdio server）
として実用できる状態になった。

ただし `v0.12.1` 以降の patch で明らかになった通り、安定版と呼ぶには
linter 本体の精度、速度、収束性を先に数値で評価する必要がある。
そのため、この change は `v0.12.8` で stable score 90 点以上、hard blocker 0 件、
ユーザー受け入れが記録されるまで着手しない。

次の課題は、MCP Registry と周辺の一覧サービス（Hub / aggregator）に載せる前に、
どの配布物を正とするか、どこまでを公開前の合格条件にするかを決めることだ。

急いで登録だけを行うと、利用者には「すぐ導入できる公式配布」のように見える。
しかし Registry は実体を置く場所ではなく、配布物への案内情報を持つ場所である。
そのため `server.json`、配布方式、検証手順、公開判断を先に固定する。

## What Changes

- MCP Registry / Hub へ公開する前の判断基準を定義する
- `server.json` の草案と ownership 確認方法を決める
- MCPB、OCI image、npm/PyPI wrapper の適合性を比較する
- crates.io の `kml-mcp` binary と Registry package type の関係を明文化する
- API から直接呼ぶ遠隔 MCP 接続（remote MCP transport）は別 change に分離する

## Impact

- `kml-mcp` の公開判断が、個人判断ではなく OpenSpec 上の gate で追える
- 次の実装 change は、配布物と `server.json` の両方を同じ前提で扱える
- local stdio MCP と遠隔 MCP 接続を混同しなくなる

## Non-Goals

- `v0.12.8` の安定版受け入れ前にこの change を進めない
- この change では MCP Registry / Hub へ公開しない
- この change では MCPB / OCI image / npm wrapper / PyPI wrapper を実装しない
- この change では遠隔 MCP 接続（remote MCP transport）を実装しない
- この change では KatanA 固有の adapter を追加しない

## References

- [Official MCP Registry quickstart](https://modelcontextprotocol.io/registry/quickstart)
- [Supported package types](https://modelcontextprotocol.io/registry/package-types)
- [Registry aggregators](https://modelcontextprotocol.io/registry/registry-aggregators)
