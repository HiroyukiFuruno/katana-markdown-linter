## Why

`MD056` (table-column-count) は列数不一致を診断するが `fix_info: None` 固定で自動修正されない。
v0.12.17 時点で残り fix=0 ルールを精査した結果、Safe-fix が安全に実装可能なのは MD056 と MD003 のみと判明。
このパッチで MD056 の列数不足行（短い行）に空セル補完の safe-fix を追加し、0.12.x 終端ロードマップを進める。

過多行（cells.len() > expected_columns）はデータ消失リスクのため fix 対象外とする。

## What Changes

- `MD056` evaluate_context に `safe_pad_fix()` ヘルパーを追加し、列数不足行に対して fix_info を生成する
- 過多行 / `safe_to_fix=false` 行は引き続き診断のみ
- `is_safe_fix_rule` allowlist に `MD056` を追加
- fixture matrix（json + md）の MD056 行に fix エントリを追加
- README.md rule map の MD056 行を更新
- ユニットテストを追加（短い行の修正・過多行のスキップ・パイプスタイル保持）

## Capabilities

### New Capabilities

（なし）

### Modified Capabilities

- `rule-coverage`: MD056 が safe-fix を提供することを既存の link-family / table-family 要件に追記する

## Impact

- `src/rules/markdown/rules/md056.rs`（safe_pad_fix 追加・fix_info 生成・テスト）
- `src/lib.rs`（is_safe_fix_rule allowlist）
- `tests/fixtures/rule-fixture-matrix.json` / `.md`（MD056 fix エントリ）
- `README.md`（rule map 表）
- `CHANGELOG.md`、`Cargo.toml`、`Cargo.lock`、`openspec/changes/active-roadmap.md`
