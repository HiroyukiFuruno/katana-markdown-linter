## ADDED Requirements

### Requirement: Editor edits SHALL use protocol positions and full document context

The LSP adapter SHALL express diagnostics and edits in UTF-16 coordinates.
Range formatting SHALL evaluate layout fixes in the full document and preserve
all bytes outside the selected lines.

#### Scenario: Unicode precedes a fix

- **WHEN** Japanese text or a supplementary character precedes a fixable link
- **THEN** applying the returned edit repairs the link and preserves surrounding text

#### Scenario: Selected lines belong to a code block

- **WHEN** the editor selects lines inside a fenced block
- **THEN** range formatting retains literal blank lines and unselected bytes
