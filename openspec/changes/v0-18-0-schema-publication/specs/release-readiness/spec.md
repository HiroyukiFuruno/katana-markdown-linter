## ADDED Requirements

### Requirement: release readiness SHALL include schema publication checks

release readiness は、schema publication の検証を含まなければならない（SHALL）。

#### Scenario: release check validates schema publication

- **WHEN** developer runs `make release-check VERSION=vX.Y.Z`
- **THEN** system verifies canonical schema file and CLI schema output match
- **AND** system verifies schema regression fixtures are current
- **AND** system verifies release workflow includes the versioned schema artifact step

### Requirement: release notes SHALL describe schema compatibility when schema changes

schema が変更される release note は、互換性の扱いを説明しなければならない（SHALL）。

#### Scenario: release includes schema changes

- **WHEN** schema output differs from the previous release
- **THEN** release metadata records whether the change is additive or breaking
- **AND** breaking schema changes include migration notes
- **AND** additive schema changes can be released without migration notes
