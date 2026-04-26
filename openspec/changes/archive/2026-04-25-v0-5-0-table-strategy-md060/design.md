## Design

`MD060` becomes a table-block rule.
The rule scans Markdown lines outside fenced code blocks, identifies GitHub Flavored Markdown table blocks using a header row plus delimiter row, and evaluates the whole block as one unit.

## Rule Behavior

Supported official states:

- `style=any`: pass when the table matches `aligned`, `compact`, or `tight`; otherwise choose the closest style for a safe fix when possible.
- `style=aligned`: pipe positions must align across rows.
- `style=compact`: each pipe separator has exactly one surrounding space.
- `style=tight`: pipe separators have no surrounding spaces.
- `aligned_delimiter=true`: delimiter row pipe positions must align with the header row.

## Safe Fix Policy

Safe fix is allowed only when:

- the table block has a valid delimiter row
- every row has the same column count
- every row uses the same leading/trailing pipe shape
- cells can be parsed without escaped pipes or inline code pipes
- the formatter can rewrite the entire table block as one non-overlapping replacement

Safe fix is skipped when:

- column count is inconsistent, leaving `MD056` responsible
- escaped pipes or code spans make cell splitting ambiguous
- table shape changes would require unsafe interpretation

## README State Model

README rule map uses state columns instead of booleans:

- `Check`: `済` when the rule is implemented in the active public catalog.
- `Fix (safe)`: `一部対応` when at least one fixture-backed safe fix exists, otherwise `未対応`.
- `Fix (unsafe)`: `未対応` when unsafe behavior is part of an active implementation track, otherwise `対応予定なし`.
- Use `対応不可能` only when behavior cannot be implemented without violating the markdownlint contract or the safe-fix model.

For this change, `MD060` moves from `Fix (safe): 未対応` to
`Fix (safe): 一部対応`. Its unsafe state is `未対応` until the separate unsafe
fix mode defines opt-in execution semantics. Other rules remain
`Fix (unsafe): 対応予定なし`.

## Non-Goals

- Implement `--unsafe`.
- Reformat tables with inconsistent column counts.
- Guarantee perfect GFM parsing for escaped pipes and nested inline constructs.
- Change official markdownlint `fixable` metadata.
