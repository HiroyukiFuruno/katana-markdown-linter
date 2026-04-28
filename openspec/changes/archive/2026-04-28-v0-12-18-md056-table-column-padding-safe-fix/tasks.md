## 0. MD029 バグ修正（nested unordered list で親番号がリセットされる問題）

- [x] 0.1 `src/rules/markdown/rules/list.rs:128` の `expected_numbers.clear()` を以下に変更する：

  ~~~rust
  let indent = line.len() - trimmed.len();
  expected_numbers.retain(|level, _| *level < indent);
  ~~~

- [x] 0.2 ユニットテスト `md029_no_false_positive_when_nested_unordered_list_interrupts_ordered` を追加する

  - 入力: `"1. item1\n2. item2\n3. item3\n   - sub bullet\n4. item4\n5. item5\n"`
  - 期待: 診断なし（`diagnostics.is_empty()`）

- [x] 0.3 ユニットテスト `md029_fix_is_correct_after_nested_unordered_list` を追加する

  - 入力: `"1. item1\n2. item2\n   - sub\n4. item3\n"`
  - 期待: 診断 1 件（行 4 のみ）、`fix.replacement == "3"`
- [x] 0.4 `cargo test -p katana-markdown-linter --lib list` で MD029 テストが全通過することを確認する

## 1. MD056 safe-fix 実装

- [x] 1.1 `src/rules/markdown/rules/md056.rs` のインポートに `DiagnosticFix` と `TableRow` を追加する
- [x] 1.2 `safe_pad_fix(row: &TableRow<'_>, expected_columns: usize) -> Option<String>` ヘルパー関数を実装する
  - `row.safe_to_fix=false` の場合は `None`
  - `row.cells.len() >= expected_columns` の場合は `None`（過多行・一致行は fix しない）
  - 不足分だけ空セル `""` を追加し、`(leading_pipe, trailing_pipe)` の 4 パターンで join した文字列を返す
- [x] 1.3 `evaluate_context` の `fix_info: None` を `safe_pad_fix(row, expected_columns).map(|replacement| DiagnosticFix { ... })` に置き換える
- [x] 1.4 `cargo check --workspace` でコンパイル通過を確認する

## 2. is_safe_fix_rule allowlist 更新

- [x] 2.1 `src/lib.rs:285-328` の `is_safe_fix_rule` の `matches!` パターンに `"MD056"` を追加する（MD055 と MD058 の間、アルファベット順）

## 3. ユニットテスト追加

`src/rules/markdown/rules/md056.rs` の `mod tests` に以下を追加する。

- [x] 3.1 `fix_pads_short_row_with_empty_cells`: `"| a | b |\n|---|---|\n| 1 |\n"` → 行 3 に `fix_info` が含まれ、`replacement` が `"| 1 |  |"` であることを検証
- [x] 3.2 `fix_skips_overflow_row_to_avoid_data_loss`: `"| a | b |\n|---|---|\n| 1 | 2 | 3 |\n"` → 行 3 に診断が出るが `fix_info.is_none()` であることを検証
- [x] 3.3 `fix_preserves_pipe_style_no_leading_or_trailing`: `"a | b\n---|---\n1\n"` → 行 3 の `replacement` が `"1 | "`（pipe なしスタイル維持）であることを検証
- [x] 3.4 `fix_skips_unsafe_row_with_escaped_pipe`: 行に `\|` が含まれる場合 `fix_info.is_none()` を検証

## 4. fixture matrix 更新

- [x] 4.1 `tests/fixtures/rule-fixture-matrix.json` の MD056 エントリの `fix` 配列に 1 件追加する：

  ~~~json
  {
    "name": "pad_short_table_row_with_empty_cells",
    "source": "| a | b |\n|---|---|\n| 1 |\n",
    "expected": "| a | b |\n| --- | --- |\n| 1 |  |\n"
  }
  ~~~

- [x] 4.2 `tests/fixtures/rule-fixture-matrix.md` の MD056 行の Fix 列を `0` から `1` に更新する

## 5. README.md rule map 更新

- [x] 5.1 README.md 内の rule map 表で MD056 行を見つけ、Safe fix 列を `Diagnostic only` から `Implemented (pads short rows; overflow rows remain diagnostic-only)` に変更する
- [x] 5.2 `make ast-lint` を実行して README ↔ catalog の整合性が保たれることを確認する

## 6. リリース成果物更新

- [x] 6.1 `CHANGELOG.md` の先頭に `## v0.12.18` セクションを追加し、MD056 safe-fix の追加内容を記述する
- [x] 6.2 `Cargo.toml` のバージョンを `0.12.17` から `0.12.18` に更新する
- [x] 6.3 `cargo update --workspace` を実行して `Cargo.lock` を更新する
- [x] 6.4 `openspec/changes/active-roadmap.md` を更新する：
  - v0.12.18 行を Done として追加
  - 残り P2 を v0.12.19+ に更新（MD003 / 終了宣言）

## 7. Quality Gates

- [x] 7.1 `cargo test -p katana-markdown-linter --lib md056` で MD056 ユニットテストが全通過することを確認する
- [x] 7.2 `make fmt-check` を実行する
- [x] 7.3 `make lint` （clippy）を実行する
- [x] 7.4 `make test`（cargo test --workspace --locked）を実行して全テストが通ることを確認する
- [x] 7.5 `make ast-lint` を実行して内部品質スコアが維持されていることを確認する
- [x] 7.6 `make dogfood` を実行して baseline が維持されることを確認する
- [x] 7.7 `make release-check VERSION=v0.12.18` を実行する
