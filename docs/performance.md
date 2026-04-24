# Performance

This project treats performance work as a report-first loop. The goal is to
make hot paths measurable on a local machine without turning CI into a flaky
wall-clock gate.

## Commands

Run the benchmark runner and write `target/perf-report.json`:

```sh
make bench
```

Compare the current report with the committed baseline:

```sh
make perf-check
```

Refresh the baseline after an intentional optimization:

```sh
make perf-refresh-baseline
```

## Report Schema

`target/perf-report.json` uses schema version `2`. Each case records repeated
samples and summary statistics:

- `sample_ms`: per-sample milliseconds per measured operation
- `mean_ms`: arithmetic mean of `sample_ms`
- `median_ms`: median of `sample_ms`
- `min_ms` and `max_ms`: sample bounds
- `stddev_ms`: population standard deviation
- `observed_items`: case-specific count returned by the measured operation

The default command runs one warmup round and five measured samples. Override
the local run shape when needed:

```sh
make bench PERF_ITERATIONS=30 PERF_SAMPLES=7 PERF_WARMUP=2
```

## Benchmark Cases

| Case | Purpose |
| --- | --- |
| `api_lint_large_document` | Measures full-rule lint cost on a diagnostics-heavy generated document. |
| `api_lint_clean_large_document` | Measures full-rule lint cost on a clean generated document. |
| `api_fix_large_document` | Measures fix orchestration cost on the diagnostics-heavy document. |
| `api_lint_many_small_documents` | Measures repeated API calls for many small documents. |
| `cli_check_many_small_files` | Measures CLI directory check cost for a synthetic workspace. |
| `config_validate_representative` | Measures config load and validation cost. |
| `api_rule_catalog` | Measures rule catalog construction cost. |

The generated corpus intentionally includes headings, lists, block quotes,
URLs, fenced code, tables, and trailing spaces. It is synthetic so the report
is reproducible and does not depend on local repository content.

## Baseline Policy

`make perf-check` validates the schema version, required cases, and statistic
fields. Timing deltas are printed as information, not as a hard failure. The
comparison summary uses `median_ms` because it is less sensitive to short CPU
spikes than a single sample.

When changing benchmark shape, update the baseline in the same change:

```sh
make perf-refresh-baseline
```

## Current Hot Path

CLI fix mode used to lint the original file before fix mode, call the public
`fix(content, options)` API, then lint the fixed content. Since the public API
must keep linting for callers that only have source text, the CLI now reuses
the diagnostics it already computed and calls `fix_with_results`.

The optimization is intentionally narrow:

- The public `fix(content, options)` API remains unchanged.
- CLI fix mode still lints the fixed content before deciding the exit code.
- The helper exists only to avoid duplicate original-content lint evaluation
  when diagnostics are already available.
