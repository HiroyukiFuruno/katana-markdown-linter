## Why

`katana-markdown-linter` を public crate として運用するには、初回実装だけでなく upstream `markdownlint` の更新追従が必要になる。
この phase は、公式 rule document を構造的に解析し、repository 内の rule 実装と configuration が upstream から乖離していないかを機械的に検出するための保守基盤を作る。

## What Changes

- upstream `markdownlint` の rule catalog を取り込み、MD0XX の増減を検出する
- 廃止・deprecated 扱いの rule を可視化し、local との差分を報告する
- 各 rule の document md を構造的に解析し、description / tags / properties / fixability / default config の drift を検査する
- repository 内の rule 実装と設定 helper が upstream contract から外れていないかを check する

## Impact

- upstream 変更が入ったときに、どの rule を追従すべきかを機械的に判断できる
- local 実装の古さを「見えない debt」にせず、deprecated / missing / mismatched を明示できる
- phase2 以降の rule parity 監視が継続可能になる
