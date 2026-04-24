## Why

`katana-markdown-linter` の check / fix coverage は拡張されたが、現状は rule 実装、fixture matrix、config property runtime、公式 document examples の整合性が十分に固定されていない。
特に matrix が実装に追従していない場合、品質ゲートが「安全な状態」を保証できない。

## What Changes

- fixture matrix を現行実装と公式 markdownlint fixability に同期する
- rule runtime に config properties を渡し、設定依存 rule が default で推測修正しないようにする
- 公式 document の examples / parameters / fixability を check / fix / config fixtures に反映する
- heuristic rule の検出範囲と fix 範囲を安全側に狭める
- 複数 rule fix の競合、順序、反復適用を検証できる quality gate を強化する

## Impact

- check / fix の安全性が fixture と config によって説明できる
- unsafe fix が default で勝手に走るリスクを下げる
- matrix drift による「実装済みなのに未対応扱い」「未実装なのに対応扱い」を検出できる
- 公式互換に届いていない rule が明示される
