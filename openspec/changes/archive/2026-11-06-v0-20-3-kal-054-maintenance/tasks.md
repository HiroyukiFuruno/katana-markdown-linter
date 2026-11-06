## 1. Implementation

- [x] 1.1 PR #142 の署名と必須 CI を再確認してマージし、Issue #139 を閉じる。
- [x] 1.2 KAL の境界回帰と配布メタデータを v0.20.3 にそろえる。
- [x] 1.3 依存関係と Actions の固定バージョンを監査する。

## 2. Verification and delivery

- [x] 2.1 既存のリリース前品質ゲート、上流比較、厳格な性能検査を通す。
- [x] 2.2 Draft レビュー、指摘への対応、必須 CI と署名を確認してマージする。
- [x] 2.3 公開実行、署名付きタグ、各配布先と成果物を検証する。

## 品質評価スコア

| 項目 | 最大 | 現在 |
| --- | --- | --- |
| 実装と回帰 | 40 | 40 |
| リリース前検証 | 30 | 30 |
| レビュー、CI と公開 | 30 | 30 |
| 合計 | 100 | 100 |

## Evidence

- [実装 PR #142](https://github.com/HiroyukiFuruno/katana-markdown-linter/pull/142) の必須 CI 4件と署名を確認してマージし、[Issue #139](https://github.com/HiroyukiFuruno/katana-markdown-linter/issues/139) を閉じた。
- リリース前検証、上流互換・ドリフト検査、ワークフロー静的検査、Node.js 26 の npm パッケージ検証が成功した。
- 正解データ272件の不一致は0件。未実行815行で既存上限881行以内、性能20ケースで既存上限1.40倍以内。

- [公開準備 PR #143](https://github.com/HiroyukiFuruno/katana-markdown-linter/pull/143) は Draft と Ready のレビューで指摘0件、必須 CI 4件成功、署名確認後にマージした。
- [公開ワークフロー](https://github.com/HiroyukiFuruno/katana-markdown-linter/actions/runs/42184571140) は初回で全8ジョブ成功。署名付きタグと [GitHub Release](https://github.com/HiroyukiFuruno/katana-markdown-linter/releases/tag/v0.20.3) の成果物30件、チェックサム14組を照合した。
- crates.io、npm、PyPI、MCP Registry、Homebrew の 0.20.3 を確認した。配布 CLI と MCP の起動、Homebrew 定義2件、MCP バンドルのハッシュを検証した。詳細は `issues/139/release-delivery.json` に記録した。
