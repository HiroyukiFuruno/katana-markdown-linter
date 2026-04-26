## ADDED Requirements

### Requirement: public confidence corpus SHALL be expanded to mixed-structure representative coverage

public confidence は、外部文書に近い構造を含む最小代表 corpus で継続評価されなければならない（SHALL）。

#### Scenario: representative corpus を管理する

- **WHEN** `v0.12.10` の public confidence を実行する
- **THEN** `tests/fixtures/public-confidence/corpus` は `representative.md` 以外を含める最小拡張を受ける
- **THEN** corpus は fenced code（backtick + tilda）、table、inline code、HTML、reference、link、画像、mixed 日本語/英語を最低1件ずつ含む
- **THEN** 追加ファイルは意図的な lint / fix / fmt 観測ポイントとして差分追跡可能であること

### Requirement: system SHALL classify public confidence diagnostics before release discussion

`make public-confidence` の結果は、release 前に診断分類を明示し、未分類結果を放置しない運用を守らなければならない（SHALL）。

#### Scenario: finding 分類を付与する

- **WHEN** public confidence の `check` が true-positive 以外を検出する
- **THEN** システムは `true-positive`、`false-positive`、`false-negative`、`unsafe-fix-risk`、`fmt-policy-gap` のいずれかを分類する
- **THEN** 分類結果は JSON report に保持される
- **THEN** 未分類が 1 件でも残る場合は release-blocking として扱う

### Requirement: system SHALL preserve check/fix/fmt convergence evidence after external corpus expansion

収束性は、corpus を増やしても維持されなければならない（SHALL）。

#### Scenario: corpus 拡張後に収束を確認する

- **WHEN** external corpus で `make public-confidence` を再実行する
- **THEN** `check` は `source_unchanged` を維持する
- **THEN** `fix` と `fmt` は 2回実行で `changed_files == 0` を満たす
- **THEN** `final_check` は収束後の状態で実行される

### Requirement: system SHALL maintain `public-confidence` evidence as release-ready metadata

外部品質 evidence は、後続版選定に使える形で保存されなければならない（SHALL）。

#### Scenario: evidence を記録して引き継ぐ

- **WHEN** `v0.12.10` の public confidence を完了する
- **THEN** 系統的な evidence（syntax カウント、timing、分類、release blocker）を `target/public-confidence-report.json` に保存する
- **THEN** release 前の `release-check` がこの evidence を参照して判断できる
- **THEN** follow-up が必要な finding は未分類で放置しない
