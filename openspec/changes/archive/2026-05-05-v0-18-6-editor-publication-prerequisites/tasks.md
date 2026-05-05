# Tasks

## Definition of Ready

- [x] 0.1 `v0.18.5` の verification hardening が完了している

## v0.18.6 Execution Readiness

- [x] 1.1 VS Code publish 手順の事前チェックを定義する
- [x] 1.2 Zed publish 手順の事前チェックを定義する
- [x] 1.3 Neovim docs-only 方針を `docs/editor-integration.md` へ固定する
- [x] 1.4 条件未達時の停止ルートを runbook へ追加する

## Definition of Done

- [x] 2.1 publish 前提チェックが実行前に満たされるまで停止すること
- [x] 2.2 `Neovim` が docs-only 方針で一貫していること
- [x] 2.3 3 change（v0.18.4/5/6）の条件整合が roadmap とドキュメントで一致すること

## 品質評価スコア

| 項目 | 重量 | スコア | 備考 |
| --- | --- | --- | --- |
| 動作整合性 | 40 | 40 | 公開ゲートのメタデータ検証が正常に動作することを確認済み。 |
| 文書整合性 | 30 | 30 | runbook, roadmap, integration docs の一貫性を確保。 |
| リリース安全性 | 30 | 30 | 条件未達時の停止（fail-fast）を実装。 |
| 合計 | 100 | 100 | |
