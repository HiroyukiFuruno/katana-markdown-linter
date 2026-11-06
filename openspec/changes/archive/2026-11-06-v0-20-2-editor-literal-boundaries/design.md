## Context

The audit reproduced incorrect UTF-16 ranges, raw HTML attribute rewrites,
loss of selection context, literal terminal newline changes and editor
fallback after semantic configuration errors.

## Decisions

- Keep core edits in byte coordinates. Convert only at the LSP boundary.
- Apply layout fixes against the whole document on every pass. Anchor the
  selection to untouched prefix and suffix bytes.
- Use one parser for source boundaries. Candidate detection only determines
  whether to parse. It cannot declare a protected source range.
- Use literal block ranges in structural scanners and terminal normalization.
- Keep strict preparation shared and preserve the CLI's explicit error policy.
- Verify the workflow using environment values for release inputs and
  retain the existing version resolver and publication gates.
