## Why

`v0.4.0` intentionally left `MD060` as diagnostic/manual-required because table column style is a table-block concern, not a single-line replacement.
The current `MD060` check is also too shallow: it flags any row containing a spaced pipe separator instead of recognizing the official `aligned`, `compact`, and `tight` table styles.

For `v0.5.0`, kml should improve table style correctness and add a default-safe fix subset without changing the future `--unsafe` confirmation plan.

## What Changes

- Add a table-block parser/formatter for `MD060`.
- Evaluate `MD060.style` values `any`, `aligned`, `compact`, and `tight` with `MD060.aligned_delimiter`.
- Accept official aligned, compact, and tight examples instead of treating compact tables as violations.
- Add safe table-block fixes for parseable, consistent-column tables.
- Skip safe fixes for ambiguous table content such as inconsistent column counts or unsafe pipe parsing.
- Change README rule coverage from `yes/no` to state-based `check`, `safe fix`, and `unsafe fix` columns.

## Impact

- `MD060` check precision improves.
- `kml fix` can normalize simple table-column-style violations while preserving default safe behavior.
- `MD055` / `MD056` / `MD058` remain separate rules; `MD060` does not guess through column-count errors.
- Unsafe table rewrites remain deferred to `unsafe-fix-mode-and-confirmation`.
