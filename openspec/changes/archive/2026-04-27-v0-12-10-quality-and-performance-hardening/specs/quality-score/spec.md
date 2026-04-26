## ADDED Requirements

### Requirement: システムは `v0.12.10` で品質スコアを算出する

`v0.12.10` の品質判定は、`v0.12.9` と同一方針で 100 点満点基準の数値として残さなければならない（SHALL）。

#### Scenario: `v0.12.10` の quality score を算出する

- **WHEN** `v0.12.10` の validation が完了する
- **THEN** system は `public-confidence-score.json` を出力する
- **THEN** score は `total` と `threshold` を含み、`status` / `version` / `technical_hard_blockers` を明示する
- **THEN** score は 100 点満点基準でカテゴリ別採点を保存すること

### Requirement: システムは `v0.12.9` と同一カテゴリ構成を維持する

採点カテゴリは下記 5 軸を維持し、`v0.12.9` 時点の評価フレームを引き継がなければならない（SHALL）。

- `External corpus confidence`
- `Precision regression`
- `Command convergence`
- `Performance stability`
- `Release reproducibility`

配点は `v0.12.9` と同じく 30 / 25 / 20 / 15 / 10、合計 100 点とする。

#### Scenario: カテゴリ採点を再現可能に残す

- **WHEN** quality score を計算する
- **THEN** 各カテゴリは `score` / `max` / `evidence` を持つ
- **THEN** evidence には `make public-confidence`、`make perf-check`、収束実行、`release-check` の結果を含める
- **THEN** `v0.12.9` の配点（30 / 25 / 20 / 15 / 10）を採用し、`v0.12.10` でもカテゴリ合計が 100 点になる

### Requirement: システムは release 進行条件として score を機械的に利用する

`score` と `technical_hard_blockers` は、release 前の進行条件として扱わなければならない（SHALL）。

#### Scenario: 合否判定を記録する

- **WHEN** `public-confidence-score.json` が更新される
- **THEN** `score.total >= 90` かつ `technical_hard_blockers = []` を満たすことを release 判断用タスクに反映する
- **THEN** 残存する `known_limitations`、`non_blocking_follow_ups`、`verification` を score レポートに残す
- **THEN** `release` 前提時点で未分類 high-risk finding を残さない
