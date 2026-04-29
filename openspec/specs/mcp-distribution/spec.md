# mcp-distribution Specification

## Purpose

MCP distribution defines how `kml-mcp` is packaged, described, and published
without overstating local stdio, MCPB, Registry, or remote transport support.

## Requirements

### Requirement: Registry publication SHALL be explicitly deferred

The project SHALL NOT publish `kml-mcp` to MCP Registry or third-party MCP hubs
until a dedicated public-readiness gate has passed.

#### Scenario: Current planning change does not publish

- **GIVEN** the project has a local stdio MCP server
- **WHEN** the MCP Registry planning change is completed
- **THEN** no registry publish command SHALL have been run as part of the change
- **AND** the required future publication conditions SHALL be documented

### Requirement: registry package type SHALL match distribution reality

The project SHALL choose an MCP Registry package type only after confirming that
the selected artifact can install and run `kml-mcp` without misleading users.

#### Scenario: crates.io alone is insufficient

- **GIVEN** `katana-markdown-linter` is published to crates.io
- **WHEN** MCP Registry package metadata is prepared
- **THEN** the plan SHALL record that crates.io is not currently a direct official MCP Registry package type
- **AND** the plan SHALL evaluate MCPB or OCI before adding npm or PyPI wrappers

### Requirement: server metadata SHALL expose workspace safety limits

The project SHALL describe workspace path and write behavior in MCP distribution metadata or workspace safety documentation.

#### Scenario: User reviews the MCP server listing

- **GIVEN** a user discovers `kml-mcp` through Registry or Hub metadata
- **WHEN** the metadata points to installation and usage documentation
- **THEN** the documentation SHALL state that workspace paths are rooted under `--workspace-root`
- **AND** the documentation SHALL state that file writes require explicit apply
- **AND** the documentation SHALL NOT imply directory-wide automatic mutation support

### Requirement: API-hosted LLM use SHALL require a compatible transport

The project SHALL distinguish local stdio MCP use from provider API remote MCP use.

#### Scenario: Embedded app uses kml directly

- **GIVEN** an application embeds `katana-markdown-linter`
- **WHEN** the application wants Markdown diagnostics or fixes
- **THEN** the application SHOULD call the Rust library API directly unless it needs MCP interoperability

#### Scenario: Provider API needs remote MCP

- **GIVEN** an LLM provider API can call remote MCP servers
- **WHEN** the application wants the provider API to invoke `kml-mcp` directly
- **THEN** the plan SHALL require a remote MCP transport or wrapper before claiming support

### Requirement: MCP package artifact SHALL be reproducible

The project SHALL produce a repeatable `kml-mcp` package artifact from the
repository release inputs.

#### Scenario: package artifact is built

- **GIVEN** the selected package type is recorded by `v0.13.0`
- **WHEN** the release gate builds the MCP package artifact
- **THEN** the artifact SHALL include the `kml-mcp` binary built with the `mcp` feature
- **AND** the artifact SHALL expose a stdio server entrypoint
- **AND** the artifact SHALL be smoke-tested before Registry publication

### Requirement: Registry metadata SHALL point to a verified install path

The project SHALL publish Registry metadata only when the referenced package
artifact can install and run.

#### Scenario: Registry metadata is validated

- **GIVEN** `server.json` references a package artifact
- **WHEN** metadata validation runs
- **THEN** the referenced install path SHALL match the package artifact smoke test
- **AND** the metadata SHALL reference workspace safety documentation
- **AND** the metadata SHALL NOT claim remote MCP support unless the remote transport change is complete

### Requirement: Registry publication SHALL be gated by safety review

The project SHALL require a completed MCP safety review before public Registry
or Hub publication.

#### Scenario: publication is attempted

- **GIVEN** the package artifact and `server.json` are ready
- **WHEN** a maintainer publishes to MCP Registry or a third-party Hub
- **THEN** the security checklist SHALL be complete
- **AND** file write behavior SHALL remain preview or explicit apply only
- **AND** directory-wide automatic mutation SHALL remain unavailable

### Requirement: Published MCP distribution SHALL preserve local workspace policy

The published distribution SHALL keep the same workspace boundary as the local
stdio server.

#### Scenario: packaged server handles workspace paths

- **GIVEN** the user starts the packaged server with `--workspace-root`
- **WHEN** a file tool receives a relative path
- **THEN** the path SHALL be resolved under the workspace root
- **AND** root escape attempts SHALL be rejected
- **AND** ignored or unsupported files SHALL follow the documented policy
