use crate::memory::MemoryUsage;
use crate::stats::{self, Normal, Regression};
use crate::BenchmarkResult;

#[derive(Debug)]
pub struct Summary {
    pub elapsed_time: Normal,
    pub memory_usage: Option<MemoryUsage>,
}

pub fn summarize(result: &BenchmarkResult) -> Summary {
    let mut sec_per_iters: Vec<f64> = result
        .measurements
        .iter()
        .map(|(i, t)| t / *i as f64)
        .collect();

    let (l, r) = stats::outlier_bound(&mut sec_per_iters, 3.0);
    let data_iter = result.measurements.iter().filter(|&(i, t)| {
        let sec_per_iter = t / *i as f64;
        l <= sec_per_iter && sec_per_iter <= r
    });

    let x: Vec<f64> = data_iter.clone().map(|(i, _)| *i as f64).collect();
    let y: Vec<f64> = data_iter.map(|(_, t)| *t).collect();
    let slope = stats::LeastSquare.slope(&x, &y);

    Summary {
        elapsed_time: slope,
        memory_usage: result.memory_usage.clone(),
    }
}
