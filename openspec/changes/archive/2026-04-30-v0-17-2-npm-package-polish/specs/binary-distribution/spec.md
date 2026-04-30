## MODIFIED Requirements

### Requirement: npm wrapper package SHALL include registry-visible usage documentation

npm wrapper package は、npm registry page 上で導入方法と thin wrapper の責務を説明できなければならない（SHALL）。

#### Scenario: npm package is packed

- **WHEN** system builds the npm package tarball for `vX.Y.Z`
- **THEN** tarball contains `README.md`
- **AND** README includes global install and `npx` examples for `katana-markdown-linter`
- **AND** README states that the npm package is a thin launcher over GitHub Release binary archives
- **AND** README lists supported platforms or points to the supported platform contract
- **AND** README does not imply npm contains independent lint logic

### Requirement: npm wrapper package SHALL keep dependency surface minimal

npm wrapper package は、thin wrapper に不要な runtime dependency を追加してはならない（SHALL NOT）。

#### Scenario: package metadata is inspected

- **WHEN** developer reviews `wrappers/npm/package.json`
- **THEN** package keeps runtime dependencies empty unless a specific dependency is justified by wrapper behavior
- **AND** package metadata includes search and support fields such as `keywords`, `homepage`, and `bugs`
- **AND** package keeps `bin.kml` pointing to the launcher script
