## Why

`v0.12.2` で公開（release）前の品質ゲートと Windows CI は締まった。
次のパッチ（patch）は機能追加より、lint 精度、実行速度、テスト層の厚みを上げる。

いま残っているリスクは、単一行判定に寄った rule の誤検知、構文が混ざった文書での検出漏れ、速度改善の根拠不足、単体テスト（UT）と結合テスト（IT）の偏りである。

`v0.12.3` はこれらを同時に扱うが、範囲はパッチ公開（patch release）に収まる内部品質改善に限定する。

## What Changes

- 誤検知と検出漏れを課題棚卸し（issue inventory）で分類し、修正対象をルール単位テスト（rule-local test）と文書単位の固定テスト（document-level fixture）の両方で固定する。
- `DocumentContext` とルール単位（rule-local）判定の境界を見直し、インラインコード（inline code）、HTML、コードフェンス（fence）、table、reference 周辺の文脈（context）判定を強化する。
- 速度改善は `make bench` / `make perf-check` の基準値（baseline）に基づき、測定できる高負荷経路（hot path）だけを対象にする。
- 単体テスト（UT）はルール（rule）、設定（config）、パス（path）、修正の冪等性（fix idempotence）、Windows path 表現を中心に拡充する。
- 結合テスト（IT）は CLI 作業領域（CLI workspace）、検証用文書群（fixture corpus）、自己適用検査（dogfood）、複数ツール比較ベンチマーク（cross-tool benchmark）、Windows CI 実行差分を中心に拡充する。

## Scope

対象範囲:

- lint 精度（linter precision）の改善と回帰 fixture 追加。
- 測定済み速度（measured performance）の高負荷経路（hot path）改善。
- 単体テスト（UT）と結合テスト（IT）の不足棚卸しと追加。
- 公開前ゲート（release gate）に必要な AST lint または Makefile target の補強。
- `v0.12.3` 公開メタデータ（release metadata）、CHANGELOG、公開検証（release verification）。

対象外:

- 新しい rule family の大規模追加。
- unsafe fix policy の変更。
- 公開 API（public API）/ CLI 出力スキーマ（CLI output schema）の破壊的変更。
- MCP Registry / Hub 公開作業。
- 遠隔 MCP 接続（remote MCP transport）。
- benchmark 数値だけを良くするための rule semantics 変更。

## Impact

- 誤検知と検出漏れの再発条件が物理 fixture に残る。
- 速度改善は正しさの検査（correctness gate）と分離した上で、根拠つきで説明できる。
- 後続の MCP 配布（distribution）作業に入る前に、linter core の信頼性が上がる。
