# v0.19.0 Editor Publication Gate Design

## 対象

- release 管理と公開ガードの見直し（`v0.18.x` の継続運用含む）
- marketplace 公開を含める `v0.19.0` の実施条件の固定
- バージョン bump の実体性を担保する運用ルール

## 設計方針

1. バージョン policy
   - `v0.18.7` は事故版として扱い、同一版の再公開は試みない。
   - `v0.18.8` 以降の patch は、linter 機能の新規拡張ではなく、`check/fix/format` の不具合報告ベース修正を許容。
   - `v0.19.0` は、Marketplace 公開実装が release 対象に入る場合のみ。

2. release gate 固定
   - release 実行前に「既存チャネル既存 version」存在チェックを厳格化する。
   - `release-check` と `release-verify` の結果は、意図した publish/defer の状態を明示する。
   - marketplace publish は manual-ready。設定不足なら fail fast。

3. 運用の可観測性
   - 失敗理由は runbook で 3 行以内で再現可能な手順として記録。
   - いつ `v0.19.0` へ進むかは、ゲート結果の通過記録でのみ判断。
4. Self-dogfood を release-readiness に固定
   - `release-check` 前に `just dogfood` を必須実行し、`README.md docs openspec` を対象に自己検査する。
   - `target/dogfood-report.json` を evidence として残し、意図しない baseline 増加を release blocker とする。
   - 変更意図がある場合のみ `just dogfood-refresh-baseline` を使って baseline を更新し、更新理由を `release-readiness` に添付する。

## 受け入れ条件

- 0.18系が `v0.18.7` 再リリース不能条件を破壊せずに進められる。
- `v0.19.0` の release 要件が満たされた時だけ、v0.19 へバンプする。
- marketplace 公開は `published/deferred` のどちらでも release-verify で状態が説明される。
