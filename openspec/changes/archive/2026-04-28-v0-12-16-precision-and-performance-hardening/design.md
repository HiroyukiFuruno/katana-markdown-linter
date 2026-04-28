## Context

v0.12.15 まで MD046 は「fenced と indented が混在するファイルに対してファイル先頭に 1 診断」を出すのみで fix_info は常に `None` だった。
MD034 は `http://` / `https://` のみを検出し、`ftp://` / `mailto:` などは見逃していた。
また MD034 の `is_ignored_url()` は `inline_code_spans` 等に対して O(n) 線形走査を行っており、URL が多い文書でのコストが積み上がっていた。

v0.12.14 で `inside_code_span` に `partition_point` 二分探索を適用した実績があるため、同手法を MD034 にも展開する。

## Goals / Non-Goals

Goals: MD046 per-block safe-fix 追加、MD034 の ftp/ftps/mailto scheme 検出拡張、MD034 `is_ignored_url` O(n) → O(log n)。

Non-Goals: MD046 fenced→indented 変換、言語タグ自動推定、MD034 javascript/data scheme 検出、MD043/MD056/MD051 Unicode。

## Decisions

### D-1: MD046 診断単位を「ファイル単位」→「ブロックグループ単位」に変更

Why: safe-fix は診断単位で適用される。ファイル単位診断に multi-block fix を付けると fix の適用範囲が不透明になり safe fix contract に反する。ブロックごとに 1 診断を出せば fix も 1-to-1 で対応し、UI（MCP preview など）でも個別確認できる。

Alternative: ファイル単位診断のまま fix を multi-span にする → 実装が複雑で fix API が単一スパン前提のため非採用。

Invariant: `has_fenced && any_indented_block` のときだけ診断を発行する（純 indented ファイルは MD046 ではなく style consistency の別問題）。

### D-2: MD046 fix は「4 スペース除去 + triple-backtick フェンス追加」

Why: インデント code block の定義は先頭 4 スペース（または 1 タブ）。フェンス変換として最も単純で可逆。

空行で分割（別々のブロックとして複数診断）: 各 fix が小さく原子的になる。

### D-3: MD034 scheme 追加は `next_url_start` の find 呼び出し拡張

Why: `url_range()` / `is_ignored_url()` は scheme 非依存のため変更不要。`next_url_start` だけに scheme のリストを追加すればよい。regex は使わず文字列の `find` のままとする（v0.12.3 で regex を除去した経緯あり）。

`mailto:` は `://` を含まないため `url_range` が `end` を正しく計算できるか確認が必要: メールアドレス部分に空白は含まれないため既存ロジックで動作する。

### D-4: `is_ignored_url` を `partition_point` で高速化

v0.12.14 で `inside_code_span` に適用したパターンを流用。Before: O(n) linear scan で `span.line == line_index` を検索。After: `partition_point` で line 境界を特定し、O(log n) + O(k) （k: 同行 span 数）。

~~~rust
let lo = ctx.inline_code_spans().partition_point(|span| span.line < line_index);
ctx.inline_code_spans()[lo..]
    .iter()
    .take_while(|span| span.line == line_index)
    .any(|span| span.full_range.start <= pos && pos < span.full_range.end)
~~~

同じパターンを `inline_links` と `reference_definitions` にも適用する。

Invariant: `inline_code_spans()` / `inline_links()` / `reference_definitions()` は DocumentContext 構築時に行順・位置順でソートされていること（v0.12.5 の InlineIndex 実装で保証）。

## Risks / Trade-offs

- MD046 ブロック分割の変更: 現行の「1 診断」から「N 診断」への変更で既存 golden snapshot が変わる → fixture を更新して新 baseline を記録する
- MD034 `mailto:` の fix: `<user@example.com>` という fix は有効だが、HTML `<a>` タグと混同されやすい → `is_ignored_url` の `<` チェックがすでに `<user@example.com>` 形式を無視するため問題なし
- `partition_point` の前提: spans がソートされていない場合に誤った結果になる → DocumentContext の InlineIndex 実装でソートが保証されているため安全
