## ADDED Requirements

### Requirement: MCP stdio server SHALL be launchable through npm, bun, and uv wrappers

`kml-mcp` は、公式 wrapper package から Rust toolchain なしで起動できなければならない（SHALL）。

#### Scenario: npx starts local stdio MCP

- **WHEN** user runs `npx --yes katana-markdown-linter@X.Y.Z kml-mcp --workspace-root <absolute-workspace>` or the exact verified `npx` equivalent。
- **THEN** wrapper downloads and verifies the matching `kml-mcp` release archive。
- **AND** wrapper starts `kml-mcp` over stdio without writing non-protocol text to stdout。
- **AND** an MCP client can call `initialize`, `tools/list`, and at least one text tool successfully。

#### Scenario: bunx starts local stdio MCP

- **WHEN** user runs `bunx katana-markdown-linter@X.Y.Z kml-mcp --workspace-root <absolute-workspace>` or the exact verified `bunx` equivalent。
- **THEN** wrapper downloads and verifies the matching `kml-mcp` release archive。
- **AND** wrapper starts `kml-mcp` over stdio without writing non-protocol text to stdout。
- **AND** an MCP client can call `initialize`, `tools/list`, and at least one text tool successfully。

#### Scenario: uvx starts local stdio MCP

- **WHEN** user runs `uvx --from katana-markdown-linter==X.Y.Z kml-mcp --workspace-root <absolute-workspace>`。
- **THEN** wrapper downloads and verifies the matching `kml-mcp` release archive。
- **AND** wrapper starts `kml-mcp` over stdio without writing non-protocol text to stdout。
- **AND** an MCP client can call `initialize`, `tools/list`, and at least one text tool successfully。

### Requirement: remote MCP server SHALL be launchable through npm, bun, and uv wrappers

`kml-mcp-remote` は、公式 wrapper package から Rust toolchain なしで起動できなければならない（SHALL）。

#### Scenario: wrapper starts remote MCP

- **WHEN** user starts `kml-mcp-remote` through npm, bun, or uv wrapper with `KML_MCP_REMOTE_TOKEN` set。
- **THEN** wrapper downloads and verifies the matching `kml-mcp-remote` release archive。
- **AND** wrapper starts the remote MCP server with the documented default localhost endpoint。
- **AND** an MCP client can call `initialize` and `tools/list` over Streamable HTTP。
- **AND** remote `tools/list` does not expose workspace file tools。

### Requirement: wrapper MCP documentation SHALL provide client-ready command arguments

MCP documentation は、LLM client の MCP 設定へそのまま分解できる command / args を示さなければならない（SHALL）。

#### Scenario: user copies Codex local stdio configuration

- **WHEN** user reads the MCP server documentation for Codex。
- **THEN** documentation includes a `command` value using `npx` or `uvx`。
- **AND** documentation includes an `args` list with package version, `kml-mcp`, `--workspace-root`, and an absolute workspace path placeholder。
- **AND** documentation states that workspace paths stay under `--workspace-root`。
- **AND** documentation states that file writes still require explicit `fix_file_apply` with `apply: true`。

### Requirement: wrapper entrypoints SHALL not change MCP safety boundaries

`npx` / `bunx` / `uvx` entrypoints は、既存の MCP workspace safety boundary を弱めてはならない（SHALL NOT）。

#### Scenario: packaged wrapper handles workspace paths

- **GIVEN** user starts `kml-mcp` through a wrapper with `--workspace-root`。
- **WHEN** a file tool receives a relative path。
- **THEN** the path is resolved under the workspace root。
- **AND** root escape attempts are rejected。
- **AND** file writes require the same explicit apply behavior as the direct `kml-mcp` binary。

#### Scenario: remote wrapper exposes tools

- **GIVEN** user starts `kml-mcp-remote` through a wrapper。
- **WHEN** an MCP client calls `tools/list`。
- **THEN** only text and metadata tools are exposed。
- **AND** workspace read or write tools are absent。
