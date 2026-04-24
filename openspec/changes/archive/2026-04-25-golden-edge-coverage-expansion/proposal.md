## Why

`docs/rule-coverage-dashboard.md` では、多くの rule の Golden coverage が `pending` であり、Edge coverage も 0 の rule が多い。
公式 document examples と local fixture matrix は存在するが、upstream markdownlint との deterministic golden comparison が全 active rule に対して十分に固定されていない。

check/fix 拡充を安全に進めるには、実装差分が「意図した差分」か「互換性 regression」かを rule ごとに判定できる必要がある。

## What Changes

- active rule ごとの golden check/fix coverage を dashboard で可視化する
- `pending` の rule に対して upstream golden corpus を追加する
- known delta は reason / resolution / owner を持つ明示的な allowlist にする
- edge cases を rule group ごとに追加し、coverage dashboard に反映する
- 公式 document examples の取り込み漏れを AST lint / fixture harness で検出する

## Impact

- rule implementation の互換性を upstream 実行結果と比較しやすくなる
- future rule change 時に regression と known delta を切り分けられる
- check/fix 拡張のレビュー単位が rule ごとに明確になる
