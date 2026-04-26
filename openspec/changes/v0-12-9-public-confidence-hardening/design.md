# Design

## Positioning

`v0.12.9` は安定化 patch として扱う。

`v0.12.8` の stable score は有効な結果だが、公開導線の拡大は別の判断である。
そのため、`v0.12.9` では「内部 fixture で通る」から「実運用に近い文書でも説明できる」へ evidence を広げる。

## Corpus Strategy

corpus は 3 層に分ける。

| Layer | Source | Purpose | Required CI |
| --- | --- | --- | --- |
| internal regression corpus | `tests/fixtures/**` | 最小再現、rule fixture、golden comparison | yes |
| performance corpus | `tests/fixtures/perf-baseline.json` と benchmark generator | 速度と hot path の継続比較 | yes |
| external confidence corpus | KatanA `docs/**/*.md`、`assets/**/*.md` | 実文書 dogfood、公開前の信頼性確認 | optional または curated subset |

KatanA `assets/` の binary asset は lint 対象ではない。
ただし `assets/**/*.md` は Markdown corpus として扱い、画像やリンクの記法が多い文書として parser / rule precision の確認に使う。

## KatanA Corpus Policy

KatanA corpus は次の原則で扱う。

- sibling checkout は `KATANA_CHECKOUT=/Users/hiroyuki_furuno/works/private/katana` のように明示する
- required CI は sibling checkout に依存しない
- public repository に取り込める文書だけを curated fixture にする
- private / product-specific な全文を無条件にコピーしない
- finding は元文書 path、rule、再現条件、分類を tasks に残す

## Finding Classification

external corpus で見つかった差分は、次のどれかに分類する。

| Category | Meaning | Action |
| --- | --- | --- |
| true-positive | 正しい診断 | baseline または report に記録 |
| false-positive | 誤検知 | rule-local test と document-level test を追加して修正 |
| false-negative | 検出漏れ | 最小再現 test と corpus fixture を追加して修正 |
| unsafe-fix-risk | fix 事故の可能性 | default-safe fix から除外または collision guard を追加 |
| fmt-policy-gap | formatter policy の不足 | `fmt` 仕様として扱うか対象外に分類 |
| perf-regression | 実文書で説明不能な速度劣化 | hot path を調査し、必要なら blocker |

## Evaluation Axes

KatanA corpus を `check` / `fix` / `fmt` に通した後は、診断数そのものではなく次を評価する。

| Axis | `check` で見ること | `fix` で見ること | `fmt` で見ること |
| --- | --- | --- | --- |
| correctness | diagnostic が true-positive か、false-positive / false-negative がないか | default-safe fix だけが適用され、意味を壊していないか | formatter policy の範囲だけを変更しているか |
| preservation | source file を一切書き換えないか | link、image、inline HTML、code block、table、reference definition を壊さないか | Markdown の意味、link target、code block 内容を壊さないか |
| convergence | repeated `check` で結果が安定するか | repeated `fix` で追加差分が出ないか | repeated `fmt` で追加差分が出ないか |
| explainability | finding が分類済みで、未分類 high-risk が残らないか | applied / skipped fix の理由を説明できるか | 変更した行と変更しない行の境界を説明できるか |
| performance | file count、document size、elapsed time を記録できるか | mutable copy 上で実測し、説明不能 regression がないか | formatter 実行時間と差分量が説明できるか |

評価の中心は、KatanA 文書をきれいにすることではない。
`kml` が実文書に対して安全に使えることを確認することである。

release-blocking とする条件:

- 未分類の false-positive または false-negative が残る
- `check` が source file を書き換える
- `fix` が default-safe ではない変更を行う
- `fix` または `fmt` の再実行で追加差分が出る
- link、image、inline HTML、code block、table、reference definition を壊す
- 実文書 corpus で説明不能な重大 performance regression がある

## Release Gate

`v0.12.9` release 前に次を満たす。

- KatanA external dogfood または curated KatanA-derived fixture のどちらかで confidence evidence が残っている
- external corpus finding が分類済みで、release-blocking issue が 0 件である
- `check` は external corpus を書き換えない
- `fix` と `fmt` は再実行で不要差分を増やさない
- performance corpus と external corpus の速度結果に説明不能な重大 regression がない
- `v0.13.0` へ進むかどうかのユーザー判断材料が tasks に残っている
