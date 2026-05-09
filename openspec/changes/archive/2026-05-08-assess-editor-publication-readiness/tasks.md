# Tasks

## 目標

- `v0.18.7` 永久欠番と `v0.18.8` 不使用を固定する。
- 現行 VS Code / Zed 拡張の公開可否を調査結果として整理する。
- Marketplace 公開 No-Go の理由を、後続作業へ引き継げる形でタスク化する。
- この change 自体から release version の意味を外し、後続の versioned change へ分割する。

## 1. 調査結果の固定

- [x] 1.1 `v0.18.7` を再公開不可の事故版として記録する
- [x] 1.2 `v0.18.8` へ進めない判断を記録する
- [x] 1.3 VS Code 拡張が `kml lsp` を起動する薄い wrapper であることを確認する
- [x] 1.4 Zed 拡張が `kml lsp` を起動する薄い wrapper であることを確認する
- [x] 1.5 LSP の診断・修正が `LintOptions::default()` に依存し、project config を反映していないことを確認する

## 2. 公開判断

- [x] 2.1 現行拡張では `kml` CLI と同等の主要機能を提供できないことを記録する
- [x] 2.2 editor 上で `.markdownlint.json` / `.markdownlint.jsonc` を反映できないことを Marketplace 公開 blocker とする
- [x] 2.3 editor 上で設定通りの診断と安全な修正ができるまで公開しない方針を記録する
- [x] 2.4 効果確認だけでは足りず、最終 dogfood を公開前必須条件にする

## 3. change 分割

- [/] 3.1 現行 change から `v0.19.0` 固有の release 実装計画を外し、調査・task 化 change として扱う
- [x] 3.2 `v0.19.0` 系の editor 機能開発 change を別途作成する
- [x] 3.3 `v0.19.0` 系の最終 editor dogfood を `v0.19.0` 系 change の完了条件に含める
- [x] 3.4 `v0.20.0` Marketplace 公開 change を別途作成する
- [x] 3.5 上記 change 間の依存順序を active roadmap に反映する

## 4. 後続changeへ渡す必須要件

- [x] 4.1 LSP が CLI と同じ config 探索・解決を使うこと
- [x] 4.2 VS Code / Zed の診断が project config を反映すること
- [x] 4.3 VS Code / Zed の安全な修正候補が project config を反映すること
- [x] 4.4 VS Code / Zed の整形が公開前に実操作で検証されること
- [x] 4.5 editor dogfood で診断・整形・安全な修正・設定反映をまとめて検証すること
- [x] 4.6 誤診断・誤修正・設定反映漏れが残る場合は `v0.20.0` 公開へ進まないこと

## 5. Evidence

- [x] 5.1 現行拡張の能力差分をコード参照付きで確認する
- [x] 5.2 `just ast-lint` を実行する
- [x] 5.3 `scripts/openspec validate assess-editor-publication-readiness --strict` を実行する
- [x] 5.4 change rename 後の参照漏れがないことを確認する
- [x] 5.5 最終 diff を確認し、versioned release change と誤読されないことを確認する

## Definition of Done

- [x] D1 この change が versioned release 実装ではなく、調査・task 化 change として読めること
- [x] D2 `v0.19.0` 系と `v0.20.0` の実装・公開作業が別 change の未完了タスクとして残っていること
- [x] D3 Marketplace 公開 No-Go の理由が、設定反映・安全な修正・最終 dogfood の観点で説明できること
- [x] D4 OpenSpec validate と Markdown 系 lint が通っていること
