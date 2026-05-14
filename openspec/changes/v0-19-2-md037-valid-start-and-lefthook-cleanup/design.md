## Context

PR #98 (v0.19.1) は、`**Note:** ... **docs-only sample**` のように同一行内に独立した強調範囲が複数並ぶケースで、MD037 が両者を 1 つの強調範囲として誤認し、`kml check --fix` が Markdown 構造を破壊する問題を修正した。原因は `valid_start` が句読点 `.!?,;:` を強調記号開始位置として許容していたため、`Note:` の閉じ `**` を新たな開始候補として扱い、後続の独立した強調記号と誤マッチしていた点にある。

最短 hotfix として `valid_start` の許容文字を `whitespace, ([{"'` のみへ縮小したが、これは CommonMark の left-flanking 規則よりも保守的で、`Hello.* spaced *` のような句読点直後のスペース付き強調を MD037 が見逃す可能性がある。v0.19.2 で本来の検出範囲を取り戻しつつ、連結バグの再発を防ぐ判定ロジックへ整理する。

加えて、PR #98 で導入された `lefthook.yml` の `run: sh -c 'just JOBS=2 check' -- {files}` 構成は、`{files}` を `sh` の位置引数として渡すだけで `just check` 自体は受け取らないため、意図が読み取りにくい。lefthook の `files` + `glob` skip 機構をシンプルに使う形へ整理する。

## Goals / Non-Goals

**Goals:**

- MD037 が CommonMark 準拠の left-flanking 条件で強調記号開始位置を判定する。句読点直後のスペース付き強調も検出できる。
- `**Note:** ... **docs-only sample**` の連結バグが回帰テストで再発しないことを保証する。
- `lefthook.yml` pre-push の構成が必要最小限で、意図が読み取れる。

**Non-Goals:**

- MD049 / MD050 など他の強調系ルールの判定ロジック変更。
- lefthook の他フック（pre-commit など）の整理。
- CommonMark 仕様への完全準拠（必要最小限の left-flanking のみ扱う）。

## Decisions

### Decision 1: `valid_start` を「直前文字が non-punctuation OR 強調内側に空白を含む候補のみ正当開始」へ整理

CommonMark の left-flanking delimiter run は「直前が空白 / 句読点 / 行頭、かつ直後が non-空白」であれば opener 候補となる。PR #98 の連結バグの真因は、閉じ側 `**` が「直後が空白」のため opener 候補と見做されない判定が抜けていた点にある。`valid_start` を以下の組み合わせで判定する:

1. 直前が行頭 OR 空白 OR `([{"'` → opener 候補（従来通り）
2. 直前が `.!?,;:` 等の句読点 → 直後が non-空白 のときのみ opener 候補
3. 直後が空白 OR 行末 → opener 候補から除外（閉じ側の連結誤認を防ぐ）

これにより `Hello.* spaced *` の `*` は (1)/(2) で opener 候補になり MD037 が検出する。`**Note:**` の閉じ `**` は (3) で除外され、後続独立強調との誤マッチを防ぐ。

**Alternatives considered:**

- (A) 現状の保守的縮小をそのまま据え置き → 句読点直後のスペース付き強調を見逃す副作用が残る。却下。
- (B) `matching_end_marker` 側で「直前 opener と直後候補のペア距離」を制約する → 判定責務が分散し maintainability が低下。却下。

### Decision 2: lefthook pre-push を `files` + `glob` のみで skip させる

lefthook は `files: <command>` で生成したファイル一覧を `glob` で絞り込み、結果が空なら command を skip する。`{files}` プレースホルダを run コマンド内で実際に消費しなくても、skip 判定は機能する想定。実環境で挙動を確認したうえで:

- 確認 OK の場合: `run: just JOBS=2 check` のシンプル形式へ戻す。
- 確認 NG の場合: `sh -c '...' -- {files}` を残し、`# {files} は lefthook の glob skip を発火させるためだけに必要` といった意図コメントを近接行に追加する。

**Alternatives considered:**

- (A) lefthook を捨てて Makefile/just script で代替 → 影響範囲が大きく Non-Goal。

## Risks / Trade-offs

- **Risk:** `valid_start` のロジック拡張が既存の MD037 / MD049 / MD050 fixture に予期せぬ差分を生む可能性 → Mitigation: `cargo test --test rule_fixture_harness MD037 MD049 MD050` で fixture 全件 pass を確認し、差分があれば fixture 側を意図的に更新するか、判定ロジックを再調整する。
- **Risk:** lefthook 構成変更によりローカル pre-push が誤動作 → Mitigation: 変更後に `lefthook run pre-push` をローカルで実行し、Markdown のみ変更時に skip、Rust 変更時に check が走ることを確認する。
- **Trade-off:** CommonMark の完全な flanking 規則ではなく必要十分な近似実装になるため、極端なケース（複合句読点の連続など）で見逃しが残る可能性。Non-Goal として許容。

## Migration Plan

破壊的変更なし。実装後に `cargo test` / `just check` / `lefthook run pre-push` を pass させてマージ。rollback は revert で完結する。
