## Design

### Prioritization Model

Rules are grouped by safe-fix feasibility:

- `safe-now`: local, deterministic rewrite with low rendering risk
- `safe-after-context`: needs `DocumentContext` structure or source ranges first
- `unsafe-candidate`: feasible only with explicit opt-in
- `manual-required`: needs user knowledge or content authoring judgment

The implementation loop starts with `safe-now`, then moves to
`safe-after-context` only when the required context already exists.

### Candidate Starting Set

The first pass should inspect diagnostic-only or partial rules from README:

- `MD001`, `MD003`, `MD013`, `MD024`, `MD028`
- `MD033`, `MD035`, `MD036`, `MD041`, `MD042`, `MD043`
- `MD045`, `MD046`, `MD048`, `MD052`, `MD055`, `MD056`, `MD059`

Not all of these should become safe fixes. The point is to record the reason
for each rule so later work is not guess-driven.

### Safety Policy

A safe fix must:

- be deterministic
- be idempotent
- preserve Markdown meaning and likely rendering
- be backed by pass/fail/fix/edge fixtures
- not require guessing author intent

If any condition is not met, the rule stays diagnostic-only or moves to unsafe
candidate planning.

### Verification

Every changed rule requires:

- unit tests around the rule
- fixture matrix update
- upstream golden comparison review
- README Rule Map update
- dogfood check

### Non-Goals

- `--unsafe`
- `kml fmt`
- table/paragraph reflow beyond rule-specific safe cases
- chasing rumdl rule count at the expense of markdownlint compatibility
