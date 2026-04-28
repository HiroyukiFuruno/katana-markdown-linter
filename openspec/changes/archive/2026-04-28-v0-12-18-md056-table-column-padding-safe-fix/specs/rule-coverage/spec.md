## MODIFIED Requirements

### Requirement: link-family precision SHALL use shared parser evidence

link 系 rule の精度改善は、共有 parser または `DocumentContext` の token index に基づかなければならない（SHALL）。

#### Scenario: link 系 rule を評価する

- **WHEN** `MD034`、`MD051`、`MD052`、`MD054`、`MD059` が document を評価する
- **THEN** system は inline code、image、reference definition、autolink を通常本文と混同しない
- **THEN** system は rule ごとの独立した簡易 parser を増やさない
- **THEN** system は誤検知と検出漏れを rule-local test と document-level fixture の両方で固定する

#### Scenario: MD051 が emoji・CJK 混在見出しのフラグメントを正しく生成する

- **WHEN** 見出しテキストに emoji または CJK 文字が含まれる Markdown を評価する
- **THEN** system は GitHub と同じフラグメント生成ルール（小文字化・スペース→ハイフン・emoji 除去・Unicode 字母保持）を適用する
- **THEN** system は emoji のみで構成された見出しを空フラグメント（検出対象外）として扱う
- **THEN** system は CJK 文字を含む見出しに対して false positive を出さない
- **THEN** system は誤ったフラグメントを参照するリンクに対して false negative を出さない

#### Scenario: MD056 が列数不足のテーブル行を空セルで補完する

- **WHEN** テーブル行のセル数が header のセル数より少ない（`row.cells.len() < expected_columns`）Markdown を fix モードで処理する
- **THEN** system は不足分だけ空セルを補完し、行のパイプスタイル（leading/trailing）を保持する
- **THEN** system は補完後の行が header と同じ列数になるように修正する
- **THEN** system は `row.safe_to_fix=false`（escaped pipe や inline code を含む行）は fix 対象外として診断のみ返す
- **THEN** system は列数過多の行（`row.cells.len() > expected_columns`）に対しては fix を生成せず診断のみ返す（データ消失防止）
