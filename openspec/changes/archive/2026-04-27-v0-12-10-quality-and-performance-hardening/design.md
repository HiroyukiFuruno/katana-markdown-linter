## Context

`v0.12.10`は「次の配布準備」ではなく、`v0.12.9`後の品質定着 patch として扱う。

現状データ:

- `src/cli.rs` と `src/upstream.rs` の行数が大きい
- 公開向け confidence は現状 1 ファイル corpus 依存が強い
- `make perf-check` の `api_lint_inline_code_heavy_document` は baseline より約 2.05x、`api_format_large_document` は 1.23x の増分が観測される

この状態で性能だけ追うと、精度・収束・分類の改善が遅れる。
`v0.12.10` は「精度優先」を最上位原則に置く。

## Goals / Non-Goals

**Goals:**

- `v0.12.10` の最重要目標を、外部品質と内部保守性の両輪に戻す
- 速度管理は、精度維持の前提で行い、回帰判断を規約化する
- `v0.12.10` 後に次版で着手すべき改善を明確化する

**Non-Goals:**

- 既存 rule セマンティクスを速度重視で変更すること
- 大規模な rewrite（新 parser 全体化など）
- 配布経路の拡大自体

## Decisions

- 3つの capability を独立して扱う:
  - `internal-quality-hardening`
  - `external-quality-hardening`
  - `performance-regression-control`
- 精度/収束の前提を守るため、性能改善は「計測・根拠・検証」を先に固めてから着手する
- 既存 CLI/API 契約（exit code、JSON構造、診断 ID の意味）を壊さずに、内部責務の再配置を行う
- `make` と既存 Python スクリプトを拡張し、将来的な手動運用を減らす（外部 human judgement に依存しすぎない）
- `v0.12.9` の quality score 運用を踏襲し、`v0.12.10` でも同一の採点フレーム（100点満点、threshold 90、`technical_hard_blockers` 判定）で release 判断材料を残す
- `v0.12.10` の成果物は tasks と evidence を残し、次版へ流す follow-up と分離する

## Quality Scoring

- `v0.12.10` では、`v0.12.9 public-confidence score` のカテゴリを継続採用する
- score は外部コーパスの信頼性・precision 回帰・収束性・性能安定性・再現性の5軸で machine-readable 形式で保存する
- 合否条件は `score >= 90` かつ `technical_hard_blockers = 0` とする

## Risks / Trade-offs

- 性能閾値を厳密化すると CI/PC 環境差でノイズが増える。対策として閾値は段階導入し、根拠付きの運用閾値を持たせる
- 公開 corpus の増加で fixture メンテナンスコストが上がる。対策として「分類 + 再現手順 + 収束確認」をセットで管理する
- 責務分離の初期実装で短期的な差分が増える。対策として API 互換の回帰テストを前提とする

## Migration Plan

- まず `internal-quality-hardening` の実装順を固定し、次に `external-quality-hardening`、最後に `performance-regression-control` の順で進める
- 性能施策は、内部/外部の回帰ガードが成立していることを確認してから `apply` を許可する
- 回帰/失敗時は、速度未最適化を許容しても精度が維持されている状態をまず保つ

## Open Questions

- 性能回帰を `perf-check` で fail まで上げる閾値は、まず `--strict` オプション（任意）として導入すべきか、まずは evidence-only で開始するか
