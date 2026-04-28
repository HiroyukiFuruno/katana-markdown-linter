## 1. MD034 Scheme 拡張 + パフォーマンス改善（C-2 + Perf-1）

- [x] 1.1 `src/rules/markdown/rules/md034.rs` の `next_url_start()` に `ftp://`、`ftps://`、`mailto:` の検索を追加する
- [x] 1.2 `is_ignored_url()` 内の `inline_code_spans` 線形走査を `partition_point` + `take_while` に置換する
- [x] 1.3 `is_ignored_url()` 内の `inline_links` 線形走査を同様に `partition_point` + `take_while` に置換する
- [x] 1.4 `is_ignored_url()` 内の `reference_definitions` 線形走査を同様に `partition_point` + `take_while` に置換する
- [x] 1.5 `tests/ast_linter.rs` に MD034 の ftp:// / mailto: 検出 fixture テストを追加する
- [x] 1.6 MD034 のインラインコードスパン内 ftp:// が無視されることを確認するテストを追加する
- [x] 1.7 `make test` を実行して全テストが通ることを確認する

## 2. MD046 Safe-Fix 追加（C-1）

- [x] 2.1 `src/rules/markdown/rules/md046.rs` で連続する indented code lines をグループ化するヘルパー関数 `indented_code_block_groups()` を実装する
- [x] 2.2 `evaluate_context` を「ファイル単位 1 診断」から「ブロック単位の複数診断」に変更し、各診断に `fix_info` を付与する（D-2 の replacement format に従う）
- [x] 2.3 `src/rules/markdown/eval.rs` の `is_safe_fix_rule` allowlist に `"MD046"` を追加する
- [x] 2.4 `tests/ast_linter.rs` に MD046 safe-fix fixture テスト（single block / multi-block / list exclusion / pure-indented-only）を追加する
- [x] 2.5 `tests/fixtures/rule-fixture-matrix.md` の MD046 行の fix 列を `✓` に更新する
- [x] 2.6 `make test` を実行して全テストが通ることを確認する

## 3. Quality Gates

- [x] 3.1 `make bench` を実行してパフォーマンス回帰（1.40× ゲート）がないことを確認する
- [x] 3.2 `tests/fixtures/perf-baseline.json` を最新のベンチマーク結果で更新する
- [x] 3.3 `make ast-lint` を実行して内部品質スコアが維持されていることを確認する
- [x] 3.4 `openspec/changes/active-roadmap.md` の P2 アイテムに `v0-12-16-precision-and-performance-hardening` を記入する
