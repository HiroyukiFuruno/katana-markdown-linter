## Why

`kml` は v0.2.0 で release flow まで整ったが、この repository 自身の Markdown に対して日常的に使う導線はまだ固定されていない。
CLI と Rust 組み込み API の実利用を先に固めることで、次の互換性強化や MCP 化の前に使い勝手の粗を発見できる。

## What Changes

- repository 内 Markdown を `kml check` / `kml check --fix` で dogfood する local workflow を追加する
- dogfood 対象、除外対象、archive 扱い、JSON output の扱いを明文化する
- CLI の実利用から出た UX issue を actionable な report として残す
- Rust 組み込み用途の public API example を追加し、`check string`、`check file tree`、`fix string`、`config load` を示す
- dogfood workflow は通常 gate では check-only とし、fix は明示操作に限定する

## Capabilities

### New Capabilities

- `dogfood-workflow`: repository 自身に `kml` を適用する対象範囲、実行結果、運用ルールを定義する
- `public-api-examples`: Rust 組み込み利用の最小例と実行可能性を定義する

### Modified Capabilities

- なし

## Impact

- `Makefile`
- `README.md`
- `docs/`
- `examples/`
- CLI 実行時の ignore / config discovery / JSON output の実利用確認
- OpenSpec 文書は対象に含めるが、archive 配下は既定では変更対象にしない
