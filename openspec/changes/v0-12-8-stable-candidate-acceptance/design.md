# Design

## Stable Score

安定版スコアは 100 点満点とする。

| Area | Points | Evaluation |
| --- | ---: | --- |
| Precision correctness | 40 | 誤検知、検出漏れ、構文除外、golden / fixture consistency |
| Safe command behavior | 20 | `check` no-write、safe fix、fix/fmt idempotence、collision safety |
| Performance stability | 20 | parser / context migration 後の benchmark、hot path、baseline explanation |
| Release reproducibility | 10 | local / CI gate alignment、OS matrix、package dry-run、release-check |
| Evidence quality | 10 | tasks evidence、known limitation、dogfood findings、score report completeness |

## Stable Decision

判定は次の通り。

| Score | Meaning |
| ---: | --- |
| 90-100 | Stable candidate。hard blocker がなければユーザー受け入れ判断へ進む |
| 80-89 | Release candidate。追加の `v0.12.x` hardening を検討する |
| 70-79 | Hardening required。安定版とは呼ばない |
| 0-69 | Not stable。設計または migration 方針を見直す |

stable として扱えるのは、`90+`、hard blocker なし、ユーザー受け入れあり、の 3 条件を満たす場合だけ。

## Hard Blockers

点数に関係なく、次が 1 つでもあれば安定版候補にしない。

- 未分類の高優先度誤検知または検出漏れが残っている
- `check` が入力を書き換える
- default-safe fix に unsafe fix が混ざる
- `fix` または `fmt` が再実行で不要差分を増やす
- parser / AST 化で説明不能な重大 performance regression がある
- `make release-check` または required CI が失敗している
- dogfood で release-blocking finding が残っている
- score report の根拠が tasks に残っていない
- ユーザーが安定版として受け入れていない

## Score Evidence

score は tasks に記録する。
可能なら machine-readable report も生成できる形にする。

最低限の記録項目:

- score total
- category score
- hard blocker list
- verification command results
- accepted limitations
- user acceptance decision

## User Acceptance

`v0.12.8` の最終工程はユーザー受け入れ判断とする。

agent は score、hard blocker、known limitation、次に進む影響を提示する。
ユーザーは次のいずれかを選ぶ。

- 安定版として受け入れる
- 追加の `v0.12.x` hardening を要求する
- `v0.13.0` への移行を保留する

ユーザー受け入れがない場合、`v0.13.0` の DoR は満たさない。
