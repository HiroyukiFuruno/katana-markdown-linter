## Why

KML currently measures internal API and CLI hot paths, but it cannot show how
the released `kml` CLI compares with mature Rust Markdown linters such as
`mado` and `rumdl`. Without an external CLI benchmark, performance regressions
against peer tools are hard to see before they become user-visible.

## What Changes

- Add an opt-in cross-tool benchmark workflow for `kml`, `mado`, and `rumdl`.
- Measure each tool as an external CLI process against the same benchmark corpus.
- Report `check` performance for clean and diagnostics-heavy corpora.
- Report `fix` performance with isolated copied workspaces so tools can mutate files safely.
- Support both default-rule comparison and common-subset comparison.
- Generate a machine-readable report and a concise terminal summary.
- Keep the benchmark local/manual by default so CI does not depend on external tool installation or unstable wall-clock thresholds.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `performance-benchmarking`: add an external cross-tool CLI benchmark contract for `kml`, `mado`, and `rumdl`.

## Impact

Affected areas are benchmark scripts, Makefile targets, benchmark fixtures,
optional developer tooling documentation, and `performance-benchmarking` specs.
The `kml` CLI behavior, public Rust API, and release flow are not changed by
this proposal.
