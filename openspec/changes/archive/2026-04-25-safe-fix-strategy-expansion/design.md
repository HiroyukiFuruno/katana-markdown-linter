## Design

### Rule Responsibility

`src/rules/markdown/rules/mdxxx.rs` は以下のみを責務とする。

- violation detection
- local replacement candidate
- candidate が安全でない場合は fix を返さない

以下は rule ファイルの責務にしない。

- rule enable/disable policy
- rule 実行順序
- 複数 fix の conflict resolution
- file-wide/table-wide/list-wide formatting strategy

### Fix Strategy

fix engine は diagnostics から candidate を集め、strategy ごとに適用可否を判断する。

- independent line replacement: 既存通り range overlap がなければ適用
- list strategy: list block を単位にして `MD005` / `MD007` / `MD030` の競合を判断
- table strategy: table block を単位にして `MD055` / `MD056` / `MD058` / `MD060` の競合を判断

### Safety Rules

- 既存 content の semantic structure を推測で変えない
- overlapping range は自動解決しない。strategy が明示された block のみ統合する
- formatting style が config から一意に決まらない場合は fix しない
- before/after fixture がない rule は fixable として数えない
- `safe` か `unsafe` かは完全な真偽判定ではなく、kml が default mode で保証する fix policy として扱う
- この change の実装対象は default mode の safe fix のみであり、unsafe fix の CLI 適用は扱わない

### Target Rules

`MD005`

- safe subset: list block 内の明確な sibling indentation inconsistency
- unsafe subset: nested list depth が ambiguous な場合

`MD030`

- safe subset: marker 後の spacing が config で一意な single-line list item
- unsafe subset: multi-line item と continuation indentation が絡む場合

`MD060`

- `v0.4.0` scope: edge fixture と manual-required reason で可視化する
- out of scope: 公式 metadata が `fixable: false` のため safe fix は実装しない

## Non-Goals

- 全 Markdown table formatter の実装
- user content の意味推測
- unsafe fix を default で有効化すること
- `--unsafe` CLI option、`[Y/n]` confirmation、non-interactive unsafe 実行 policy の実装
- official metadata が `fixable: false` の rule を local policy だけで fixable に変更すること
