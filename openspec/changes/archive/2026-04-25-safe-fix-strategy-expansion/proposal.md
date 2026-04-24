## Why

`v0.4.0` の主目的は check/fix 拡充である。
`v0.3.0` 時点で check coverage は広がったが、fixture matrix には safe fix 未対応として `MD005`, `MD030` が残っている。
これらは単一 rule 内で即時 replacement を返すだけでは安全性を保証しにくく、複数 rule の順序、重複 range、table/list の構造再整形を考慮する必要がある。
`MD060` は table edge coverage と manual-required 理由を明確にするが、公式 metadata が `fixable: false` のため `v0.4.0` の safe fix 対象からは外す。

対応 rule 数を増やすために unsafe fix を有効化すると、Markdown を壊すリスクが高い。
そのため fix strategy を明示し、安全に扱える subset から段階的に fixture で固定する。
unsafe fix mode、`--unsafe`、interactive confirmation は `v0.5.0` 向けの `unsafe-fix-mode-and-confirmation` に分離し、この change には含めない。

## What Changes

- rule の `check/fix` 責務を維持しつつ、fix orchestration 側で strategy-aware な順序制御を追加する
- `MD005` / `MD030` の list indentation / marker spacing fix を安全な subset から実装する
- `MD060` は edge fixture と manual-required reason で可視化し、safe fix 実装は行わない
- unsafe なケースは diagnostic のみ、または `manual_required` に明確な理由を残す
- fix before/after fixtures と cross-tool validation を増やす
- unsafe fix candidate は将来の `--unsafe` 対象候補として分類できるよう記録するが、CLI からは適用しない

## Impact

- fix 対応 rule 数が増える
- fix range conflict や order dependency が実装に埋もれず可視化される
- `mdxxx.rs` は check/fix の局所責務に留まり、複合制御は fix engine 側に集約される
- default の `kml fix` / `kml check --fix` は safe policy を維持する
