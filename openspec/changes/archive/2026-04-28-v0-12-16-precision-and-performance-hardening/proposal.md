## Why

v0.12.15 で MD052 collapsed-reference safe-fix と dead-code 削除を完了した。
次の precision-first ステップとして、MD046 に safe-fix を追加し、MD034 の検出対象 scheme を拡張する。
同時に MD034 の `is_ignored_url` 内の O(n) 線形走査を v0.12.14 の `partition_point` 手法で O(log n) に削減し、
URL が多い文書でのホットパスコストを下げる。

## What Changes

- **MD046 (`code-block-style`)**: 現行の「ファイル単位 1 診断 / fix なし」から「インデント code block グループ単位の診断 + safe-fix」に変更。各インデントブロックを fenced block に変換する fix を提供する。`is_safe_fix_rule` allowlist に追加。
- **MD034 (`no-bare-urls`)**: `next_url_start()` に `ftp://`、`ftps://`、`mailto:` を追加し、http(s) 以外の bare URL も検出・fix 対象とする。
- **MD034 パフォーマンス**: `is_ignored_url()` 内の `inline_code_spans` / `inline_links` / `reference_definitions` への線形走査を `partition_point` 二分探索に置換し、O(n) → O(log n) を達成する。

## Capabilities

### New Capabilities

- `md046-style-unification-fix`: MD046 がインデント code block を fenced block に変換する safe-fix を提供すること。グループ単位で診断を発行し、fix を適用すると 4-space インデントを除去して triple-backtick フェンスで囲む。
- `md034-extended-scheme-detection`: MD034 が `ftp://`、`ftps://`、`mailto:` の bare URL を検出・fix すること。また `is_ignored_url` の URL 包含チェックを二分探索で行い、URL 密度が高い文書でのチェックコストを削減する。

### Modified Capabilities

## Impact

- `src/rules/markdown/rules/md046.rs`: 診断ロジックをグループ単位に再設計し、fix_info を追加
- `src/rules/markdown/rules/md034.rs`: `next_url_start` scheme 追加、`is_ignored_url` を `partition_point` に置換
- `src/rules/markdown/eval.rs`: `is_safe_fix_rule` に `"MD046"` を追加
- `tests/ast_linter.rs`: MD046 safe-fix fixture テスト、MD034 scheme fixture テストを追加
- `tests/fixtures/rule-fixture-matrix.md`: MD046 の fix 列を更新
- `tests/fixtures/perf-baseline.json`: Perf-1 後に baseline を再計測して更新
