# mcp-integration Specification

## ADDED Requirements

### Requirement: remote MCP transport SHALL be explicitly separated from stdio

The project SHALL distinguish remote MCP transport from the local stdio MCP server.

#### Scenario: documentation describes MCP support

- **GIVEN** local stdio MCP support is available
- **WHEN** documentation describes API-hosted LLM usage
- **THEN** the documentation SHALL state that provider API direct usage requires remote transport
- **AND** the documentation SHALL NOT present local stdio as a remote MCP connector

### Requirement: remote MCP server SHALL expose only safe capabilities by default

The remote MCP server SHALL default to non-mutating operations unless explicit
write safety requirements are implemented.

#### Scenario: remote capabilities are listed

- **GIVEN** a remote MCP client calls `tools/list`
- **WHEN** the server returns available tools
- **THEN** text-only diagnostics and metadata tools MAY be available
- **AND** workspace write tools SHALL be absent or marked unavailable unless auth, audit, and explicit apply are enabled

### Requirement: workspace-backed remote mode SHALL enforce authenticated workspace boundaries

Workspace-backed remote mode SHALL require authenticated access and server-side
workspace boundaries.

#### Scenario: remote file tool resolves a path

- **GIVEN** a remote session has an assigned workspace root
- **WHEN** a file tool receives a path
- **THEN** the path SHALL resolve under the assigned workspace root
- **AND** root escape attempts SHALL be rejected
- **AND** anonymous sessions SHALL NOT perform file writes

### Requirement: core library SHALL remain independent of MCP transport

The core `katana-markdown-linter` library SHALL remain independent of local or
remote MCP transport dependencies.

#### Scenario: library consumer builds without MCP features

- **GIVEN** a consumer depends on `katana-markdown-linter` as a library
- **WHEN** the consumer builds without MCP features
- **THEN** MCP transport dependencies SHALL NOT be required
- **AND** public lint / fix APIs SHALL remain available
