## ADDED Requirements

### Requirement: release workflow SHALL build MCP wrapper binary artifacts for supported targets

release workflow は、`kml-mcp` と `kml-mcp-remote` の target 別 binary archive を build しなければならない（SHALL）。

#### Scenario: release builds MCP target matrix

- **WHEN** release workflow が `vX.Y.Z` を公開する。
- **THEN** system は `x86_64-unknown-linux-gnu`、`x86_64-apple-darwin`、`aarch64-apple-darwin`、`x86_64-pc-windows-msvc` の `kml-mcp` binary を build する。
- **AND** system は同じ target matrix の `kml-mcp-remote` binary を build する。
- **AND** `kml-mcp` は `mcp` feature を有効にして build される。
- **AND** `kml-mcp-remote` は `mcp-remote` feature を有効にして build される。
- **AND** target ごとの build failure は release failure として扱われる。

### Requirement: release workflow SHALL upload MCP wrapper binary artifacts with checksums

release workflow は、MCP wrapper 用 binary archive と checksum を GitHub Release に添付しなければならない（SHALL）。

#### Scenario: release uploads MCP assets

- **WHEN** release workflow が GitHub Release を作成または更新する。
- **THEN** system は target ごとの `kml-mcp` archive を release asset として upload する。
- **AND** system は target ごとの `kml-mcp-remote` archive を release asset として upload する。
- **AND** system は各 archive の `.sha256` file を release asset として upload する。
- **AND** existing release を更新する場合、system は同じ名前の MCP asset を安全に置き換える。

### Requirement: local and CI gates SHALL validate wrapper MCP entrypoints

local release gate と CI release gate は、npm / bun / uv wrapper 経由の MCP entrypoint を検証しなければならない（SHALL）。

#### Scenario: local release check runs wrapper MCP smoke

- **WHEN** developer runs `just VERSION=vX.Y.Z release-check`。
- **THEN** system builds or uses local MCP binary archives for current platform。
- **AND** system runs an `npx` or local npm package smoke for `kml-mcp`。
- **AND** system runs a `bunx` smoke for `kml-mcp` when bun is available in the release environment。
- **AND** system runs a `uvx` smoke for `kml-mcp`。
- **AND** each smoke performs MCP JSON-RPC initialization and at least one tool call。

#### Scenario: release workflow runs wrapper MCP smoke

- **WHEN** release workflow prepares wrapper publication。
- **THEN** system validates npm package metadata exposes `kml-mcp` and `kml-mcp-remote` bins。
- **AND** system validates PyPI metadata exposes `kml-mcp` and `kml-mcp-remote` console scripts。
- **AND** system fails before registry publish if wrapper MCP smoke fails。

### Requirement: release verification SHALL include MCP wrapper assets and entrypoints

release verification は、公開後に MCP wrapper asset と wrapper entrypoint の利用可能性を確認しなければならない（SHALL）。

#### Scenario: release verify checks MCP wrapper assets

- **WHEN** developer runs `just VERSION=vX.Y.Z release-verify` after publication。
- **THEN** system verifies GitHub Release contains target-specific `kml-mcp` archives and checksum files。
- **AND** system verifies GitHub Release contains target-specific `kml-mcp-remote` archives and checksum files。
- **AND** system smoke-tests at least the current platform `kml-mcp` archive through MCP stdio。
- **AND** system smoke-tests at least the current platform `kml-mcp-remote` archive through Streamable HTTP。

#### Scenario: release verify checks registry wrapper entrypoints

- **WHEN** npm and PyPI wrapper versions are published。
- **THEN** system verifies npm exposes `kml-mcp` and `kml-mcp-remote` entrypoints。
- **AND** system verifies PyPI exposes `kml-mcp` and `kml-mcp-remote` console scripts。
- **AND** system verifies `npx`, `bunx`, or `uvx` wrapper smoke for `kml-mcp` according to documented support。
- **AND** missing MCP entrypoints are release verification failures。
