## MODIFIED Requirements

### Requirement: System SHALL provide original composite Markdown samples

System SHALL provide at least 50 original Markdown samples for historical bug pattern combinations.

#### Scenario: original corpus is prepared

- **WHEN** original samples are created
- **THEN** system creates at least 50 original Markdown input files
- **AND** each original input is at least 200 characters long
- **AND** each original input is a meaningful Markdown document, not a simple repeated character or random string sequence
- **AND** each original input combines multiple historical bug patterns in one document

## ADDED Requirements

### Requirement: Repeated fixes SHALL preserve reviewed answers

System SHALL fix every evaluated input three consecutive times, compare every pass byte for byte with the independently reviewed answer, and reject remaining applicable safe fixes.

#### Scenario: Three consecutive fix passes

- **WHEN** the document answer harness evaluates an input
- **THEN** all three fix outputs match its reviewed answer
- **AND** the second and third passes introduce no further changes
- **AND** applicable safe fixes remaining in fixed outputs or answer fixtures fail evaluation
