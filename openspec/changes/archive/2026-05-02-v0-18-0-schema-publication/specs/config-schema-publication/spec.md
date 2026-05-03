## ADDED Requirements

### Requirement: repository SHALL publish a canonical markdownlint config schema

repository は、`.markdownlint.json` と `.markdownlint.jsonc` 用の canonical schema file を保持しなければならない（SHALL）。

#### Scenario: canonical schema is checked

- **WHEN** developer runs the schema check
- **THEN** system reads `schema/markdownlint.schema.json`
- **AND** system compares it with `kml config schema --output json`
- **AND** system fails when the canonical file is missing or stale

### Requirement: schema publication SHALL provide stable and versioned references

schema publication は、stable URL と release-pinned artifact の両方を提供しなければならない（SHALL）。

#### Scenario: schema is published for a release

- **WHEN** release workflow publishes `vX.Y.Z`
- **THEN** system includes a versioned schema artifact for that release
- **AND** canonical schema `$id` remains the stable schema URL
- **AND** docs explain when to use stable URL and when to use pinned artifact URL

### Requirement: schema compatibility SHALL be regression-tested

schema compatibility は、意図しない破壊的変更を検出する regression test で守られなければならない（SHALL）。

#### Scenario: rule metadata changes

- **WHEN** rule metadata or config property metadata changes
- **THEN** system detects schema output changes during tests
- **AND** additive rule or property additions are allowed when fixtures are updated
- **AND** type, enum, or default value removals require an explicit OpenSpec decision

### Requirement: editor validation docs SHALL reference the published schema contract

editor validation docs は、published schema contract に沿って書かれなければならない（SHALL）。

#### Scenario: user configures editor schema validation

- **WHEN** user follows editor integration docs
- **THEN** docs show schema association for `.markdownlint.json`
- **AND** docs show schema association for `.markdownlint.jsonc`
- **AND** docs do not require installing an editor extension only to validate config files
