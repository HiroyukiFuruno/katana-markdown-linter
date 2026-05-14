# v0.19.2 MD037 valid_start Hardening and Lefthook Cleanup

## 対象バージョン

- `v0.19.2`

## Why

PR #98 (v0.19.1) は MD037 が同一行内の別強調範囲を 1 つの範囲として誤認し、`kml check --fix` が Markdown 構造を壊すリリースブロッカーを修正した。修正は最短経路として `valid_start` の許容文字を `whitespace, ([{"'` のみに縮小したが、レビューで以下 2 点の P2 指摘が残った。リリース遅延を避けるため v0.19.1 では先送りとし、v0.19.2 で品質を整える。

## What Changes

- MD037 (`no-space-in-emphasis`) の `valid_start` 判定を CommonMark の left-flanking 規則と突き合わせて再評価し、句読点直後（`. ! ? , ; :`）のスペース付き強調も検出可能な形へ整理する。連結バグが再発しないことを保証する回帰テストを増補する。
- `lefthook.yml` の pre-push.check-strict が `run: sh -c 'just JOBS=2 check' -- {files}` で `{files}` を空読みしている構成を整理する。lefthook の `files` + `glob` だけで skip が効くか検証し、可能なら `run: just JOBS=2 check` のシンプル形式へ戻す。残す場合は意図を inline コメントで明記する。

破壊的変更なし。

## Capabilities

### New Capabilities

なし。

### Modified Capabilities

- `rule-coverage`: MD037 の強調記号開始位置判定が、句読点直後を含む CommonMark 準拠の left-flanking 条件を扱うことを要件として明記する。同一行内の別強調範囲の連結が起きないことも要件化する。

## Impact

- `src/rules/markdown/rules/spaces_in_emphasis.rs` の `valid_start` ロジック調整、必要に応じて `matching_end_marker` 等の補助ロジックも整理。
- `tests/emphasis_regressions.rs` に句読点直後のスペース付き強調・連結誤認の双方をカバーする回帰テストを追加。
- `lefthook.yml` の pre-push 構成（dev tooling のみ。spec 範囲外）。
- 公開 API・CLI 互換性に影響なし。
