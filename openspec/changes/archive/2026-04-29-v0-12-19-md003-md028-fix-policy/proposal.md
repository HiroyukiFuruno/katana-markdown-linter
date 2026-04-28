## Why

`v0.12.18` で `MD056` の安全な自動修正（safe-fix）まで進んだが、`MD003` と `MD028` はまだ `Diagnostic only` のまま残っている。
`v0.12.19` では、この 2 ルールを実装可能性と安全性の境界まで詰め、0.12.x 終端に向けた未整理状態を減らす。

## What Changes

- `MD003` (`heading-style`) に、source range を保った安全な自動修正を追加する。
- `MD003` の修正範囲は、setext 見出しから ATX 見出しへの変換を中心にし、設定値ごとの安全条件を明文化する。
- `MD028` (`no-blanks-blockquote`) は、空行に `>` を足す修正が文意を変えないと証明できる範囲だけを候補にする。
- `MD028` に安全な範囲を定義できない場合は、実装を無理に入れず、`v0.12.21` の「意図的に実装しない」宣言へ送る。
- README の rule map、fixture matrix、CHANGELOG、active roadmap を更新する。

## Capabilities

### New Capabilities

### Modified Capabilities

- `rule-coverage`: `MD003` の safe-fix と、`MD028` の fix 方針決定を rule coverage contract に追加する。

## Impact

- `src/rules/markdown/rules/heading_style.rs`
- `src/rules/markdown/rules/blockquote.rs`
- `src/lib.rs` の safe-fix allowlist
- `tests/fixtures/rule-fixture-matrix.json`
- `tests/fixtures/rule-fixture-matrix.md`
- `README.md`
- `CHANGELOG.md`
- `Cargo.toml`
- `Cargo.lock`
- `openspec/changes/active-roadmap.md`
