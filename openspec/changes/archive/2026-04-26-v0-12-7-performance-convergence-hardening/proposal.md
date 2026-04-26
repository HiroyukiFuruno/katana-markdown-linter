# Performance Convergence Hardening

## Target Version

`v0.12.7`

## Why

`v0.12.5` と `v0.12.6` で parser / context migration を進めると、精度は上がる一方で parser index の構築 cost と rule migration の副作用が増える可能性がある。

安定版へ進む前に、速度、収束性、冪等性、release gate の実測を固める必要がある。

## What Changes

- parser / context index の性能を測定し、必要な hot path を最適化する
- `check` / `fix` / `fmt` の組み合わせで不要差分が増えないことを確認する
- link-heavy / inline-code-heavy / mixed corpus の benchmark baseline を安定化する
- fix collision と repeated run の収束性を強化する
- `v0.12.8` の安定版スコアリング前に、測定可能な品質 evidence を揃える

## Impact

- AST / parser 化による速度劣化を説明できる
- check / fix / fmt の信頼性を安定版評価に使える
- `v0.12.8` で主観ではなく score に基づいて受け入れ判断できる

## Non-Goals

- この change では配布、MCP Registry、remote MCP を進めない
- この change では新しい user-facing feature を追加しない
- この change では benchmark 数値のために rule semantics を弱めない
