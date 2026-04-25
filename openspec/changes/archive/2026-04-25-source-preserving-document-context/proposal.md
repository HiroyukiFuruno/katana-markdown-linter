## Why

`mado` は速く安定して見えるが、その理由は AST 採用だけではない。check 中心の狭い責務、parse once、parallel walk、静的な rule dispatch、release profile まで含めた設計で成り立っている。

`kml` は library-first で check と fix の両方を提供するため、AST を全面導入するだけでは source range、空白、改行、table alignment を安全に扱えない。fix 精度と性能を同時に上げるには、source-preserving な document context を一度構築し、rule がそれを共有する必要がある。

## What Changes

- `MarkdownDocument` / `DocumentContext` を追加し、line offsets、lines、code block ranges、headings、references、tables を一度だけ構築する
- AST は `DocumentContext` の optional/lazy component として部分導入する
- context-based rule API を追加し、既存 `evaluate(file_path, content)` は adapter で互換維持する
- heading / reference / code fence / table 系 rule の一部を context-based に移行する
- CLI directory processing と library API の責務境界を維持し、library API は caller が並列制御できる形を保つ
- benchmark と fixture で before/after と behavior parity を固定する

## Selected Direction

- `DocumentContext` を主軸にする。AST は補助情報として lazy に構築し、fix range の主根拠にはしない
- `mado` / `rumdl` は設計上の参考に留め、実装や behavior をコピーしない
- v0.5.0 の完了条件は full AST 化ではなく、source-preserving context の導入と代表 rule family の移行である
- unsafe fix mode、MCP productization、全 rule 一括 rewrite は別 change に分離する

## Impact

- rule ごとの hand-rolled scan を減らし、check/fix の判断材料を共有できる
- MD051/MD060 など構文依存 rule の精度改善がしやすくなる
- 将来の unsafe fix mode や workspace index の土台になる
- 旧 public API は互換維持されるため、既存 embedding user は壊れない

## Non-Goals

- `comrak` または `pulldown-cmark` への全面移行
- 全 rule の一括 rewrite
- unsafe fix mode の実装
- MCP tool productization
