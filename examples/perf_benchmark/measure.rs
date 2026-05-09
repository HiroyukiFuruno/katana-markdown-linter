use super::args::Args;
use super::report::Case;
use super::BenchResult;
use std::hint::black_box;
use std::time::Instant;

const MILLISECONDS_PER_SECOND: f64 = 1000.0;

pub(super) fn measure<F>(
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
        let elapsed = start.elapsed().as_secs_f64() * MILLISECONDS_PER_SECOND;
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
