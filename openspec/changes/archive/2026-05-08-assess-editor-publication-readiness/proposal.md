# Editor Publication Readiness Triage

## Why

- `v0.18.7` は公開事故により再公開できない状態となったため、永久欠番として明示した上で次段階の方針を整理する必要がある。
- `v0.18.8` へ即時継続するべきではなく、現行の VS Code / Zed 拡張の公開可否を先に triage で固定する必要がある。
- 調査結果、現行拡張は `kml lsp` の起動ラッパーに留まり、`LintOptions::default()` 依存で設定反映が不十分であることが確認できた。
- このまま marketplace 公開を進めると、設定不整合や安全修正差分の不一致を持ち込むリスクが高いため、公開前提条件を先に明確化する必要がある。

## What Changes

- `v0.18.7` の再公開不可を明文化し、`v0.18.8` へ飛ばさない運用制約を確定する。
- 現行 VS Code / Zed 拡張では marketplace 公開を保留する No-Go を明示する。
- `v0.19.0` 系 editor 機能開発と `v0.20.0` 公開準備を別 change に切り分ける。
- 後続作業として、config 反映済み診断・整形・安全な修正・最終 dogfood を必須条件として tasks に固定する。
- この change は仕様実装を含めず、公開 readiness triage と次行動の分割を文書化する。

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
