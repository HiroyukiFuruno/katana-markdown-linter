# Performance

This project treats performance work as a report-first loop. The first goal is
to make hot paths measurable on a local machine without turning CI into a flaky
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

## Benchmark Cases

| Case | Purpose |
| --- | --- |
| `api_lint_large_document` | Measures full-rule lint cost on a large generated document. |
| `api_fix_large_document` | Measures fix orchestration cost on the same large document. |
| `api_lint_many_small_documents` | Measures repeated API calls for many small files. |
| `api_rule_catalog` | Measures rule catalog construction cost. |

The generated corpus intentionally includes headings, lists, block quotes,
URLs, fenced code, tables, and trailing spaces. It is synthetic so the report
is reproducible and does not depend on local repository content.

## Baseline Policy

`make perf-check` validates that every baseline case still appears in the
current report. Timing deltas are printed as information, not as a hard failure.
This keeps the gate useful for shape regressions while avoiding false failures
from CPU, thermal, or background-process noise.

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
