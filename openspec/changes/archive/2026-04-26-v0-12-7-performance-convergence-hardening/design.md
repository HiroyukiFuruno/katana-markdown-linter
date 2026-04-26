# Design

## Stability Focus

`v0.12.7` は新しい精度 migration を広げる回ではない。
`v0.12.5` と `v0.12.6` で増えた parser / context 化の実測と収束性を固める。

## Performance Model

性能評価は次の種類に分ける。

| Area | Purpose |
| --- | --- |
| parser index construction | AST / parser readiness の固定 cost を見る |
| API lint | rule migration 後の純粋な lint cost を見る |
| API fix | fix candidate と再診断の cost を見る |
| CLI check/fix/fmt | 実利用 path の cost を見る |
| dogfood | この repository の実文書で実用速度を見る |

CI は wall-clock fluctuation だけで止めない。
ただし unexplained regression は release evidence として残し、必要なら blocker にする。

## Convergence Model

収束性は次の順番で確認する。

~~~text
check
  -> check --fix
  -> fix
  -> fmt
  -> check
  -> fmt again
~~~

期待する状態:

- check-only は書き換えない
- safe fix は unsafe fix を混ぜない
- fix 再実行で同じ変更を繰り返さない
- fmt は lint fix を代替しない
- fmt 再実行で差分が出ない

## Baseline Policy

baseline refresh は、正しさの gate が通った後だけ行う。
refresh する場合は、対象 case、before / after、理由を tasks に記録する。

## Stable Score Preparation

`v0.12.8` の score に使う evidence を先に揃える。

- precision fixture result
- fix / fmt convergence result
- performance benchmark summary
- dogfood result
- release-check result
