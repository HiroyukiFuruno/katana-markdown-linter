# mcp-distribution Specification

## ADDED Requirements

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
