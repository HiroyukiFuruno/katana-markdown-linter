## ADDED Requirements

### Requirement: v0.19.6 dependency refresh evidence

The v0.19.6 patch release SHALL refresh GitHub Actions and Rust dependencies against upstream stable versions, synchronize release metadata, and pass all existing release gates without reducing validation thresholds.

#### Scenario: Dependency refresh is ready for publication

- **WHEN** v0.19.6 is prepared for publication
- **THEN** workflow action revisions match verified upstream commits, root and Zed Cargo lockfiles contain the evaluated updates, and the existing release-check succeeds
- **AND** the release follows Draft review, findings resolution, Ready promotion, required checks, merge, canonical publication, and release verification
