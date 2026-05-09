#[path = "perf_benchmark/args.rs"]
mod args;
#[path = "perf_benchmark/cases.rs"]
mod cases;
#[path = "perf_benchmark/documents.rs"]
mod documents;
#[path = "perf_benchmark/measure.rs"]
mod measure;
#[path = "perf_benchmark/report.rs"]
mod report;
#[path = "perf_benchmark/workspace.rs"]
mod workspace;

use args::Args;
use report::Report;
use std::fs;

const SCHEMA_VERSION: u32 = 2;

type BenchResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> BenchResult<()> {
    let args = Args::parse(std::env::args().skip(1));
    let cases = cases::collect_cases(&args)?;
    let report = Report {
        schema_version: SCHEMA_VERSION,
        generated_by: "examples/perf_benchmark.rs",
        iterations: args.iterations,
        warmup_iterations: args.warmup,
        samples: args.samples,
        cases,
    };

    if let Some(parent) = args.output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&args.output, serde_json::to_string_pretty(&report)? + "\n")?;

    println!("Performance report written: {}", args.output.display());
    for case in &report.cases {
        println!(
            "{}: median={:.3}ms mean={:.3}ms min={:.3}ms max={:.3}ms observed={}",
            case.name, case.median_ms, case.mean_ms, case.min_ms, case.max_ms, case.observed_items
        );
    }
    Ok(())
}
