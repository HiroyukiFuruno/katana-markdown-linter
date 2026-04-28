## Context

`fix::apply` (`src/fix/mod.rs`) は既に accepted/skipped を内部で計算しているが、
`FixResult { content, applied_fixes }` としか返さず呼び出し元は何をどこに適用したか知れない。

MD051 の `github_heading_slug` (`src/rules/markdown/rules/md051/fragments.rs`) は
`ch.is_alphanumeric()` を用いており、Rust の Unicode Alphabetic/Numeric 定義で判定する。
emoji（Unicode Symbol/Other カテゴリ）は `is_alphanumeric()` が false になるため文字がドロップされる。
`_` を保持しているが GitHub は実際にはアンダースコアも slug に含めるため不一致はない。
既存の実装は大部分正しいが、GitHub の実際のフラグメント生成との差分を厳密に仕様化し、
regression テストを充実させる。

## Goals / Non-Goals

このデザインが達成する目標は以下の通り。`FixDetail` を公開 API として追加し、
`FixResult.details` で per-fix の rule_id・range・applied を返す。
`fix_with_results` / `fix_with_results_including_unsafe` が `FixDetail` を詰めて返す。
`applied_fixes` / `content` フィールドは後方互換のまま維持する。
MD051 `github_heading_slug` の挙動を GitHub の実際のルールと一致させる仕様をテストで固める。
emoji・CJK 混在見出しで発生する false negative / false positive を排除する。

スコープ外（Non-Goals）: CLI 出力形式の変更（`FixDetail` は型として公開するがデフォルト CLI には表示しない）、
MD043 / MD056 の fix 追加（v0.12.18+ 候補）、`LintResult` 構造体の変更。

## Decisions

### D-1: `FixDetail` に `range` として `Range` 型を再利用する

`Range { start_line, start_column, end_line, end_column }` は既存の公開型。
新たに `FixRange` を作ると API が複雑になるため、同じ `Range` を `FixDetail.range` に使う。
`applied: bool` は accepted/skipped を区別する最小フィールド。

代替案: `skipped_fixes: usize` だけを追加 — rule_id レベルの粒度が得られないため却下。

### D-2: `fix::apply` に `detail` 情報を乗せる方法

edits タプルに `(start, end, replacement, rule_id, range)` を追加し、
accepted ループ内で `FixDetail { rule_id, range, applied: true }` を生成。
スキップされた edit は後から `applied: false` として別途収集する。

シンプルさ優先: skipped edits は `applied_fixes` と同様に count のみでもよいが、
Issue #43 の要求は rule_id 単位の区別なので detail 付きで返す。

### D-3: MD051 fragment 正規化の方針

現行の `github_heading_slug` は emoji・特殊記号を無音で除去しており、
GitHub の動作（emoji 除去・Unicode 字母保持・スペース→ハイフン・重複ハイフン除去）と概ね一致している。
主な確認事項: `trim_end_matches('#')` による見出しレベル文字の除去、先頭・末尾ハイフンの除去、
`_` を保持（GitHub と一致）、`previous_dash` フラグによる重複ハイフン除去は既存実装で済んでいる。
regression テストを emoji・CJK・混在ケースで網羅し、挙動を固めることが主な成果物。

大規模な書き直しは不要。

## Risks / Trade-offs

`FixResult` への `details` フィールド追加は semver minor のため、Cargo.toml version は 0.12.17 で OK。
`FixDetail` が `Serialize` を持つと JSON シリアライズが可能になり、将来の CLI 出力拡張が容易になる。

[Risk] MD051 の fragment 正規化変更が既存の golden test と衝突する —
Mitigation: 変更前に golden を確認し、許容 delta として登録する。

[Risk] `fix::apply` の edits ループにタプル要素を追加すると可読性が下がる —
Mitigation: named struct で包む。
