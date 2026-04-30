## MODIFIED Requirements

### Requirement: CLI config schema output SHALL match published schema

CLI は、published schema と一致する config schema を出力しなければならない（SHALL）。

#### Scenario: schema command prints JSON schema

- **WHEN** user runs `kml config schema --output json`
- **THEN** CLI prints a valid JSON Schema document
- **AND** `$id` matches the stable schema URL
- **AND** output is semantically equal to `schema/markdownlint.schema.json`

### Requirement: CLI config schema output SHALL remain editor-consumable

CLI の config schema output は、editor の JSON schema integration でそのまま使える構造でなければならない（SHALL）。

#### Scenario: editor consumes generated schema

- **WHEN** user saves `kml config schema --output json` to a local file
- **THEN** JSON Schema validators accept the document
- **AND** schema includes rule descriptions, defaults, and supported property shapes
- **AND** schema rejects unknown top-level rule keys through `additionalProperties: false`
