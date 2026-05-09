use std::path::PathBuf;

const DEFAULT_ITERATIONS: usize = 20;
const DEFAULT_SAMPLES: usize = 5;
const DEFAULT_WARMUP: usize = 1;

#[derive(Debug, Clone)]
pub(super) struct Args {
    pub(super) output: PathBuf,
    pub(super) iterations: usize,
    pub(super) samples: usize,
    pub(super) warmup: usize,
}

impl Args {
    pub(super) fn parse(args: impl Iterator<Item = String>) -> Self {
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

    pub(super) fn scaled(&self, factor: usize) -> Self {
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
