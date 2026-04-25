use katana_markdown_linter::cli::{run, Cli, Command, OutputFormat};
use katana_markdown_linter::rules::markdown::DocumentContext;
use katana_markdown_linter::{available_rules, fix, lint, LintOptions, MarkdownLintConfig};
use serde::Serialize;
use std::fs;
use std::hint::black_box;
use std::path::{Path, PathBuf};
use std::time::Instant;

const SCHEMA_VERSION: u32 = 2;
const DEFAULT_ITERATIONS: usize = 20;
const DEFAULT_SAMPLES: usize = 5;
const DEFAULT_WARMUP: usize = 1;
const CLI_WORKSPACE_FILES: usize = 80;

type BenchResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> BenchResult<()> {
    let args = Args::parse(std::env::args().skip(1));
    let options = LintOptions::default();
    let large_document = generate_large_document();
    let clean_large_document = generate_clean_large_document();
    let many_small_documents = generate_many_small_documents();
    let workspace = prepare_cli_workspace()?;
    let config_path = prepare_config_fixture()?;

    let mut cases = Vec::new();
    cases.push(measure(
        "api_lint_large_document",
        &args,
        large_document.lines().count(),
        "lines",
        || Ok(lint(black_box(&large_document), black_box(&options))?.len()),
    )?);
    cases.push(measure(
        "api_lint_clean_large_document",
        &args,
        clean_large_document.lines().count(),
        "lines",
        || Ok(lint(black_box(&clean_large_document), black_box(&options))?.len()),
    )?);
    cases.push(measure(
        "api_fix_large_document",
        &args,
        large_document.lines().count(),
        "lines",
        || Ok(fix(black_box(&large_document), black_box(&options))?.applied_fixes),
    )?);
    cases.push(measure(
        "api_lint_many_small_documents",
        &args,
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
        "context_build_large_document",
        &args,
        large_document.lines().count(),
        "lines",
        || {
            let ctx = DocumentContext::new(Path::new("<bench>"), black_box(&large_document));
            Ok(ctx.lines().len())
        },
    )?);
    cases.push(measure(
        "context_heading_index_large_document",
        &args,
        large_document.lines().count(),
        "lines",
        || {
            let ctx = DocumentContext::new(Path::new("<bench>"), black_box(&large_document));
            Ok(ctx.headings().len())
        },
    )?);
    cases.push(measure(
        "context_table_index_large_document",
        &args,
        large_document.lines().count(),
        "lines",
        || {
            let ctx = DocumentContext::new(Path::new("<bench>"), black_box(&large_document));
            Ok(ctx.tables().len())
        },
    )?);
    cases.push(measure(
        "cli_check_many_small_files",
        &args,
        CLI_WORKSPACE_FILES,
        "files",
        || run_cli_check(&workspace),
    )?);
    cases.push(measure(
        "config_validate_representative",
        &args.scaled(50),
        1,
        "config",
        || validate_config(&config_path),
    )?);
    cases.push(measure(
        "api_rule_catalog",
        &args.scaled(50),
        1,
        "catalog",
        || Ok(available_rules().len()),
    )?);

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

fn measure<F>(
    name: &'static str,
    args: &Args,
    work_units: usize,
    work_unit_name: &'static str,
    mut operation: F,
) -> BenchResult<Case>
where
    F: FnMut() -> BenchResult<usize>,
{
    for _ in 0..args.warmup {
        run_iterations(args.iterations, &mut operation)?;
    }

    let mut sample_ms = Vec::with_capacity(args.samples);
    let mut total_ms = 0.0;
    let mut observed_items = 0;
    for _ in 0..args.samples {
        let start = Instant::now();
        observed_items += black_box(run_iterations(args.iterations, &mut operation)?);
        let elapsed = start.elapsed().as_secs_f64() * 1000.0;
        total_ms += elapsed;
        sample_ms.push(elapsed / args.iterations as f64);
    }

    let stats = Stats::from_samples(&sample_ms, total_ms);
    Ok(Case {
        name,
        iterations: args.iterations,
        samples: args.samples,
        work_units,
        work_unit_name,
        total_ms: stats.total_ms,
        mean_ms: stats.mean_ms,
        median_ms: stats.median_ms,
        min_ms: stats.min_ms,
        max_ms: stats.max_ms,
        stddev_ms: stats.stddev_ms,
        sample_ms,
        observed_items,
    })
}

fn run_iterations<F>(iterations: usize, operation: &mut F) -> BenchResult<usize>
where
    F: FnMut() -> BenchResult<usize>,
{
    let mut observed_items = 0;
    for _ in 0..iterations {
        observed_items += black_box(operation()?);
    }
    Ok(observed_items)
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

fn generate_clean_large_document() -> String {
    let mut content = String::from("# Title\n\n");
    for index in 0..400 {
        content.push_str(&format!("## Section {index}\n\n"));
        content.push_str("Paragraph text stays short and plain.\n\n");
        content.push_str("- first item\n");
        content.push_str("- second item\n\n");
        content.push_str("```rust\nfn main() {}\n```\n\n");
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

fn prepare_cli_workspace() -> BenchResult<PathBuf> {
    let dir = std::env::temp_dir().join(format!("kml-perf-workspace-{}", std::process::id()));
    if dir.exists() {
        fs::remove_dir_all(&dir)?;
    }
    fs::create_dir_all(&dir)?;
    fs::write(dir.join(".markdownlint.json"), "{ \"default\": true }\n")?;
    for index in 0..CLI_WORKSPACE_FILES {
        fs::write(
            dir.join(format!("doc-{index:03}.md")),
            format!("# Document {index}\n\nParagraph text.\n\n"),
        )?;
    }
    Ok(dir)
}

fn prepare_config_fixture() -> BenchResult<PathBuf> {
    let dir = std::env::temp_dir().join(format!("kml-perf-config-{}", std::process::id()));
    fs::create_dir_all(&dir)?;
    let path = dir.join(".markdownlint.jsonc");
    fs::write(
        &path,
        r#"{
  "default": true,
  "MD013": false,
  "MD024": true,
  "MD031": true,
  "MD048": true
}
"#,
    )?;
    Ok(path)
}

fn run_cli_check(workspace: &Path) -> BenchResult<usize> {
    let exit = run(Cli {
        command: Command::Check,
        format: OutputFormat::Text,
        inputs: vec![workspace.display().to_string()],
        quiet: true,
        ..Cli::default()
    })
    .map_err(std::io::Error::other)?;
    if exit != 0 {
        return Err(std::io::Error::other(format!("kml check exited with {exit}")).into());
    }
    Ok(CLI_WORKSPACE_FILES)
}

fn validate_config(config_path: &Path) -> BenchResult<usize> {
    let config = MarkdownLintConfig::load(config_path)?;
    let rules =
        katana_markdown_linter::rules::markdown::MarkdownLinterOps::user_configurable_rules();
    let errors = config.validate_cached_rules();
    if !errors.is_empty() {
        return Err(std::io::Error::other(format!(
            "config validation failed with {} errors",
            errors.len()
        ))
        .into());
    }
    Ok(rules.len())
}

#[derive(Debug, Clone)]
struct Args {
    output: PathBuf,
    iterations: usize,
    samples: usize,
    warmup: usize,
}

impl Args {
    fn parse(args: impl Iterator<Item = String>) -> Self {
        let mut output = PathBuf::from("target/perf-report.json");
        let mut iterations = DEFAULT_ITERATIONS;
        let mut samples = DEFAULT_SAMPLES;
        let mut warmup = DEFAULT_WARMUP;
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
                        iterations = parse_positive(&value, DEFAULT_ITERATIONS);
                    }
                }
                "--samples" => {
                    if let Some(value) = args.next() {
                        samples = parse_positive(&value, DEFAULT_SAMPLES);
                    }
                }
                "--warmup" => {
                    if let Some(value) = args.next() {
                        warmup = value.parse().unwrap_or(DEFAULT_WARMUP);
                    }
                }
                _ => {}
            }
        }
        Self {
            output,
            iterations,
            samples,
            warmup,
        }
    }

    fn scaled(&self, factor: usize) -> Self {
        let mut args = self.clone();
        args.iterations *= factor;
        args
    }
}

fn parse_positive(value: &str, default: usize) -> usize {
    value
        .parse()
        .ok()
        .filter(|parsed| *parsed > 0)
        .unwrap_or(default)
}

#[derive(Debug)]
struct Stats {
    total_ms: f64,
    mean_ms: f64,
    median_ms: f64,
    min_ms: f64,
    max_ms: f64,
    stddev_ms: f64,
}

impl Stats {
    fn from_samples(samples: &[f64], total_ms: f64) -> Self {
        let mut sorted = samples.to_vec();
        sorted.sort_by(|left, right| left.total_cmp(right));
        let mean_ms = samples.iter().sum::<f64>() / samples.len() as f64;
        let median_ms = median(&sorted);
        let min_ms = *sorted.first().unwrap_or(&0.0);
        let max_ms = *sorted.last().unwrap_or(&0.0);
        let variance = samples
            .iter()
            .map(|sample| {
                let delta = sample - mean_ms;
                delta * delta
            })
            .sum::<f64>()
            / samples.len() as f64;
        Self {
            total_ms,
            mean_ms,
            median_ms,
            min_ms,
            max_ms,
            stddev_ms: variance.sqrt(),
        }
    }
}

fn median(sorted: &[f64]) -> f64 {
    let mid = sorted.len() / 2;
    if sorted.len().is_multiple_of(2) {
        (sorted[mid - 1] + sorted[mid]) / 2.0
    } else {
        sorted[mid]
    }
}

#[derive(Debug, Serialize)]
struct Report {
    schema_version: u32,
    generated_by: &'static str,
    iterations: usize,
    warmup_iterations: usize,
    samples: usize,
    cases: Vec<Case>,
}

#[derive(Debug, Serialize)]
struct Case {
    name: &'static str,
    iterations: usize,
    samples: usize,
    work_units: usize,
    work_unit_name: &'static str,
    total_ms: f64,
    mean_ms: f64,
    median_ms: f64,
    min_ms: f64,
    max_ms: f64,
    stddev_ms: f64,
    sample_ms: Vec<f64>,
    observed_items: usize,
}
