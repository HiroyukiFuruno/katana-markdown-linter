## ADDED Requirements

### Requirement: MD046 emits per-block diagnostics with safe-fix

MD046 SHALL emit one diagnostic per indented code block group (consecutive `is_indented_code_line` lines, split at blank lines) when the document also contains at least one fenced code block.
Each diagnostic SHALL carry a `fix_info` that replaces the indented block with a fenced block: strip 4 leading spaces from every line and wrap with triple-backtick fences fences.
MD046 SHALL be listed in `is_safe_fix_rule` so that `kml fix --safe` applies the conversion.

#### Scenario: Single indented block alongside fenced block gets fix

- **WHEN** content contains one fenced code block and one indented code block (4-space-indented, not inside a list or existing code block)
- **THEN** MD046 emits exactly one diagnostic at the first line of the indented block
- **THEN** `fix_info` replacement produces a fenced block with 4 leading spaces stripped and triple-backtick fences added

#### Scenario: Multiple indented blocks each get a separate diagnostic and fix

- **WHEN** content contains one fenced code block and two separate indented code blocks (separated by a blank line)
- **THEN** MD046 emits two diagnostics, one per indented block
- **THEN** each `fix_info` covers only that block's line range

#### Scenario: Indented block inside list is not flagged

- **WHEN** an indented line is a continuation of a list item (list marker at 0-indent, 4-space-indented continuation)
- **THEN** MD046 does not emit a diagnostic for that line

#### Scenario: Pure indented-only document emits no diagnostic

- **WHEN** content contains only indented code blocks and no fenced blocks
- **THEN** MD046 emits no diagnostic (no style conflict)

#### Scenario: kml fix --safe converts indented blocks to fenced

- **WHEN** `kml fix --safe` is applied to a file with MD046 diagnostics
- **THEN** all indented code blocks are replaced with equivalent fenced blocks
- **THEN** re-running lint on the fixed file emits no MD046 diagnostic
