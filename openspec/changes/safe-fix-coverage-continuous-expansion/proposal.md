## Why

kml should become stronger as a markdownlint-compatible linter before it grows
formatter semantics. The README Rule Map now makes diagnostic-only safe fix
gaps visible, so the next quality release should convert safe, low-risk gaps
into fixture-locked corrections and tighten check parity when behavior is uncertain.

Unsafe fix mode and formatter productization are intentionally later. This
change keeps the default contract conservative: if a rewrite can change
meaning, structure, or rendered output in a non-obvious way, it is not a safe
fix.

## What Changes

- Rank currently diagnostic-only rules by safe-fix feasibility.
- Add fixture matrix entries for new safe fixes and edge cases.
- Implement safe corrections only when the output is deterministic and markdownlint semantics are clear.
- Expand upstream golden coverage for changed rules.
- Update README Rule Map and coverage dashboard from fixtures.

## Scope

In scope:

- Safe fixes for rules with straightforward local rewrites.
- Check precision fixes discovered while implementing those rewrites.
- Fixture matrix, golden comparison, dogfood, and dashboard updates.

Out of scope:

- Unsafe fix mode and confirmation prompts.
- Formatter-wide rewrites such as general paragraph reflow.
- New markdown flavors or non-markdownlint rules.

## Impact

- Additional violations can be corrected by default without sacrificing safety.
- Diagnostic-only rules become explicitly justified when no safe rewrite exists.
- Later unsafe/formatter changes start from a stronger linter baseline.
