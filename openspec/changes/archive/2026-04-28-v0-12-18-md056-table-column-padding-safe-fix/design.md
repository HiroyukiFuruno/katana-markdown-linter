## Context

`src/rules/markdown/rules/md056.rs:24-55` は `ctx.tables()` を走査し、
header の `cells.len()` と各行の `cells.len()` を比較して不一致行に診断を出す。
現状は `fix_info: None` 固定。

`TableRow` 構造体（`src/rules/markdown/document.rs:62-71`）には
`cells: Vec<TableCell>`、`leading_pipe: bool`、`trailing_pipe: bool`、`safe_to_fix: bool` が既にあり、
MD055 (`src/rules/markdown/rules/md055.rs:123-136`) の `format_row` パターンを流用できる。
独自パーサ追加は不要。

## Goals / Non-Goals

このデザインの目標は以下の通り。
MD056 に「列数不足行のみ」を対象とした safe-fix を追加する。
過多行（`cells.len() > expected_columns`）と `safe_to_fix=false` 行は引き続き診断のみ。
既存 `TableBlock` / `TableRow` の構造には変更を加えない。
MD055 の format_row パターンを参考にして行のパイプスタイル（leading/trailing）を保持する。

スコープ外（Non-Goals）: 過多行の自動切り詰め（データ消失リスク）、
`TableBlock` / `TableRow` 構造体の変更、
MD003 / MD028 / MD059 の fix 追加（別パッチまたは by-design）。

## Decisions

### D-1: 過多行は fix しない（非対称な fix 戦略）

列数過多の行（例: `expected=2` だが `cells.len()=3`）は安全に削除できない。
末尾セルが意図された情報か、誤って `|` を打ったかは判別不能。
データ消失を回避するため `fix_info: None` を維持し、診断のみ報告する。

代替案: 末尾セルを末尾結合する → ユーザー意図に反する可能性が高く却下。

### D-2: 空セルは空文字を 1 文字で表現する

MD055 の `format_row` は cells を `" | "` で join し、leading/trailing は `"| "` / `" |"` で挟む。
このため空セル `""` を join すると `" | "` の境界に空文字が入り、
最終的に `"| 4 |  |"`（中央が空セル）として自然な出力となる。

代替案: ユーザー設定で空セル placeholder（例: `--`）を指定可能にする → 範囲を広げすぎるため却下。

### D-3: pipe style の継承戦略

各行の `(leading_pipe, trailing_pipe)` の組み合わせに応じて 4 パターン分岐する。
MD055 の `format_row` を踏襲し、行ごとに既存スタイルを保持する。
これにより MD055 と MD056 の修正結果が衝突しない。

~~~rust
match (row.leading_pipe, row.trailing_pipe) {
    (true, true)   => format!("| {inner} |"),
    (true, false)  => format!("| {inner}"),
    (false, true)  => format!("{inner} |"),
    (false, false) => inner,
}
~~~

### D-4: safe_to_fix=false の行はスキップ

`row.safe_to_fix` が `false` になるのは、escaped pipe（`\|`）やコード span を含む行です。
これらは cell 境界の判定が不確実なため fix 対象外。MD055 と同じガードを継承する。

## Risks / Trade-offs

`fix_info` を per-row で生成するため、`fix::apply` での edits 並べ替えで他のテーブル系 rule（MD055 / MD058）と
範囲が衝突する可能性がある。

[Risk] 同一行に MD055 と MD056 の両方が fix を提案するケース —
Mitigation: `fix::apply` は範囲オーバーラップした edit を後勝ちでスキップするため、
1 パスではどちらか片方のみ適用され、次のパスで残った方が適用される（既存の multi-pass 戦略）。

[Risk] 短い行に対する補完を期待しないユーザーがいる可能性 —
Mitigation: 仕様書（spec.md）に明記し、過多行は fix しないことを CHANGELOG にも記載する。
将来 unsafe-only モードで「過多行の切り詰め」を提供する選択肢を残す。
