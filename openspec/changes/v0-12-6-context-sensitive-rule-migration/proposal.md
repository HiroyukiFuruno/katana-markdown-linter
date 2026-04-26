# Context Sensitive Rule Migration

## Target Version

`v0.12.6`

## Why

`v0.12.5` で link / inline code / reference の parser readiness を作る。
次は、その受け皿を使って構文依存 rule を広く移譲する必要がある。

安定版を目指すには、既知の誤検知だけを潰すのでは足りない。
同じ種類のバグが再発しない構造へ移す必要がある。

## What Changes

- context-sensitive rule を全件棚卸しする
- rule ごとに `parser-backed`、`document-context-backed`、`line-local-by-spec`、`future-work` を分類する
- link 以外の HTML、table、list、heading、emphasis、command prompt 系の文脈判定を `DocumentContext` へ寄せる
- mixed Markdown fixture を rule family ごとに拡張する
- migration しない rule の理由と後続条件を tasks に残す

## Impact

- 精度改善が個別 rule の小修正で終わらなくなる
- code block / inline code / HTML / table の除外判定が一貫する
- `v0.12.8` の安定版スコアで評価できる母集団が揃う

## Non-Goals

- この change では配布、MCP Registry、remote MCP を進めない
- この change では全 rule を外部 AST parser に置き換えない
- この change では unsafe fix を増やさない
- この change では public API を破壊しない
