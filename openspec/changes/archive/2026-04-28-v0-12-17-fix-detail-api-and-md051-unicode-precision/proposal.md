## Why

`fix()` / `fix_with_results()` の呼び出し元がどのルールの何行目が修正されたかを知る手段がなく、
CLIログや IDE 拡張でのハイライトが作れない（Issue #43）。
また MD051 のフラグメント正規化が GitHub の実際の anchor 生成ルール（emoji 除去・Unicode 保持）と乖離しており、
CJK や emoji を含む見出しで false negative / false positive が発生している。

## What Changes

- `FixDetail { rule_id: String, range: FixRange, applied: bool }` を新規公開型として追加する
- `FixResult` に `details: Vec<FixDetail>` フィールドを追加する（既存フィールドはそのまま保持）
- `fix_with_results` / `fix_with_results_including_unsafe` の内部実装が accepted/skipped 情報を `FixDetail` として詰める
- `src/rules/markdown/rules/md051/fragments.rs` のフラグメント正規化を GitHub の実際の変換ルールに合わせる
  - 小文字化
  - スペース → ハイフン
  - ASCII 英数字・ハイフン・Unicode 字母以外を除去（emoji も除去）
  - 重複ハイフンを正規化

## Capabilities

### New Capabilities

- `fix-detail-api`: `FixResult` に適用済み/スキップ済み修正の詳細情報を付与する公開 API

### Modified Capabilities

- `rule-coverage`: MD051 のフラグメント正規化要件が変わるため、既存の rule-coverage spec の MD051 行動定義を更新する

## Impact

- `src/types.rs` または `src/lib.rs`（公開型 `FixDetail`・`FixRange` 追加）
- `src/fix/mod.rs`（`apply` 関数が FixDetail を収集して返す）
- `src/rules/markdown/rules/md051/fragments.rs`（正規化ロジック変更）
- `tests/ast_linter.rs`・unit tests（regression + 新テスト追加）
- `CHANGELOG.md`、`Cargo.toml`、`Cargo.lock`
