## ADDED Requirements

### Requirement: v0.19.6 dependency refresh evidence

The v0.19.6 patch release SHALL refresh GitHub Actions and Rust dependencies against upstream stable versions, synchronize release metadata, and pass all existing release gates without reducing validation thresholds.

#### Scenario: Dependency refresh is ready for publication

- **WHEN** v0.19.6 is prepared for publication
- **THEN** workflow action revisions match verified upstream commits, root and Zed Cargo lockfiles contain the evaluated updates, and the existing release-check succeeds
- **AND** the release follows Draft review, findings resolution, Ready promotion, required checks, merge, canonical publication, and release verification

#### Scenario: Live upstream gate detects the MD043 headings mismatch

- **WHEN** the current upstream specifies nullable headings with a null default
- **THEN** KML accepts omitted/null headings as no restriction and treats an explicit empty array as forbidding headings
- **AND** config validation, generated schema, metadata and runtime agree, other array properties retain their null rejection, and the existing strict upstream drift gate passes
