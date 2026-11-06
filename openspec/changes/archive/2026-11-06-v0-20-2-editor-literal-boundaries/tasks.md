## 1. Implementation

- [x] 1.1 LSP の座標変換と範囲整形の共有境界を実装する。
- [x] 1.2 HTML とコードの保護範囲を共有し、必要時に構文解析する。
- [x] 1.3 CLI、LSP、MCP の設定準備を共有する。
- [x] 1.4 正解データと実プロトコル回帰を追加する。
- [x] 1.5 依存を確認し、互換更新と配布メタデータを同期する。
- [x] 1.6 脚注と定義リストに続くリテラルを共有候補判定へ通す。

## 2. Verification

- [x] 2.1 完全なリリース前検証と既存の厳格な性能検査を通す。
- [x] 2.2 Draft レビュー、指摘の修正・返信・解決、必須 CI を確認する。
- [x] 2.3 マージ起動の公開と公開済み成果物を検証する。

## Delivery ownership

公開後の仕様同期、アーカイブ、Issue クローズ、作業ブランチの整理は
Issue #138 とセッション台帳で追跡する。

## 品質評価スコア

| 項目 | 最大 | 現在 |
| --- | --- | --- |
| 実装と回帰 | 40 | 40 |
| リリース前検証 | 30 | 30 |
| レビュー、CI と公開 | 30 | 30 |
| 合計 | 100 | 100 |

## Evidence

- [実装 PR #140](https://github.com/HiroyukiFuruno/katana-markdown-linter/pull/140) の署名を確認し、指摘1件の修正・返信・解決後にマージした。
- [必須 CI](https://github.com/HiroyukiFuruno/katana-markdown-linter/actions/runs/42159058123) と [リリース前 CI](https://github.com/HiroyukiFuruno/katana-markdown-linter/actions/runs/42159058106) は成功した。
- [公開実行](https://github.com/HiroyukiFuruno/katana-markdown-linter/actions/runs/42159620897) は Homebrew 用の認証更新後の再実行で全8ジョブが成功した。
- 署名付きタグ、公開成果物30件、チェックサム14組、各レジストリ、Homebrew および配布バイナリの起動を確認した。
- 検証結果の詳細は `issues/138/release-preflight.json` と `issues/138/release-delivery.json` を参照する。
