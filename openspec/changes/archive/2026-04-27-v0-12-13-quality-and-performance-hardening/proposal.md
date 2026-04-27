## Why

`v0.12.12` で `src/cli.rs` / `src/upstream.rs` の分割、MD003/MD046 の誤検知修正、`severity_map` の `&str` 化による文字列クローン削減、中間 `Vec<&str>` の排除を完了し、public-confidence-score 100/100 を達成しました。
しかし、`src/cli/workflow.rs`（size_score 1197、35 関数）が split_candidates のトップに残っており、35 の関数が check/fix/fmt/config の各コマンドフローを一枚岩で実装しています。
可読性・保守性・将来の機能追加コストの観点からこれを解体することが v0.12.13 の主目的です。

また、`src/rules/markdown/rules/md059.rs` の `normalize_link_text` 関数は `split_whitespace().collect::<Vec<_>>().join(" ")` という明確に回避可能なアロケーションを含んでいます。

`v0.12.13` では、`v0.12.12` で確立した品質・回帰ゲートを防御壁として維持しながら、以下に取り組みます。

1. `src/cli/workflow.rs` の責務別サブモジュール分割（内部品質）
2. ホットパスの残存アロケーション除去（パフォーマンス）
3. 全品質ゲートの再通過確認と public-confidence-score の再作成

## What Changes

- 内部品質: `src/cli/workflow.rs` を `check.rs` / `fmt.rs` / `config_cmd.rs` / `common.rs` の 4 サブモジュールに分割し、各 size_score を ≤300 に抑える。
- パフォーマンス向上: `md059.rs` の `collect::<Vec<_>>()` を除去し、中間 Vec アロケーションをなくす。
- 外部品質: `public-confidence` corpus における unclassified: 0 の継続確認、`make perf-check-strict` の再通過。
- `v0.12.12` と同様に `public confidence score`（100点満点）を継続適用し、リリース可否の絶対的基準とする。

## Capabilities

### New Capabilities

- なし（既存機能の内部構造および性能の抜本的改善）

### Modified Capabilities

- `internal-quality-hardening`: `src/cli/workflow.rs` の責務分離によるモジュール性向上
- `performance-regression-control`: ホットパスのアロケーション削減によるベースライン性能の継続改善

## Impact

- `src/cli/workflow.rs` が削除され `src/cli/workflow/` サブモジュール群に置き換わる
- `src/rules/markdown/rules/md059.rs` の内部実装変更（外部インターフェース変更なし）
- 既存の CLI 契約テスト（`cli_convergence_contract` 等）には影響を与えず、すべて Pass させること

## Non-Goals

- ユーザー向け新機能の追加
- 仕様の破壊的変更
- Linter の検査精度の意図的な低下を伴うパフォーマンス最適化
