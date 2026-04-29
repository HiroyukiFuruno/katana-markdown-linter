## Purpose

Document answer fix evaluation verifies that `kml check --fix` produces exactly
the reviewed Markdown output for full documents, not only isolated rule
fixtures.

## Requirements

### Requirement: System SHALL collect a public GitHub Markdown corpus

System SHALL collect at least 200 Markdown samples from public GitHub repositories for document-level fix evaluation.

#### Scenario: public corpus is prepared

- **WHEN** the evaluation corpus is prepared for `v0.16.2`
- **THEN** system records at least 200 public GitHub Markdown samples
- **AND** each sample records source repository, commit SHA, path, license, retrieval date, and selection reason
- **AND** samples without acceptable license evidence are not committed as fixture content

### Requirement: System SHALL provide original composite Markdown samples

System SHALL provide 50 original Markdown samples for historical bug pattern combinations.

#### Scenario: original corpus is prepared

- **WHEN** original samples are created
- **THEN** system creates exactly 50 original Markdown input files
- **AND** each original input is at least 200 characters long
- **AND** each original input is a meaningful Markdown document, not a simple repeated character or random string sequence
- **AND** each original input combines multiple historical bug patterns in one document

### Requirement: Original samples SHALL combine historical bug patterns

Original samples SHALL combine previously detected or previously fixed bug patterns instead of representing only isolated single-rule cases.

#### Scenario: historical patterns are combined

- **WHEN** an original sample is reviewed
- **THEN** the sample includes two or more historical bug pattern categories
- **AND** the categories include combinations from lists, tables, links, code blocks, inline code, Unicode anchors, references, or line ending boundaries
- **AND** the sample manifest records which historical bug patterns are covered

### Requirement: Every evaluated input SHALL have an answer fixture

Every evaluated Markdown input SHALL have a corresponding `xxx_answer.md` file.

#### Scenario: answer fixture exists

- **WHEN** system evaluates `xxx.md`
- **THEN** system locates `xxx_answer.md` in the same answer fixture mapping
- **AND** missing answer fixtures fail the evaluation

### Requirement: Fix output SHALL match answer fixtures exactly

System SHALL compare fixed Markdown output with answer fixtures byte-for-byte.

#### Scenario: fixed output is compared

- **WHEN** system runs `kml check --fix` against an evaluated input using the default rule set with `MD013` disabled
- **THEN** system compares the fixed output with the corresponding answer fixture byte-for-byte
- **AND** whitespace, newline, or encoding differences are treated as failures
- **AND** any mismatch is reported as a bug candidate with input path and diff evidence

### Requirement: Answer fixtures SHALL NOT be generated from current implementation output alone

Answer fixtures SHALL represent reviewed expected Markdown output, not merely the current implementation output.

#### Scenario: answer fixture is reviewed

- **WHEN** a new answer fixture is added
- **THEN** system records that the expected output was reviewed against the intended Markdown structure
- **AND** the fixture is not accepted solely because the current `kml check --fix` output produced it

### Requirement: Answer fixtures SHALL validate as stable fixed documents

System SHALL validate answer fixtures independently from the input-to-answer comparison.

#### Scenario: answer fixture is independently validated

- **WHEN** system evaluates an answer fixture
- **THEN** system runs `kml check` against the answer fixture with the same evaluation config
- **AND** remaining diagnostics are reported in the evaluation summary
- **AND** system runs `kml check --fix` against a copy of the answer fixture
- **AND** any second-pass fix changes fail the evaluation
