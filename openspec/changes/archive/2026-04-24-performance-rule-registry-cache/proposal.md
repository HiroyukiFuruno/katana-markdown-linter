## Why

Rule lookup currently rebuilds rule metadata and catalog data in hot paths such
as CLI config validation, rule catalog APIs, and benchmark cases. That creates
avoidable allocation and setup overhead, especially for many small files.

## What Changes

The rule registry will keep cached official and user-configurable rule metadata
for internal metadata paths. Public compatibility helpers that return owned rule
vectors will remain available. Lint execution will preserve the fastest measured
dispatch path rather than forcing a cache shape that regresses runtime.

The rule catalog will also be cached and cloned from a shared source of truth.
Config validation will gain a cached-rule path so CLI file loops do not rebuild
rule metadata for every file. The performance baseline will be refreshed after
the implementation.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

`performance-benchmarking` is modified to require cached rule metadata use for
metadata hot paths while preserving public behavior.

## Impact

Affected files are `src/rules/markdown/eval.rs`, `src/rules/markdown/mod.rs`,
`src/rules/markdown/config.rs`, `src/catalog.rs`, `src/lib.rs`, `src/cli.rs`,
`examples/perf_benchmark.rs`, `tests/fixtures/perf-baseline.json`, and
`openspec/specs/performance-benchmarking/spec.md`.
