use std::cell::Cell;
use std::io::Write;

use super::{Reporter, ReporterOptions};
use crate::fmt;
use crate::stats::Distribution;
use crate::summary;
use crate::{BenchmarkGroup, BenchmarkInfo, BenchmarkResult};

pub struct ConsoleReporter {
    name_width_max: Cell<usize>,
}

impl ConsoleReporter {
    pub fn new() -> Self {
        #[cfg(feature = "sysinfo")]
        {
            use crate::sys::get_sysinfo;
            let sysinfo = get_sysinfo();
            if let Some(os) = sysinfo.os {
                println!("OS Type: {}", os.as_str());
            };
            if let Some(arch) = sysinfo.architecture {
                println!("CPU Architecture: {}", arch.as_str());
            }
            if let Some(ref model) = sysinfo.cpu_model {
                println!("CPU Model Name: {}", model);
            }
        }

        println!("SMBench Version: {}", env!("CARGO_PKG_VERSION"));
        println!("");

        ConsoleReporter {
            name_width_max: Cell::new(0),
        }
    }
}

impl Reporter for ConsoleReporter {
    fn on_group_init(&self, group: &BenchmarkGroup, _options: &ReporterOptions) {
        if let Some(w) = group.benchmarks().iter().map(|b| b.name().len()).max() {
            self.name_width_max.set(w);
        }
    }

    fn on_benchmark_start(&self, info: &BenchmarkInfo, _options: &ReporterOptions) {
        print!("{}: ", info.name());
        std::io::stdout().flush().unwrap();
    }

    fn on_benchmark_complete(
        &self,
        info: &BenchmarkInfo,
        result: &BenchmarkResult,
        options: &ReporterOptions,
    ) {
        let padding = self.name_width_max.get() - info.name().len();
        let summ = summary::summarize(result);
        let mean = summ.elapsed_time.mean();

        let margin = (1.0 - options.confidence_level) * 0.5;
        let confidence_interval = (
            summ.elapsed_time.icdf(margin),
            summ.elapsed_time.icdf(1.0 - margin),
        );

        print!(
            "{}{} [{}, {}]",
            " ".repeat(padding),
            fmt::time(mean),
            fmt::time(confidence_interval.0),
            fmt::time(confidence_interval.1)
        );

        if let Some(memory) = summ.memory_usage.as_ref() {
            print!(
                "{:>7} ({} allocs)",
                fmt::bytes(memory.alloc_size),
                memory.alloc_counts
            );
        }

        println!();
    }
}
