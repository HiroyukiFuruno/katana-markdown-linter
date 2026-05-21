# v0.19.3 MCP Wrapper Entrypoints

## 対象バージョン

- `v0.19.3`

## Why

`kml-mcp` は MCP stdio server として動作し、release gate でも検証されている。一方で、利用者が Codex / Claude Code / Antigravity などへ登録するには、現状 `cargo install --features mcp --bin kml-mcp` または release MCPB を前提にする必要があり、`npx` / `bunx` / `uvx` で一時実行できる理想形には届いていない。

`v0.19.3` では、既存の npm / PyPI thin wrapper を MCP entrypoint まで拡張し、Rust toolchain なしで LLM client から `kml-mcp` を起動できる配布契約を固定する。

## What Changes

- npm wrapper に `kml-mcp` / `kml-mcp-remote` 起動口を追加し、`npx` / `bunx` から MCP server を起動できるようにする。
- PyPI wrapper に `kml-mcp` / `kml-mcp-remote` console script を追加し、`uvx` から MCP server を起動できるようにする。
- GitHub Release に `kml-mcp` / `kml-mcp-remote` 用の target 別 binary archive と checksum を追加する。
- wrapper installer は `kml` / `kml-mcp` / `kml-mcp-remote` の役割ごとに、対応する公式 archive を download / checksum verify / cache / execute する。
- release gate に `npx` / `bunx` / `uvx` 経由の MCP smoke test を追加する。
- README / docs / wrapper README / release runbook に、LLM client 登録で使う具体的なコマンド例を追加する。

破壊的変更なし。既存の `kml` CLI wrapper の挙動は維持する。

## Capabilities

### New Capabilities

なし。

### Modified Capabilities

- `binary-distribution`: npm / PyPI wrapper が `kml` だけでなく `kml-mcp` / `kml-mcp-remote` も薄く起動できることを配布契約へ追加する。
- `mcp-distribution`: `kml-mcp` / `kml-mcp-remote` が `npx` / `bunx` / `uvx` で起動できる公式導線を持つことを MCP 配布契約へ追加する。
- `release-cicd`: release workflow / local release gate が MCP wrapper entrypoint と MCP binary archive を検証することを追加する。

## Impact

- `scripts/release/binary_artifacts.py`
- `scripts/ci/mcp-stdio-smoke.py`
- `scripts/ci/mcp-remote-smoke.py`
- `scripts/release/smoke-wrappers.sh`
- `scripts/release/verify-release-published.sh`
- `scripts/release/verify-npm-package.js`
- `scripts/release/verify-pypi-package.py`
- `wrappers/npm/**`
- `wrappers/python/**`
- `just/mcp.just`
- `just/release.just`
- `.github/workflows/release.yml`
- `docs/mcp-server.md`
- `docs/remote-mcp-transport.md`
- `docs/distribution.md`
- `docs/release-runbook.md`
- `README.md`
