use std::cmp;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::common::black_box;
use crate::config::BenchmarkConfig;
use crate::error::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct BenchmarkResult {
    pub measurements: Vec<(usize, f64)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Bencher {
    measure_time: bool,
    dur: Duration,
    iterations: usize,
    config: Arc<BenchmarkConfig>,
}

impl Bencher {
    pub fn new(config: Arc<BenchmarkConfig>) -> Bencher {
        Bencher {
            measure_time: true,
            dur: Duration::new(0, 0),
            iterations: 1,
            config,
        }
    }

    #[inline]
    pub fn iter<T, F>(&mut self, mut inner: F)
    where
        F: FnMut() -> T,
    {
        let start = Instant::now();
        let k = self.iterations;
        for _ in 0..k {
            black_box(inner());
        }
        self.dur = start.elapsed();
        self.measure_time = true;
    }

    fn warm_up(&mut self, how_long: Duration, f: fn(&mut Bencher)) -> f64 {
        let how_long = duration_ns(how_long);
        let mut total_iters = 0;
        let mut elapsed_time = 0;
        self.iterations = 1;

        // finish warmups if total_iters exceeds 1000000 times
        while elapsed_time < how_long / 2 {
            f(self);
            total_iters += self.iterations;
            elapsed_time += duration_ns(self.dur);
            self.iterations <<= 1;
        }

        elapsed_time = cmp::max(elapsed_time, 1);

        let ns_per_iter = elapsed_time / total_iters as u64;
        self.iterations = (how_long.saturating_sub(elapsed_time) / ns_per_iter) as usize;
        black_box(f(self));

        // recalculate ns_per_iter
        elapsed_time as f64 / total_iters as f64
    }

    #[inline(never)]
    pub(crate) fn auto_bench(&mut self, mut f: fn(&mut Bencher)) -> Result<BenchmarkResult, Error> {
        f = black_box(f);

        self.iterations = 1;
        self.measure_time = false;
        f(self);

        if !self.measure_time {
            return Err(Error::InvalidBenchmarkFunction(
                "Bencher::iter() method was not called.".to_string(),
            ));
        };

        let ns_per_iter = self.warm_up(Duration::from_secs_f64(self.config.warmup_time), f);

        // ensure that each iteration takes >100us
        let d = cmp::max((100_000. / ns_per_iter) as usize + 1, 10);
        let max_iters = self.config.measurement_time * 1e9 / ns_per_iter;

        // maximize n over `(d + 2 * d + ... + n * d) <= max_iters`
        let n = cmp::max(
            (((1.0 + 8.0 * max_iters / d as f64).sqrt() - 1.0) * 0.5) as usize,
            1,
        );
        let mut measurements = Vec::with_capacity(n);
        for k in (d..=d * n).step_by(d) {
            self.iterations = k;
            f(self);
            measurements.push((k, self.dur.as_secs_f64()));
        }

        Ok(BenchmarkResult {
            measurements,
        })
    }
}

#[inline]
fn duration_ns(dur: Duration) -> u64 {
    dur.as_secs() * 1_000_000_000 + dur.subsec_nanos() as u64
}
