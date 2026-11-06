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
        let mut parsed = Self {
            output: PathBuf::from("target/perf-report.json"),
            iterations: DEFAULT_ITERATIONS,
            samples: DEFAULT_SAMPLES,
            warmup: DEFAULT_WARMUP,
        };
        let mut args = args.peekable();
        while let Some(arg) = args.next() {
            if matches!(
                arg.as_str(),
                "--output" | "--iterations" | "--samples" | "--warmup"
            ) {
                parsed.apply_option(&arg, args.next());
            }
        }
        parsed
    }

    fn apply_option(&mut self, arg: &str, value: Option<String>) {
        match arg {
            "--output" => {
                if let Some(value) = value {
                    self.output = PathBuf::from(value);
                }
            }
            "--iterations" => {
                if let Some(value) = value {
                    self.iterations = parse_positive(&value, DEFAULT_ITERATIONS);
                }
            }
            "--samples" => {
                if let Some(value) = value {
                    self.samples = parse_positive(&value, DEFAULT_SAMPLES);
                }
            }
            "--warmup" => {
                if let Some(value) = value {
                    self.warmup = value.parse().unwrap_or(DEFAULT_WARMUP);
                }
            }
            _ => {}
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
