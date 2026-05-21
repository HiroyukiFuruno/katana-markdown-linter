## ADDED Requirements

### Requirement: MCP server binary archives SHALL be published for wrapper entrypoints

システムは、npm / PyPI wrapper が `kml-mcp` と `kml-mcp-remote` を Rust toolchain なしで起動できるよう、target 別 MCP server binary archive を publish しなければならない（SHALL）。

#### Scenario: release uploads MCP binary archives

- **WHEN** release workflow が `vX.Y.Z` を公開する。
- **THEN** system は supported target ごとの `kml-mcp` binary archive を作成する。
- **AND** system は supported target ごとの `kml-mcp-remote` binary archive を作成する。
- **AND** archive 名は `kml-mcp-vX.Y.Z-<target>.tar.gz` / `.zip` と `kml-mcp-remote-vX.Y.Z-<target>.tar.gz` / `.zip` になる。
- **AND** system は各 archive と隣接する `.sha256` file を GitHub Release に添付する。

#### Scenario: MCP archive smoke test runs protocol call

- **WHEN** system が `kml-mcp` binary archive を展開する。
- **THEN** system は展開された `kml-mcp` または `kml-mcp.exe` を MCP stdio server として起動する。
- **AND** system は JSON-RPC `initialize` と `tools/list` が成功することを検証する。
- **AND** system は少なくとも 1 つの text tool call が成功することを検証する。

#### Scenario: remote MCP archive smoke test runs HTTP call

- **WHEN** system が `kml-mcp-remote` binary archive を展開する。
- **THEN** system は bearer token 付きで remote MCP server を起動する。
- **AND** system は Streamable HTTP endpoint で `initialize` と `tools/list` が成功することを検証する。
- **AND** remote `tools/list` は workspace file tools を公開しない。

### Requirement: wrappers SHALL cache binaries by executable role

npm / PyPI wrapper は、download 済み binary cache を package version、target platform、executable role ごとに分離しなければならない（SHALL）。

#### Scenario: wrapper installs CLI and MCP binaries

- **WHEN** user が同じ wrapper version で `kml` と `kml-mcp` を実行する。
- **THEN** wrapper は `kml` と `kml-mcp` を別々の cache path に保存する。
- **AND** wrapper は `kml` cache を `kml-mcp` 実行に再利用しない。
- **AND** wrapper は `kml-mcp` cache を `kml` 実行に再利用しない。

## MODIFIED Requirements

### Requirement: npm and pip wrappers SHALL be thin launchers

npm / pip の薄いラッパー（wrapper）は、`kml`、`kml-mcp`、`kml-mcp-remote` の独自実装を持ってはならない（SHALL NOT）。

#### Scenario: wrapper installs kml

- **WHEN** user が npm または pip wrapper を install する。
- **THEN** wrapper は platform に対応する公式 `kml` binary archive を取得する。
- **AND** wrapper は checksum を検証する。
- **AND** wrapper は取得した `kml` binary を実行する。
- **AND** wrapper は lint rule や formatter logic を実装しない。

#### Scenario: wrapper installs kml-mcp

- **WHEN** user が npm、bun、または PyPI wrapper 経由で `kml-mcp` を起動する。
- **THEN** wrapper は platform に対応する公式 `kml-mcp` binary archive を取得する。
- **AND** wrapper は checksum を検証する。
- **AND** wrapper は取得した `kml-mcp` binary を stdio を壊さずに実行する。
- **AND** wrapper は MCP protocol や lint logic を再実装しない。

#### Scenario: wrapper installs kml-mcp-remote

- **WHEN** user が npm、bun、または PyPI wrapper 経由で `kml-mcp-remote` を起動する。
- **THEN** wrapper は platform に対応する公式 `kml-mcp-remote` binary archive を取得する。
- **AND** wrapper は checksum を検証する。
- **AND** wrapper は取得した `kml-mcp-remote` binary を実行する。
- **AND** wrapper は remote MCP protocol や lint logic を再実装しない。

### Requirement: npm wrapper package SHALL include registry-visible usage documentation

npm wrapper package は、npm registry page 上で導入方法、MCP 起動方法、thin wrapper の責務を説明できなければならない（SHALL）。

#### Scenario: npm package is packed

- **WHEN** system builds the npm package tarball for `vX.Y.Z`。
- **THEN** tarball contains `README.md`。
- **AND** README includes global install and `npx` examples for `katana-markdown-linter`。
- **AND** README includes `npx --yes katana-markdown-linter@X.Y.Z kml-mcp --workspace-root <path>` or the exact verified `npx` equivalent。
- **AND** README includes `bunx katana-markdown-linter@X.Y.Z kml-mcp --workspace-root <path>` or the exact verified `bunx` equivalent。
- **AND** README states that the npm package is a thin launcher over GitHub Release binary archives。
- **AND** README lists supported platforms or points to the supported platform contract。
- **AND** README does not imply npm contains independent lint or MCP server logic。

### Requirement: npm wrapper package SHALL keep dependency surface minimal

npm wrapper package は、thin wrapper に不要な runtime dependency を追加してはならない（SHALL NOT）。

#### Scenario: package metadata is inspected

- **WHEN** developer reviews `wrappers/npm/package.json`。
- **THEN** package keeps runtime dependencies empty unless a specific dependency is justified by wrapper behavior。
- **AND** package metadata includes search and support fields such as `keywords`, `homepage`, and `bugs`。
- **AND** package exposes `bin.katana-markdown-linter`, `bin.kml`, `bin.kml-mcp`, and `bin.kml-mcp-remote`。
- **AND** `bin.kml` points to the CLI launcher。
- **AND** `bin.kml-mcp` points to the stdio MCP launcher。
- **AND** `bin.kml-mcp-remote` points to the remote MCP launcher。

### Requirement: PyPI wrapper package SHALL include registry-visible usage documentation

PyPI wrapper package は、PyPI project page 上で導入方法、MCP 起動方法、thin wrapper の責務を説明できなければならない（SHALL）。

#### Scenario: PyPI package is built

- **WHEN** system builds the PyPI source distribution and wheel for `vX.Y.Z`。
- **THEN** project metadata points `readme` at `README.md`。
- **AND** README includes install and `uvx` examples for `katana-markdown-linter`。
- **AND** README includes `uvx --from katana-markdown-linter==X.Y.Z kml-mcp --workspace-root <path>`。
- **AND** README states that the PyPI package is a thin launcher over GitHub Release binary archives。
- **AND** README lists supported platforms or points to the supported platform contract。
- **AND** README does not imply Python contains independent lint or MCP server logic。

### Requirement: PyPI wrapper package SHALL keep dependency surface minimal

PyPI wrapper package は、thin wrapper に不要な runtime dependency を追加してはならない（SHALL NOT）。

#### Scenario: project metadata is inspected

- **WHEN** developer reviews `wrappers/python/pyproject.toml`。
- **THEN** package keeps runtime dependencies empty unless a specific dependency is justified by wrapper behavior。
- **AND** package metadata includes search and support fields such as `keywords` and project URLs。
- **AND** package keeps the `kml` console script pointing to the CLI launcher module。
- **AND** package exposes `kml-mcp` as a console script pointing to the stdio MCP launcher module。
- **AND** package exposes `kml-mcp-remote` as a console script pointing to the remote MCP launcher module。
