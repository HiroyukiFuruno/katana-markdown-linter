## Why

`v0.12.11` で内部・外部の品質定着、およびパフォーマンス回帰の監視ルール（`make perf-check-strict` や `public confidence` 運用）が整備され、品質ゲートとして機能することが証明されました。
しかし、`src/cli.rs` や `src/upstream.rs` といったコアモジュールに対する抜本的な責務分離（リファクタリング）や、性能改善の余地（無駄なアロケーションやイテレーションの最適化）は手付かずのまま残されています。

`v0.12.12` では、`v0.12.11` で確立した品質・回帰ゲート（`strict-perf-check`, `public-confidence` 等）を防御壁として活用しつつ、以下の根本的な品質・性能向上に取り組みます。

1. `src/cli.rs` などの肥大化モジュールに対する責務分離（内部品質向上）
2. 不要なヒープアロケーションの削減やクリティカルパスの最適化（パフォーマンス向上）
3. 外部品質の更なる担保と `public-confidence` 運用の洗練

## What Changes

- 内部品質: `v0.12.11` で先送りされた `src/cli.rs` 等の責務分離（入出力、バリデーション、実行パス）を実施。さらに `target/internal-quality-report.json` の `split_candidates` に基づくコード改善。
- パフォーマンス向上: `String::clone()` や不要な `Vec` のアロケーションなど、ホットパスでのパフォーマンスボトルネックの解消。並びに `make perf-refresh-baseline` の正式な運用。
- 外部品質: 既存の `public-confidence` corpus における `md-broken-link` などの known limitations に対する扱い・整理、または検証用 corpus の拡張と洗練。
- `v0.12.11` と同様に、`v0.12.12` でも `public confidence score`（100点満点）を継続適用し、リリース可否の絶対的基準とする。

## Capabilities

### New Capabilities

- なし（既存機能の内部構造および性能の抜本的改善）

### Modified Capabilities

- `internal-quality-hardening`: `src/cli.rs` や `src/upstream.rs` などの責務分離によるモジュール性向上
- `performance-regression-control`: ホットパスの最適化（メモリアロケーション削減等）によるベースライン性能の底上げ
- `external-quality-hardening`: `public-confidence` 運用ルールの継続強化

## Impact

- `src/cli.rs`, `src/lib.rs`, `src/upstream.rs`, `src/rules/markdown/` の広範囲にわたるリファクタリング
- パフォーマンス計測値の向上（ベースラインの更新）
- 既存の CLI 契約テスト（`cli_convergence_contract` 等）には影響を与えず、すべて Pass させること

## Non-Goals

- ユーザー向け新機能の追加
- 仕様の破壊的変更
- Linterの検査精度の意図的な低下を伴うパフォーマンス最適化