## Context

The repository already has `make bench` and `make perf-check`, but the report
stores only one total and one average per case. A single sample is too sensitive
to CPU load and cache state, which makes small optimization work hard to judge.

The current runner focuses on API lint and fix cases. That is useful, but it
does not cover clean documents, CLI directory checks, or config validation.
Those paths matter because real users run `kml` on workspaces, not only on one
in-memory document.

## Goals / Non-Goals

### Goals

This design makes performance reports less sensitive to single-run noise. It
adds benchmark cases that represent clean documents, CLI directory checks, and
config validation. It keeps the performance gate report-first for timing deltas
and keeps public library and CLI behavior unchanged.

### Non-Goals

This change does not add a strict timing threshold in CI. It does not rewrite
the rule engine, and it does not add an external benchmark framework dependency.

## Decisions

### Decision: use built-in repeated sampling before adding a framework

The runner will keep using `std::time::Instant` and add warmup plus repeated
samples. This keeps the tool light and preserves the current JSON contract.

Alternative considered: add Criterion. Criterion is strong for isolated Rust
microbenchmarks, but this project needs a committed JSON report consumed by
`make perf-check`. A custom runner is enough for this phase.

### Decision: compare median first

Reports will include mean, median, min, max, and standard deviation. The summary
will compare median values because median is less sensitive to short spikes.

Alternative considered: compare mean only. Mean is still recorded, but using it
as the headline value makes small regressions harder to interpret.

### Decision: add CLI and config cases without changing CLI behavior

CLI benchmark cases will create a temporary synthetic workspace, run the public
`kml` binary path through the CLI module, and measure end-to-end check cost.
Config cases will load and validate representative config content repeatedly.

This broadens coverage while keeping behavior changes out of scope.

## Risks / Trade-offs

Repeated sampling increases `make bench` runtime. The mitigation is a modest
default sample count plus command-line overrides.

CLI directory benchmark cases can include filesystem noise. The mitigation is a
small deterministic workspace and median comparison instead of one sample.

The new report schema can break old baselines. The mitigation is to update the
committed baseline and make `perf-check` fail with explicit schema messages.

## Migration Plan

Update the benchmark runner and refresh the committed baseline in the same
change. Keep `make perf-check` timing deltas informational. After this change,
future optimization work can use median ratios as the main local comparison
signal.

## Open Questions

None.
