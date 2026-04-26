# AST Readiness And Parser Precision Hardening

## Target Version

`v0.12.5`

## Why

`v0.12.1` 以降の patch は、誤検知、検出漏れ、fix 事故、速度劣化の芽を潰すために続けてきた。
その主因は、構文依存 rule が単一行文字列と rule ごとの小さな手書き parser に依存していることだった。

`v0.13.0` で配布展開へ戻る前に、`v0.12.x` を安定版へ近づける。
`v0.12.5` はその開始点として、AST 化の受け皿と共有 Markdown 構文解析を作る。

## What Changes

- `v0.12.x` を精度、速度、安定性だけの系列として再定義する
- AST 化の前提になる source-preserving parser の責務を決める
- inline code、link、image、reference definition の共有 token を `DocumentContext` から参照できるようにする
- `MD034`、`MD051`、`MD052`、`MD054`、`MD059` の手書き解析を共有 parser へ寄せる
- link-heavy / inline-code-heavy の性能計測を追加する
- `v0.13.0` の配布計画は `v0.12.8` の安定版受け入れ後まで凍結する

## Impact

- 構文依存 rule の精度改善が rule ごとの場当たり修正になりにくくなる
- AST 化しない箇所も、なぜしないのかを明文化できる
- 性能計測が実際の不安定箇所に近づく
- 配布展開の前に linter 本体の品質を評価できる

## Non-Goals

- この change では配布、MCP Registry、remote MCP を進めない
- この change では全 rule の完全 AST 化を完了条件にしない
- この change では unsafe fix の対象を広げない
- この change では public API に AST 型を公開しない
