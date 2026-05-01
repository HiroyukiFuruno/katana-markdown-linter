## ADDED Requirements

### Requirement: v0.18.7 release readiness SHALL document unsafe fix opt-in for check --fix

`v0.18.7` の release readiness は、`check --fix` でも unsafe fix の明示 opt-in が使えることを CLI help と公開 README で説明しなければならない（SHALL）。

#### Scenario: command help documents unsafe fix opt-in

- **WHEN** developer prepares `v0.18.7`
- **THEN** system runs `kml check --help`
- **AND** command exits with code `0`
- **AND** command help includes `--unsafe --yes`
- **AND** command help explains that unsafe fixes are allowed only when used with `--fix`

#### Scenario: check fix applies unsafe fixes with explicit approval

- **WHEN** developer prepares `v0.18.7`
- **THEN** system runs `kml check --fix --unsafe --yes` against an unsafe-fixable `MD036` fixture
- **AND** command exits with code `0`
- **AND** command applies the unsafe fix
- **AND** JSON output keeps `command` as `check`
- **AND** JSON output records the applied `MD036` fix detail

#### Scenario: public CLI usage documents both fix entrypoints

- **WHEN** developer prepares `v0.18.7`
- **THEN** README includes `kml fix --unsafe --yes`
- **AND** README includes `kml check --fix --unsafe --yes`
- **AND** README explains that unsafe fixes require explicit opt-in for both `fix` and `check --fix`
