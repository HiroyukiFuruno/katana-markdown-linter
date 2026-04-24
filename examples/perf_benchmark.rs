use katana_markdown_linter::{available_rules, fix, lint, LintOptions};
use serde::Serialize;
use std::fs;
use std::hint::black_box;
use std::path::PathBuf;
use std::time::Instant;

const DEFAULT_ITERATIONS: usize = 20;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse(std::env::args().skip(1));
    let options = LintOptions::default();
    let large_document = generate_large_document();
    let many_small_documents = generate_many_small_documents();

    let mut cases = Vec::new();
    cases.push(measure(
        "api_lint_large_document",
        args.iterations,
        large_document.lines().count(),
        "lines",
        || lint(black_box(&large_document), black_box(&options)).map(|items| items.len()),
    )?);
    cases.push(measure(
        "api_fix_large_document",
        args.iterations,
        large_document.lines().count(),
        "lines",
        || fix(black_box(&large_document), black_box(&options)).map(|result| result.applied_fixes),
    )?);
    cases.push(measure(
        "api_lint_many_small_documents",
        args.iterations,
        many_small_documents.len(),
        "documents",
        || {
            let mut diagnostics = 0;
            for document in &many_small_documents {
                diagnostics += lint(black_box(document), black_box(&options))?.len();
            }
            Ok(diagnostics)
        },
    )?);
    cases.push(measure(
        "api_rule_catalog",
        args.iterations * 50,
        1,
        "catalog",
        || Ok(available_rules().len()),
    )?);

    let report = Report {
        schema_version: 1,
        generated_by: "examples/perf_benchmark.rs",
        iterations: args.iterations,
        cases,
    };

    if let Some(parent) = args.output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&args.output, serde_json::to_string_pretty(&report)? + "\n")?;

    println!("Performance report written: {}", args.output.display());
    for case in &report.cases {
        println!(
            "{}: avg={:.3}ms total={:.3}ms observed={}",
            case.name, case.average_ms, case.total_ms, case.observed_items
        );
    }
    Ok(())
}

fn measure<F>(
    name: &'static str,
    iterations: usize,
    work_units: usize,
    work_unit_name: &'static str,
    mut operation: F,
) -> Result<Case, katana_markdown_linter::Error>
where
    F: FnMut() -> Result<usize, katana_markdown_linter::Error>,
{
    black_box(operation()?);
    let start = Instant::now();
    let mut observed_items = 0;
    for _ in 0..iterations {
        observed_items += black_box(operation()?);
    }
    let elapsed = start.elapsed();
    let total_ms = elapsed.as_secs_f64() * 1000.0;
    let average_ms = total_ms / iterations as f64;
    Ok(Case {
        name,
        iterations,
        work_units,
        work_unit_name,
        total_ms,
        average_ms,
        observed_items,
    })
}

fn generate_large_document() -> String {
    let mut content = String::new();
    for index in 0..600 {
        content.push_str("#Heading\n");
        content.push_str("Paragraph with bare URL https://example.com and trailing spaces.  \n");
        content.push_str(">  blockquote with too many spaces\n");
        content.push_str("+ list item\n");
        content.push_str("```rust\nfn main() {}\n```\n\n");
        if index % 25 == 0 {
            content.push_str("| a | b |\n|---|---|\n| 1 | 2 |\n\n");
        }
    }
    content
}

fn generate_many_small_documents() -> Vec<String> {
    (0..250)
        .map(|index| {
            format!("# Doc {index}\n\nParagraph with https://example.com/{index}\n\n+ item\n\n")
        })
        .collect()
}

#[derive(Debug)]
struct Args {
    output: PathBuf,
    iterations: usize,
}

impl Args {
    fn parse(args: impl Iterator<Item = String>) -> Self {
        let mut output = PathBuf::from("target/perf-report.json");
        let mut iterations = DEFAULT_ITERATIONS;
        let mut args = args.peekable();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--output" => {
                    if let Some(value) = args.next() {
                        output = PathBuf::from(value);
                    }
                }
                "--iterations" => {
                    if let Some(value) = args.next() {
                        iterations = value
                            .parse()
                            .ok()
                            .filter(|iterations| *iterations > 0)
                            .unwrap_or(DEFAULT_ITERATIONS);
                    }
                }
                _ => {}
            }
        }
        Self { output, iterations }
    }
}

#[derive(Debug, Serialize)]
struct Report {
    schema_version: u32,
    generated_by: &'static str,
    iterations: usize,
    cases: Vec<Case>,
}

#[derive(Debug, Serialize)]
struct Case {
    name: &'static str,
    iterations: usize,
    work_units: usize,
    work_unit_name: &'static str,
    total_ms: f64,
    average_ms: f64,
    observed_items: usize,
}
