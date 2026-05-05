# Editor Publication Readiness Triage

## 背景

- `v0.18.7` は公開事故により再公開できないため、永久欠番として扱う。
- `v0.18.8` へ飛ばして継続するのではなく、次の開発・公開方針を整理する必要がある。
- 調査の結果、現行の VS Code / Zed 拡張は `kml lsp` の起動ラッパーに近い。
- editor 上の診断・修正は `LintOptions::default()` を使っており、`.markdownlint.json` / `.markdownlint.jsonc` の設定を反映していない。
- そのため、Marketplace 公開をこの change の成果として扱うのは不適切。

## 方針

1. この change は version 付き release 実装ではなく、公開可否の調査結果と後続タスクを固定する triage change として扱う。
2. `v0.18.7` は永久欠番とし、`v0.18.8` へは進めない。
3. VS Code / Zed の Marketplace 公開は、設定反映済み診断・整形・安全な修正・最終 dogfood が揃うまで保留する。
4. `v0.19.0` 系の editor 機能開発と `v0.20.0` の公開準備は、この change では実装しない。別 OpenSpec change として切り出す。
5. この change の成果は、調査結果、No-Go 判断、後続change作成タスク、release gate 上の停止条件を明文化することに限定する。

## 成果物

- 現行 VS Code / Zed 拡張でできること・できないことの整理。
- Marketplace 公開を保留する判断の記録。
- `v0.19.0` 系と `v0.20.0` 用の別 OpenSpec change を作るためのタスク台帳。
- `v0.18.7` 永久欠番と `v0.18.8` 不使用の release policy。
