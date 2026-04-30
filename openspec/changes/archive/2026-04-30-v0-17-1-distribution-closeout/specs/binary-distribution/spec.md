## MODIFIED Requirements

### Requirement: npm and PyPI wrappers SHALL be official only after registry verification

npm / PyPI wrapper は、registry 公開状態と wrapper 起動検証が揃った場合だけ公式 install channel として扱わなければならない（SHALL）。

#### Scenario: wrapper channel is documented as official

- **WHEN** repository docs list npm or PyPI as an official install channel
- **THEN** system verifies the registry package version matches the release version
- **AND** system verifies the wrapper launches `kml --version` for that release
- **AND** system does not describe unpublished or unverified wrapper paths as official

### Requirement: Homebrew formula SHALL be updated from verified release artifacts

Homebrew formula は、GitHub Release の verified binary archive と checksum から生成された内容を tap に反映しなければならない（SHALL）。

#### Scenario: tap formula is prepared

- **WHEN** developer prepares the Homebrew tap update for `vX.Y.Z`
- **THEN** system uses the generated formula output for that release
- **AND** formula URLs point to GitHub Release binary archives for `vX.Y.Z`
- **AND** formula checksum values match the release checksum files
- **AND** tap mutation remains reviewable as a separate repository diff
