## ADDED Requirements

### Requirement: Shared literal regions SHALL protect Markdown edits

Markdown scanners and layout formatting SHALL use shared parser-derived
literal boundaries for raw HTML and fenced code. Terminal normalization MUST
preserve newline counts within literal content.

#### Scenario: Raw HTML attribute and adjacent Markdown

- **WHEN** raw HTML attributes and adjacent Markdown contain similar fix candidates
- **THEN** safe fixing preserves the attributes and repairs the Markdown

#### Scenario: Unterminated literal block at the document end

- **WHEN** a fenced code or raw HTML block reaches the document end
- **THEN** terminal normalization preserves literal content and its newline count

#### Scenario: A literal block follows a container label on the same line

- **WHEN** a footnote or definition list opens fenced code or raw HTML after its marker
- **THEN** shared syntax analysis recognizes the block and preserves its literal contents

### Requirement: Strict entrypoints SHALL share configuration preparation

CLI, editor and MCP SHALL share semantic configuration validation and option
preparation. Strict entrypoints MUST reject invalid configuration before
proposing edits.

#### Scenario: Invalid rule option

- **WHEN** the editor resolves a valid JSON config with an invalid property value
- **THEN** it reports a configuration error and offers no formatting or quick fix
