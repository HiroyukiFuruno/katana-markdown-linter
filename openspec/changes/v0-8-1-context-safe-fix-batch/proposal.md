# v0.8.1 Context-Safe Fix Batch

## Summary

Add the next small safe-fix batch after `v0.8.0` by converting `MD048`
(`code-fence-style`) from a document-level mixed-style check into a
`DocumentContext`-backed block check with conservative safe fixes.

## Motivation

`MD048` currently detects only the presence of both backtick and tilde fences
and reports a single diagnostic at line 1. That is weaker than the official
markdownlint behavior because the configured `style` is not honored and
diagnostics are not tied to the offending fenced block.

`DocumentContext` already exposes fenced code block ranges and fence kinds, so
`MD048` can be improved without introducing formatter semantics or unsafe fix
policy changes.

## Scope

- Implement `MD048` check using parsed fenced code blocks.
- Honor `MD048.style` values `consistent`, `backtick`, and `tilde`.
- Add safe fixes that rewrite only the opening and closing fence markers when
  the conversion cannot collide with fence markers inside the code block.

- Keep default `fix` behavior safe-only.
- Update fixture, golden, dashboard, README, changelog, and version metadata for
  `v0.8.1`.

## Out of Scope

- `MD046` indented/fenced conversion.
- `MD003` setext/ATX conversion.
- Unsafe fix mode and confirmation behavior.
- Formatter semantics.

## Release Target

`v0.8.1`
