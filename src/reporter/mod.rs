mod console;
pub use console::*;

use crate::bench::BenchmarkResult;
use crate::common::{BenchmarkGroup, BenchmarkInfo};
use crate::config::BenchmarkConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct ReporterOptions {
    pub benchmem: bool,
    pub confidence_level: f64,
}

impl ReporterOptions {
    pub fn from_config(config: &BenchmarkConfig) -> Self {
        ReporterOptions {
            benchmem: config.benchmem,
            confidence_level: config.confidence_level,
        }
    }
}

pub trait Reporter {
    fn on_group_init(&self, _group: &BenchmarkGroup, _options: &ReporterOptions) {}
    fn on_benchmark_start(&self, _info: &BenchmarkInfo, _options: &ReporterOptions) {}
    #[doc(hidden)]
    fn on_warmup(&self, _info: &BenchmarkInfo, _options: &ReporterOptions) {}
    #[doc(hidden)]
    fn on_terminated(&self, _info: &BenchmarkInfo, _options: &ReporterOptions) {}
    #[doc(hidden)]
    fn on_measurement_start(
        &self,
        _info: &BenchmarkInfo,
        _sample_size: usize,
        _estimated_time: f64,
        _options: &ReporterOptions,
    ) {
    }
    fn on_benchmark_complete(
        &self,
        _info: &BenchmarkInfo,
        _result: &BenchmarkResult,
        _options: &ReporterOptions,
    ) {
    }
    fn on_group_finish(&self, _group: &BenchmarkGroup, _options: &ReporterOptions) {}
}
