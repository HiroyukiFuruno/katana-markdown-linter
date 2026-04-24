## Why

`katana-markdown-linter` は Rust 組み込みを主目的にしているが、agent や editor から lint / fix / rule introspection を呼び出す用途では MCP server 化の価値がある。
一方で core crate が MCP や特定 consuming application を知る設計にすると責務が崩れるため、先に境界と採用判断を固める必要がある。

## What Changes

- MCP server 化の価値、利用シナリオ、非対象を評価する
- core crate、CLI、MCP server の責務境界を定義する
- MCP tool / resource / prompt として公開する候補を整理する
- Rust MCP SDK の採用可否、crate 構成、feature flag、binary 名の候補を比較する
- security / workspace access / fix operation の制約を明文化する
- 実装に進む場合の次 change に渡せる DoR / DoD を作る

## Capabilities

### New Capabilities

- `mcp-integration`: `kml` を MCP server として公開する場合の contract、境界、評価条件を定義する

### Modified Capabilities

- なし

## Impact

- `docs/`
- future `kml-mcp` binary または optional feature の設計
- public library API の安定性
- workspace file access と fix operation の safety policy
- agent / editor integration の利用導線
