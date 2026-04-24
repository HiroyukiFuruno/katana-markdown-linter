# Tasks

## DoR

- [x] `dogfood-cli-and-api-usage` の DoR 状態を確認し、この change が CLI UX issue 修正を主目的にしないことを確認する
- [x] 既存の rule fixture matrix と upstream drift check の位置づけを確認する
- [x] upstream oracle の実行方法を決めるため、固定 version と default branch 追従の使い分けを確認する
- [x] known delta を許容する判断基準を design に照らして確認する

## Implementation

- [x] upstream markdownlint を実行する oracle command を定義する
- [x] `kml` diagnostics を normalized diagnostic schema に変換する
- [x] upstream diagnostics を normalized diagnostic schema に変換する
- [x] fixture corpus を golden comparison の入力として再利用できるようにする
- [x] diagnostics comparison harness を追加する
- [x] fix output comparison harness を追加する
- [x] known delta file schema を追加する
- [x] unknown delta を failure にする gate を追加する
- [x] live upstream update target と deterministic golden target を分離する
- [x] rule coverage dashboard generator を追加する
- [x] dashboard を docs 配下に出力する
- [x] CI または release-check に入れる対象を明示する

## DoD

- [x] deterministic golden comparison が network access なしで実行できる
- [x] live upstream update check が明示 target として実行できる
- [x] unknown delta が failure になる
- [x] known delta は reason と解消条件なしでは登録できない
- [x] dashboard に check / fix / config / edge / golden / known delta 状態が表示される
- [x] failure report に rule ID、fixture、expected、actual が含まれる
- [x] `openspec status --change upstream-compatibility-golden-gate --json` で apply-ready である
