## Why

`kml` は Rust 組み込みを主目的とする library-first linter だが、agent や editor が shell command に依存せず diagnostics、fix preview、rule metadata、config validation を取得する入口がまだない。
既存の MCP evaluation で `kml-mcp` を optional adapter とする方針が固まったため、core crate を MCP-free に保ったまま read-only / text-first prototype を追加する。

## What Changes

- optional Cargo feature `mcp` を追加し、MCP SDK と async runtime を feature 配下に閉じ込める
- `kml-mcp` binary を追加し、`required-features = ["mcp"]` で通常 install/build から分離する
- MCP tools として `check_text`、`fix_text`、`config_validate`、`rule_list`、`rule_get` を公開する
- `fix_text` は fixed content と metadata を返すだけにし、workspace file へ書き込まない
- file read / file write tool は prototype scope から外し、path allowlist policy が実装されるまで提供しない
- README と docs に optional / experimental MCP server の使い方と責務境界を記載する

## Capabilities

### Modified Capabilities

- `mcp-integration`: evaluation-only から read-only / text-first prototype の実装 contract を追加する

## Impact

- `Cargo.toml`
- `Cargo.lock`
- `src/bin/kml-mcp.rs`
- `README.md`
- `docs/`
- `Makefile`
- OpenSpec main spec for `mcp-integration`
