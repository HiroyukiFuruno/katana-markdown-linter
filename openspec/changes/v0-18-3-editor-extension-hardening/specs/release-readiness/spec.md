## ADDED Requirements

### Requirement: release readiness SHALL include editor extension checks

release readiness は、editor extension package と LSP launch の検証を含まなければならない（SHALL）。

#### Scenario: release check runs editor extension gates

- **WHEN** developer runs `just VERSION=vX.Y.Z release-check`
- **THEN** system runs VS Code extension check
- **AND** system runs Zed extension check
- **AND** system verifies both checks use the same `kml` release version

### Requirement: post-release verification SHALL cover editor extension artifacts when published

公開後検証は、editor extension artifact が公開された場合にその状態を確認しなければならない（SHALL）。

#### Scenario: extension artifact is published

- **WHEN** developer runs `just VERSION=vX.Y.Z release-verify`
- **THEN** system checks published extension artifact version
- **AND** system verifies extension metadata references the compatible `kml` version
- **AND** system reports a clear skip reason when extension publish is intentionally manual or deferred
