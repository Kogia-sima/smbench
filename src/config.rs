#[cfg(feature = "argparse")]
use argparse::{
    action::{IFlagAction, ParseResult},
    ArgumentParser, Store, StoreTrue,
};

#[derive(Clone, Debug, PartialEq)]
pub struct BenchmarkConfig {
    pub filter: String,
    pub benchmem: bool,
    pub warmup_time: f64,
    pub measurement_time: f64,
    pub confidence_level: f64,
}

impl BenchmarkConfig {
    #[cfg(feature = "argparse")]
    pub fn from_args() -> Self {
        struct StoreNone;
        impl IFlagAction for StoreNone {
            fn parse_flag(&self) -> ParseResult {
                ParseResult::Parsed
            }
        }

        let mut config = BenchmarkConfig::default();

        let mut ap = ArgumentParser::new();
        ap.set_description("SMBench Executable");
        ap.add_option(&["--bench"], StoreNone, "");
        ap.refer(&mut config.filter)
            .add_argument("filter", Store, "filter string for benchmarks");
        ap.refer(&mut config.benchmem).add_option(
            &["--benchmem"],
            StoreTrue,
            "benchmark memory usage",
        );
        ap.refer(&mut config.warmup_time).add_option(
            &["--warm-up-time"],
            Store,
            "Specify the maximum warm up time in seconds [default is 2.0]",
        );
        ap.refer(&mut config.measurement_time).add_option(
            &["--measurement-time"],
            Store,
            "Specify the measurement time in seconds [default is 3.0]",
        );
        ap.refer(&mut config.confidence_level).add_option(
            &["--confidence-level"],
            Store,
            "Specify the level of confidence intervals. [default is 0.95]",
        );

        ap.parse_args_or_exit();
        drop(ap);

        // varidate arguments
        if config.confidence_level < 0.0 || 1.0 < config.confidence_level {
            panic!(
                "invalid confidence level: {}. \
                confidence level must be positive and less than 1.0",
                config.confidence_level
            );
        }

        config
    }

    #[cfg(not(feature = "argparse"))]
    pub fn from_args() -> Self {
        Self::default()
    }
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        BenchmarkConfig {
            filter: "".to_owned(),
            benchmem: false,
            warmup_time: 2.0,
            measurement_time: 3.0,
            confidence_level: 0.95,
        }
    }
}
