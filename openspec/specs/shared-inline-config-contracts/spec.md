## Purpose

Keep Markdown source interpretation, rule configuration, formatter policy and safe fix application consistent across supported entrypoints.

## Requirements

### Requirement: Shared escape-aware inline source interpretation

MD011, MD049 and MD050 SHALL consume shared source adapters for escape-aware candidate boundaries. Escaped punctuation MUST remain literal, source byte ranges MUST remain valid for Unicode/CRLF, and code protection MUST remain consistent.

#### Scenario: Escaped reversed-link example

- **WHEN** an input contains escaped parentheses followed by a bracketed URL
- **THEN** safe fixing leaves the literal example unchanged without an MD011 false positive

#### Scenario: Escaped emphasis marker

- **WHEN** a literal escaped asterisk precedes valid underscore emphasis
- **THEN** it does not determine the inferred emphasis style or trigger an MD049 false positive

### Requirement: Consistent common rule configuration

Public schema, validation and option conversion SHALL share common rule-property definitions and accept the same boolean enabled property with the same type constraints.

#### Scenario: Explicit rule enable under default false

- **WHEN** a rule object sets enabled true and valid official properties under default false
- **THEN** schema and runtime validation accept it and the specified rule is evaluated

### Requirement: Shared formatter configuration application

CLI file formatting, CLI stdin formatting and editor formatting SHALL consume the shared layout policy with resolved rule enable flags and properties. Non-layout rule policies MUST remain excluded.

#### Scenario: Configured four-space list indentation

- **WHEN** a document satisfies MD007 indent four and is formatted using that effective configuration
- **THEN** formatting preserves its four-space indentation and a subsequent check still passes the same layout configuration

#### Scenario: Disabled layout rule

- **WHEN** a layout rule is explicitly disabled
- **THEN** CLI and editor formatting do not apply that rule

### Requirement: Shared emphasis semantics safety

Emphasis and strong-emphasis rules SHALL share CommonMark flanking semantics and use authoritative backend source ranges for complex nested or crossing candidates. A style conversion MUST NOT be offered as safe when it changes parsed structure or literal content.

#### Scenario: Intraword underscore literal

- **WHEN** a valid asterisk emphasis precedes ASCII or Japanese intraword underscore text
- **THEN** the underscore text remains literal and does not trigger a style fix

#### Scenario: Nested triple delimiter

- **WHEN** MD050 changes strong style in triple-asterisk emphasis
- **THEN** its source ranges preserve the outer emphasis and produce the equivalent nested structure

#### Scenario: Unsafe style conversion

- **WHEN** changing a style marker would alter grouping or literal content
- **THEN** kml keeps the diagnostic and original text without offering a safe fix for that conversion

### Requirement: Structured formatter convergence failure

Formatter convergence errors SHALL use the existing structured rule-error report and exit code two. A failed file MUST remain unchanged and other files in the batch SHALL continue.

#### Scenario: Contradictory layout settings

- **WHEN** enabled MD012 maximum zero and MD022 cause a formatter cycle
- **THEN** file and stdin JSON modes report a rule error, preserve the input and return exit code two

### Requirement: Per-operation configuration preparation

CLI check, fix and formatting SHALL share source resolution, parsing, validation and option preparation within each operation. Prepared settings MUST NOT persist across operations; nearest-directory and explicit-source precedence and existing error behavior SHALL remain unchanged.

#### Scenario: Repeated files use the same configuration

- **WHEN** files resolve to the same configuration source in one operation
- **THEN** they consume the same prepared settings without repeated source parsing

#### Scenario: Configuration changes before the next operation

- **WHEN** the configuration changes between consecutive workflow invocations in one process
- **THEN** the next invocation reads and applies the new configuration

### Requirement: Combined style fix semantics

The shared fix application boundary SHALL validate the meaning of the combined non-overlapping safe style edits. It MUST prevent new reference bindings produced only by multiple edits, including edits from different style rules. Intentional syntax repairs SHALL remain available. Single-pass and iterative APIs SHALL use the same plan policy.

#### Scenario: Individually safe edits bind a reference when combined

- **WHEN** individually safe style replacements together would create a reference binding
- **THEN** the application plan retains only a deterministic meaning-preserving subset and reports applied fix details accurately

#### Scenario: Many independent style replacements

- **WHEN** a paragraph contains 128 independent style replacements
- **THEN** the shared plan applies them together without exhausting the existing pass budget

#### Scenario: Style edits interact with a content repair

- **WHEN** a content repair and a style edit would together create a reference binding
- **THEN** the content repair remains available and the style edit preserves the meaning of that repaired baseline
