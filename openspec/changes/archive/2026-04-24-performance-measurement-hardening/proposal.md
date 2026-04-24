## Why

Current performance reports record a single average per case, so local noise can
look like a regression or hide a real improvement. Before deeper optimization,
the project needs repeatable measurements, richer case coverage, and clearer
baseline comparison.

## What Changes

The benchmark runner will gain warmup and repeated samples.
`target/perf-report.json` will include sample statistics such as median, min,
max, and standard deviation.

The benchmark case set will add clean large documents, CLI directory checks,
and config validation. `make perf-check` will validate the report schema,
required cases, and required statistic fields. `docs/performance.md` will
describe the new report format and baseline policy.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

`performance-benchmarking` is modified to strengthen benchmark report
statistics, required case coverage, and baseline comparison rules.

## Impact

Affected files are `examples/perf_benchmark.rs`, `scripts/ci/perf-check.py`,
`tests/fixtures/perf-baseline.json`, `docs/performance.md`, and
`openspec/specs/performance-benchmarking/spec.md`.
