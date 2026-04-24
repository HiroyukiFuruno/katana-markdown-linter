## Why

現在の rule parity は公式 document 由来の fixture と drift check を中心にしているが、実際の upstream `markdownlint` 実装との診断差分を継続的に比較する仕組みはまだない。
本家実装を oracle とする golden comparison を追加することで、rule の挙動差分をより早く、具体的に検出できる。

## What Changes

- upstream `markdownlint` と `kml` を同じ fixture corpus に対して実行する comparison harness を追加する
- diagnostics を正規化し、rule id、line、column、range、fix result、known delta を比較できるようにする
- unknown delta を失敗として扱う gate を追加する
- rule ごとの check / fix / config / edge / golden status を dashboard として出力する
- network や upstream default branch の揺れを通常 test と分離し、再現性のある gate と明示的な update gate を分ける

## Capabilities

### New Capabilities

- `upstream-golden-comparison`: upstream markdownlint と kml の診断・fix 差分を比較する仕組みを定義する

### Modified Capabilities

- `rule-coverage`: rule coverage dashboard と golden comparison status を追加する

## Impact

- `tests/`
- `scripts/`
- `docs/`
- `Makefile`
- CI quality gate
- upstream markdownlint 実行環境
- known delta 管理ファイル
