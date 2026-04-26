## Why

`v0.12.9`で public confidence と speed 計測を更新しましたが、次の patch では
「内部の継続的改善」「外部品質の運用」「性能回帰の再現性」を同時に締める必要があります。

現状の観点:

- `src/cli.rs` が 2,399 行、`src/upstream.rs` が 1,196 行で、責務の分離が進みにくい
- `public-confidence` は現在1ファイル (`representative.md`) のみで、外部 corpus の説明力が限定的
- `make perf-check` は速度退行を報告するが、今回の `v0.12.10` 方針では「精度優先」を崩さない形で回帰管理を強化したい
- `v0.12.9` で `public confidence score` を公開し release 判断に使った実績がある

## What Changes

- 内部品質: CLI/設定/実行パスの責務分離と検証観点を明文化し、将来の保守性を上げる
- 外部品質: `public-confidence` を1ファイル運用から運用可能な最小 corpus + 分類運用へ拡張する
- 性能: 現行 `perf-check` に、再現性と回帰判断のルールを追加し、速度は「精度が守られている前提」で管理する
- `v0.12.9` と同様に、`v0.12.10` でも `public confidence score`（100点満点）を算出する。カテゴリ・閾値・`hard blocker` を明示し、公開用の品質判断材料として残す
- 3本柱を分離した tasks と DoD を追加し、`v0.12.10` で完了条件を明示する

## Capabilities

### New Capabilities

- `internal-quality-hardening`: 内部品質の担保（責務分離・回帰観点・運用可能な内部品質証跡）を追加する
- `external-quality-hardening`: 外部品質（実文書に近い証跡、finding 分類、収束証跡）を追加する
- `performance-regression-control`: 性能退行を検知・記録する制御ルールを追加する

### Modified Capabilities

- なし

## Impact

- 主に `src/cli.rs`、`scripts/ci/public-confidence*.py`、`scripts/ci/perf-check.py`、`Makefile`
- `tests/fixtures/public-confidence/**` と `tests/fixtures/perf-baseline.json` の運用方式変更
- `src/rules/markdown` / `tests/fixtures/*` のリファクタリング対象が明示的に増える

## Non-Goals

- ユーザー向け機能追加
- 既存挙動を速度優先で変更する最適化（精度を犠牲にする変更）
- MCP Registry / distribution / remote transport への追加作業
