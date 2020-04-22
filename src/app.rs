use std::sync::Arc;

use crate::bench::Bencher;
use crate::common::{BenchmarkGroup, BenchmarkInfo};
use crate::config::BenchmarkConfig;
use crate::reporter::{Reporter, ReporterOptions};

pub struct App {
    #[allow(dead_code)]
    config: Arc<BenchmarkConfig>,
    bencher: Bencher,
    reporters: Vec<Box<dyn Reporter>>,
    reporter_options: ReporterOptions,
}

impl App {
    #[inline]
    pub fn new() -> Self {
        Self::from_config(Arc::new(BenchmarkConfig::default()))
    }

    pub fn from_config(config: Arc<BenchmarkConfig>) -> Self {
        let bencher = Bencher::new(Arc::clone(&config));
        let reporter_options = ReporterOptions::from_config(&*config);
        let reporters = config
            .reporters_string
            .split(",")
            .map(|s| <dyn Reporter>::from_str(s))
            .collect();

        Self {
            config,
            bencher,
            reporters,
            reporter_options,
        }
    }

    pub fn bench_group(&mut self, group: &BenchmarkGroup) {
        self.reporters
            .iter()
            .for_each(|r| r.on_group_init(group, &self.reporter_options));

        for benchmark in group.benchmarks() {
            #[cfg(feature = "regex")]
            {
                let reg = regex::Regex::new(&self.config.filter).unwrap();
                if reg.is_match(benchmark.name()) {
                    self.bench_single(benchmark);
                }
            }

            if !cfg!(feature = "regex")
                && (self.config.filter.is_empty() || self.config.filter.contains(benchmark.name()))
            {
                self.bench_single(benchmark)
            }
        }

        self.reporters
            .iter()
            .for_each(|r| r.on_group_finish(group, &self.reporter_options));
    }

    pub fn bench_single(&mut self, info: &BenchmarkInfo) {
        self.reporters
            .iter()
            .for_each(|r| r.on_benchmark_start(info, &self.reporter_options));

        let result = match self.bencher.auto_bench(info.func) {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Benchmark failed: {:?}", e);
                return;
            }
        };

        self.reporters
            .iter()
            .for_each(|r| r.on_benchmark_complete(info, &result, &self.reporter_options));
    }

    pub fn finish(self) {}
}

impl Drop for App {
    fn drop(&mut self) {
        self.reporters.iter().for_each(|r| r.on_finish(&self.reporter_options));
    }
}
