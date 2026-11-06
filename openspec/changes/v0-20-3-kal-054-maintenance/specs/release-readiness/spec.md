## ADDED Requirements

### Requirement: Standard AST gate SHALL cover source and function length boundaries

The repository SHALL exercise length boundary regressions through the same
AST lint runner and configuration used to check repository sources.

#### Scenario: Production follows an external test module declaration

- **WHEN** a source file declares an external test module before production
  code and the production code exceeds the source length limit
- **THEN** the standard gate reports the production file length violation

#### Scenario: Function bodies are at and above the length limit

- **WHEN** function bodies span the configured limit and one line more
- **THEN** the standard gate accepts the boundary case and reports the
  over-limit function at its declaration
