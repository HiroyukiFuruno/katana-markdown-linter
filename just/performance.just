# ============================================================
# Performance
# ============================================================

# Run repeatable performance benchmarks and write target/perf-report.json
bench:
    cargo run --release --example perf_benchmark --locked -- --output {{PERF_REPORT}} --iterations {{PERF_ITERATIONS}} --samples {{PERF_SAMPLES}} --warmup {{PERF_WARMUP}}

# Compare performance report with the committed baseline
perf-check: bench
    python3 scripts/ci/perf-check.py --baseline {{PERF_BASELINE}} --report {{PERF_REPORT}}

# Compare performance report and fail when strict ratio thresholds are exceeded
perf-check-strict: bench
    python3 scripts/ci/perf-check.py --baseline {{PERF_BASELINE}} --report {{PERF_REPORT}} --strict --max-ratio 1.4

# Refresh performance baseline after intentional optimization
perf-refresh-baseline: bench
    python3 scripts/ci/perf-check.py --update --baseline {{PERF_BASELINE}} --report {{PERF_REPORT}}

# Benchmark kml against optional mado and rumdl CLIs
bench-cross-tools:
    cargo build --release --bin kml --locked
    python3 scripts/bench/cross-tool-cli-benchmark.py --kml {{CROSS_TOOL_BIN}} --output {{CROSS_TOOL_REPORT}} --summary {{CROSS_TOOL_SUMMARY}} --runs {{CROSS_TOOL_RUNS}} --warmup {{CROSS_TOOL_WARMUP}} {{CROSS_TOOL_ARGS}}

# Benchmark default check behavior across available CLIs
bench-cross-tools-default:
    cargo build --release --bin kml --locked
    python3 scripts/bench/cross-tool-cli-benchmark.py --mode default --workflow check --kml {{CROSS_TOOL_BIN}} --output {{CROSS_TOOL_REPORT}} --summary {{CROSS_TOOL_SUMMARY}} --runs {{CROSS_TOOL_RUNS}} --warmup {{CROSS_TOOL_WARMUP}} {{CROSS_TOOL_ARGS}}

# Benchmark common-subset check behavior across available CLIs
bench-cross-tools-common:
    cargo build --release --bin kml --locked
    python3 scripts/bench/cross-tool-cli-benchmark.py --mode common --workflow check --kml {{CROSS_TOOL_BIN}} --output {{CROSS_TOOL_REPORT}} --summary {{CROSS_TOOL_SUMMARY}} --runs {{CROSS_TOOL_RUNS}} --warmup {{CROSS_TOOL_WARMUP}} {{CROSS_TOOL_ARGS}}

# Benchmark fix behavior across available CLIs
bench-cross-tools-fix:
    cargo build --release --bin kml --locked
    python3 scripts/bench/cross-tool-cli-benchmark.py --workflow fix --kml {{CROSS_TOOL_BIN}} --output {{CROSS_TOOL_REPORT}} --summary {{CROSS_TOOL_SUMMARY}} --runs {{CROSS_TOOL_RUNS}} --warmup {{CROSS_TOOL_WARMUP}} {{CROSS_TOOL_ARGS}}
