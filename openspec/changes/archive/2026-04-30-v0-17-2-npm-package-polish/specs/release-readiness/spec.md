## MODIFIED Requirements

### Requirement: v0.17.2 release readiness SHALL close the npm package visibility gap

`v0.17.2` の release readiness は、npm package page の README / metadata 不足を release blocker として扱わなければならない（SHALL）。

#### Scenario: v0.17.2 release is prepared

- **WHEN** developer prepares `v0.17.2`
- **THEN** system confirms `wrappers/npm/README.md` exists and is included in the npm tarball
- **AND** system confirms npm package metadata has search and support fields
- **AND** system confirms trusted publisher configuration is present for `HiroyukiFuruno/katana-markdown-linter` and `release.yml`
- **AND** system keeps the npm package as a thin wrapper with no independent lint logic

### Requirement: v0.17.2 post-release verification SHALL prove npm publication

`v0.17.2` の公開後検証は、npm registry と npm wrapper 起動を確認しなければならない（SHALL）。

#### Scenario: v0.17.2 npm publication is verified

- **WHEN** npm wrapper publication for `v0.17.2` completes
- **THEN** system verifies npm contains `katana-markdown-linter` version `0.17.2`
- **AND** system runs `npx --yes katana-markdown-linter@0.17.2 --version`
- **AND** command output is `0.17.2`
- **AND** verification result is recorded before `v0.18.0` work resumes
