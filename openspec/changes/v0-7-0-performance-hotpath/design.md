## Design

### Performance Sequence

v0.7.0 focuses on hot paths that are already measured in this repository:

- `api_fix_large_document`
- `api_lint_large_document`
- `api_lint_clean_large_document`
- `cli_check_many_small_files`

The first implementation target is fix application. It is narrower than a
workspace cache and has fewer correctness risks.

### Fix Range Offset Index

Current fix application maps every `Fix` range from line/column to byte offsets
by scanning the full content. A diagnostics-heavy file can contain thousands of
fix candidates, so this becomes repeated linear work.

The optimized design builds a line-start index once per `fix::apply` call:

- collect byte offsets for every line start
- preserve virtual EOF behavior for line `N + 1`, column `1`
- clamp columns to previous UTF-8 char boundaries
- reject invalid ranges before overlap resolution

This keeps the existing range contract but turns repeated offset lookup into
per-line indexing plus small local boundary checks.

### Correctness Guardrails

The optimization must not change these behaviors:

- overlapping fixes keep the later sorted non-overlapping candidate behavior
- multi-line ranges apply to the same byte spans
- Unicode columns do not split UTF-8 code points
- virtual EOF inserts still work
- unchanged or no-fix content stays byte-identical

### rumdl Reference Notes

`rumdl` exposes several product and performance ideas worth tracking:

- release-profile benchmarks and dedicated benchmark binaries
- intelligent cache/watch mode for editor loops
- `fmt` stdout behavior that is friendly to editor integrations
- multi-channel distribution: Cargo, Homebrew, npm, pip, standalone binaries, pre-commit, GitHub Action, LSP
- configuration schema and markdownlint migration helpers

For v0.7.0, kml only adopts the measurable hot-path discipline. Cache/watch,
LSP/editor server, and distribution expansion are separate changes because they
change product contracts.

### Non-Goals

- Persistent file cache.
- Watch mode.
- Unsafe fixes.
- Rule behavior rewrites solely for benchmark numbers.
- Copying rumdl implementation details.
