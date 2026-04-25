# mcp-distribution Specification

## ADDED Requirements

### Requirement: Registry Publication Is Explicitly Deferred

The project SHALL NOT publish `kml-mcp` to MCP Registry or third-party MCP hubs
until a dedicated public-readiness gate has passed.

#### Scenario: Current planning change does not publish

- **GIVEN** the project has a local stdio MCP server
- **WHEN** the MCP Registry planning change is completed
- **THEN** no registry publish command SHALL have been run as part of the change
- **AND** the required future publication conditions SHALL be documented

### Requirement: Registry Package Type Must Match Distribution Reality

The project SHALL choose an MCP Registry package type only after confirming that
the selected artifact can install and run `kml-mcp` without misleading users.

#### Scenario: crates.io alone is insufficient

- **GIVEN** `katana-markdown-linter` is published to crates.io
- **WHEN** MCP Registry package metadata is prepared
- **THEN** the plan SHALL record that crates.io is not currently a direct official MCP Registry package type
- **AND** the plan SHALL evaluate MCPB or OCI before adding npm or PyPI wrappers

### Requirement: API-Hosted LLM Use Requires A Compatible Transport

The project SHALL distinguish local stdio MCP use from provider API remote MCP use.

#### Scenario: Embedded app uses kml directly

- **GIVEN** an application embeds `katana-markdown-linter`
- **WHEN** the application wants Markdown diagnostics or fixes
- **THEN** the application SHOULD call the Rust library API directly unless it needs MCP interoperability

#### Scenario: Provider API needs remote MCP

- **GIVEN** an LLM provider API can call remote MCP servers
- **WHEN** the application wants the provider API to invoke `kml-mcp` directly
- **THEN** the plan SHALL require a remote MCP transport or wrapper before claiming support
