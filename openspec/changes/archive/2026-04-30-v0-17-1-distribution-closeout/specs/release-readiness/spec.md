## ADDED Requirements

### Requirement: release verification SHALL include public package registries

公開後検証（post-release verification）は、GitHub Release と crates.io だけでなく、npm と PyPI の公開状態を確認しなければならない（SHALL）。

#### Scenario: release verification checks registry versions

- **WHEN** developer runs `make release-verify VERSION=vX.Y.Z`
- **THEN** system verifies crates.io contains `katana-markdown-linter` version `X.Y.Z`
- **AND** system verifies npm contains `katana-markdown-linter` version `X.Y.Z`
- **AND** system verifies PyPI contains `katana-markdown-linter` version `X.Y.Z`
- **AND** system fails with a registry-specific error when a version is missing

### Requirement: release verification SHALL execute wrapper launch smoke tests

公開後検証は、公開済み wrapper から `kml` が起動することを確認しなければならない（SHALL）。

#### Scenario: release verification launches wrappers

- **WHEN** developer runs `make release-verify VERSION=vX.Y.Z`
- **THEN** system runs the npm wrapper through `npx --yes katana-markdown-linter@X.Y.Z --version`
- **AND** system runs the PyPI wrapper through `uvx --from katana-markdown-linter==X.Y.Z kml --version`
- **AND** both commands must print `X.Y.Z`

### Requirement: release verification SHALL include Homebrew formula evidence

公開後検証は、Homebrew formula が release artifact と一致していることを確認しなければならない（SHALL）。

#### Scenario: release verification checks formula output

- **WHEN** developer runs `make release-verify VERSION=vX.Y.Z`
- **THEN** system renders or reads the Homebrew formula for `vX.Y.Z`
- **AND** system verifies formula URL values reference the expected release archives
- **AND** system verifies formula checksum values match generated checksum files
- **AND** system verifies formula test block executes `kml --version`
