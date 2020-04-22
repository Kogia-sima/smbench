use std::cell::RefCell;
use serde::ser::{Serialize, Serializer, SerializeStruct};

use super::{Reporter, ReporterOptions};
use crate::stats::Distribution;
use crate::summary;
use crate::{BenchmarkGroup, BenchmarkInfo, BenchmarkResult};
use crate::memory::MemoryUsage;

struct BenchmarkRecords {
    groups: Vec<GroupBenchmarkRecord>
}

struct GroupBenchmarkRecord {
    name: String,
    benchmarks: Vec<BenchmarkRecord>
}

struct BenchmarkRecord {
    name: String,
    mean: f64,
    confidence_interval: (f64, f64),
    memory_usage: Option<MemoryUsage>,
}

impl Serialize for BenchmarkRecords {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut s = serializer.serialize_struct("BenchmarkRecords", 1)?;
        s.serialize_field("groups", &self.groups)?;
        s.end()
    }
}

impl Serialize for GroupBenchmarkRecord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut s = serializer.serialize_struct("GroupBenchmarkRecord", 1)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("benchmarks", &self.benchmarks)?;
        s.end()
    }
}

impl Serialize for BenchmarkRecord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut s = serializer.serialize_struct("BenchmarkRecord", 1)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("mean", &self.mean)?;
        s.serialize_field("confidence_interval", &self.confidence_interval)?;
        if let Some(ref memory) = self.memory_usage {
            s.serialize_field("max_heap_size", &memory.max_heap_size)?;
            s.serialize_field("alloc_size", &memory.alloc_size)?;
            s.serialize_field("alloc_counts", &memory.alloc_counts)?;
        }
        s.end()
    }
}

pub struct JsonReporter {
    data: RefCell<BenchmarkRecords>,
}

impl JsonReporter {
    pub fn new() -> Self {
        Self {
            data: RefCell::new(BenchmarkRecords { groups: Vec::new() })
        }
    }
}

impl Reporter for JsonReporter {
    fn on_group_init(&self, group: &BenchmarkGroup, _options: &ReporterOptions) {
        let new_entry = GroupBenchmarkRecord {
            name: group.name().to_owned(),
            benchmarks: Vec::new()
        };
        self.data.borrow_mut().groups.push(new_entry);
    }

    fn on_benchmark_complete(
        &self,
        info: &BenchmarkInfo,
        result: &BenchmarkResult,
        options: &ReporterOptions,
    ) {
        let summ = summary::summarize(result);
        let mean = summ.elapsed_time.mean();

        let margin = (1.0 - options.confidence_level) * 0.5;
        let confidence_interval = (
            summ.elapsed_time.icdf(margin),
            summ.elapsed_time.icdf(1.0 - margin),
        );

        let new_entry = BenchmarkRecord {
            name: info.name().to_owned(),
            mean,
            confidence_interval,
            memory_usage: result.memory_usage.clone()
        };

        self.data.borrow_mut().groups.last_mut().unwrap().benchmarks.push(new_entry);
    }

    fn on_finish(&self, _options: &ReporterOptions) {
        let rendered = serde_json::to_string_pretty(&*self.data.borrow()).unwrap();
        println!("{}", rendered);
    }
}
