## ADDED Requirements

### Requirement: Unique manifest sample identity

The document answer harness SHALL validate unique sample IDs and unambiguous input/answer mappings before launching kml or creating sample copies. Invalid identity MUST fail closed rather than overwrite a sample.

#### Scenario: Duplicate sample ID

- **WHEN** distinct manifest records use the same sample ID
- **THEN** metadata validation fails before kml is launched and neither record silently replaces the other

### Requirement: Independent configuration and structure verification

The harness SHALL supplement the existing document corpus with independently specified expected outputs covering escape parity, Unicode, CRLF, nesting and selected configuration/entrypoint differences. Repeated fixing MUST match the expected answer, not only reach a stable output.

#### Scenario: Stable incorrect fix is rejected

- **WHEN** fixing reaches a byte-stable result that differs from the reviewed answer
- **THEN** evaluation fails even if repeated fixing reports no remaining applicable safe fix

#### Scenario: Formatter configuration matrix

- **WHEN** selected layout configurations are evaluated across supported file and stdin paths and shared library policy
- **THEN** the results match the independently specified output and applicable configured layout checks

#### Scenario: Style-only meaning preservation

- **WHEN** a safe fix only changes emphasis style in a reviewed valid input
- **THEN** the expected document structure and literal content are preserved by the independent verification oracle

### Requirement: Independent execution filenames

The harness SHALL allocate temporary execution filenames independently of logical sample IDs through one shared allocator used by both input and answer runners.

#### Scenario: Case-folding filesystem

- **WHEN** distinct logical IDs differ only in letter case
- **THEN** input and answer copies have distinct portable execution filenames and both samples retain their independent expected answers
