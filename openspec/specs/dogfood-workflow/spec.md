## Purpose

Repository-local dogfooding defines how `kml` is used against this repository's Markdown documents.

## Requirements

### Requirement: repository SHALL run kml against its Markdown documents

repository は、`kml` を用いて自身の Markdown 文書を check する dogfood workflow を提供しなければならない（SHALL）。

#### Scenario: dogfood check を実行する

- **WHEN** developer が dogfood check command を実行する
- **THEN** system は repository 内の対象 Markdown を再帰的に lint する
- **THEN** system は違反があれば非 zero exit code を返す
- **THEN** system は実行対象と除外対象を developer が確認できる形で定義する

### Requirement: dogfood workflow SHALL separate check from fix

dogfood workflow は、check-only と自動 fix を明確に分離しなければならない（SHALL）。

#### Scenario: safe fix を明示実行する

- **WHEN** developer が dogfood fix command を実行する
- **THEN** system は fixable rule のみを適用する
- **THEN** system は fix 後に残存違反を確認できる状態にする
- **THEN** system は check-only command で文書を書き換えない

### Requirement: dogfood workflow SHALL preserve archived OpenSpec documents by default

dogfood workflow は、履歴として保存された archived OpenSpec documents を既定の自動修正対象から除外しなければならない（SHALL）。

#### Scenario: archive を既定除外する

- **WHEN** developer が既定の dogfood command を実行する
- **THEN** system は `openspec/changes/archive/**` を自動修正対象から除外する
- **THEN** system は archive を確認したい場合の明示 command または option を提供する

### Requirement: dogfood workflow SHALL record actionable CLI usability findings

dogfood workflow は、実利用で見つかった CLI の課題を再現可能な findings として記録しなければならない（SHALL）。

#### Scenario: usability finding を記録する

- **WHEN** dogfood 実行で CLI の不足や違和感が見つかる
- **THEN** system は command、期待結果、実際の結果、対応判断を記録する
- **THEN** system は未対応 findings を次の OpenSpec change に移せる粒度にする

### Requirement: dogfood workflow SHALL exercise core command separation before release

dogfood workflow は、release 前に `check`、`fix`、`fmt` の責務分離を確認しなければならない（SHALL）。

#### Scenario: core command dogfood を実行する

- **WHEN** developer が release 前 dogfood を実行する
- **THEN** system は check-only path が対象 Markdown を書き換えないことを確認する
- **THEN** system は fix path が default-safe fix だけを適用することを確認する
- **THEN** system は fmt path が formatter policy の範囲だけを変更することを確認する
- **THEN** system は見つかった CLI usability findings を後続対応できる粒度で記録する

### Requirement: public confidence dogfood SHALL support representative external Markdown corpus

公開前 confidence dogfood は、repository 内 fixture だけでなく、実運用に近い外部 Markdown corpus を扱えなければならない（SHALL）。

#### Scenario: KatanA Markdown corpus を確認する

- **WHEN** developer が `KATANA_CHECKOUT` を指定して external dogfood を実行する
- **THEN** system は KatanA `docs/**/*.md` と `assets/**/*.md` を候補 corpus として扱う
- **THEN** system は binary asset を lint 対象に含めない
- **THEN** system は `check` 実行で source file を書き換えない
- **THEN** system は finding を source path、rule、分類とともに記録する

### Requirement: public confidence dogfood SHALL avoid private corpus coupling in required CI

公開前 confidence dogfood は、required CI を private checkout に依存させてはならない（SHALL NOT）。

#### Scenario: required CI を実行する

- **WHEN** GitHub Actions required CI が実行される
- **THEN** system は sibling KatanA checkout を必須にしない
- **THEN** system は public repository に置ける curated fixture または synthetic equivalent で再現性を確保する
- **THEN** system は external dogfood が未実行の場合、その理由を release evidence に記録する

### Requirement: public confidence corpus SHALL be expanded to mixed-structure representative coverage

public confidence は、外部文書に近い構造を含む最小代表 corpus で継続評価されなければならない（SHALL）。

#### Scenario: representative corpus を管理する

- **WHEN** `v0.12.11` の public confidence を実行する
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

- **WHEN** `v0.12.11` の public confidence を完了する
- **THEN** 系統的な evidence（syntax カウント、timing、分類、release blocker）を `target/public-confidence-report.json` に保存する
- **THEN** release 前の `release-check` がこの evidence を参照して判断できる
- **THEN** follow-up が必要な finding は未分類で放置しない
