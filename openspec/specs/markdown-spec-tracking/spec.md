# Markdown specification tracking

## Purpose

Keep shared Markdown semantics, safe fix convergence, and release verification traceable to authoritative specifications and reviewed regression data.

## Requirements

### Requirement: Versioned Markdown semantics

kml SHALL record the CommonMark and GFM versions used by its syntax regressions and distinguish GitHub platform extensions from those specifications.

#### Scenario: Source attribution

- **WHEN** a user reads the syntax support documentation
- **THEN** the user sees authoritative source URLs, checked versions, tested semantics, and limitations without an unsupported full-conformance claim

### Requirement: Shared syntax accuracy

The shared document context SHALL preserve escaped table pipes, accept tab-separated ATX headings, distinguish valid fenced-code boundaries, and protect multiline inline-code content within a block.

#### Scenario: Escaped table separators

- **WHEN** a table cell contains an escaped pipe
- **THEN** that pipe remains within the cell and source byte ranges remain valid

#### Scenario: ATX whitespace

- **WHEN** an ATX opening sequence is followed by a tab
- **THEN** it is recognized as a heading

#### Scenario: Fence boundaries

- **WHEN** a fence-like code-content line has a nonblank suffix or a top-level opener has four spaces of indentation
- **THEN** it does not incorrectly terminate or open a fenced code block

#### Scenario: Multiline inline code

- **WHEN** matching backtick runs surround multiple lines within one paragraph
- **THEN** apparent links and HTML inside them remain code, and parsing does not cross separate block boundaries

### Requirement: Minor release evidence

The change SHALL retain existing quality gates and follow Draft review, findings resolution, Ready, CI, merge, publication verification, and scoped local cleanup for v0.20.0.

#### Scenario: Release completion

- **WHEN** the change is reported complete
- **THEN** its automated verification and public distribution evidence exist, and required review findings are resolved

### Requirement: Shared deterministic convergent fixing

Library fixing, CLI fixing, and formatting SHALL share one fix orchestration engine. The engine SHALL apply syntax repairs before content/style changes and then layout changes, recheck after every applied snapshot, and reject cycles or remaining effective fixes at the configured pass limit. Identical insertions at the same source position MUST NOT accumulate whitespace.

#### Scenario: Contradictory layout configuration

- **WHEN** an enabled zero-blank MD012 policy conflicts with enabled MD022 heading spacing
- **THEN** fixing fails with a convergence error and leaves that input file unchanged

#### Scenario: Stable reviewed document answers

- **WHEN** each reviewed document input is fixed three times
- **THEN** all three results equal its independently specified answer byte for byte, no additional changes occur, and no applicable safe fix remains

#### Scenario: Independent rule evaluation

- **WHEN** multiple rules inspect one document snapshot
- **THEN** they reuse its shared source context and avoid independent backtick state machines

#### Scenario: Literal whitespace and line endings

- **WHEN** code consists only of whitespace or contains Unicode and CRLF source lines
- **THEN** safe fixes preserve literal content and internal line endings without creating malformed empty code spans
