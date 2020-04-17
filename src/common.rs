use crate::bench::Bencher;

use std::fmt;
use std::mem;
use std::ptr;

#[derive(Clone)]
pub struct BenchmarkInfo {
    name: String,
    pub(crate) func: fn(&mut Bencher),
}

impl BenchmarkInfo {
    #[inline]
    pub fn new(name: &str, func: fn(&mut Bencher)) -> Self {
        Self {
            name: name.to_owned(),
            func,
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &*self.name
    }
}

impl fmt::Debug for BenchmarkInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BenchmarkInfo")
            .field("name", &self.name)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct BenchmarkGroup {
    name: String,
    file: String,
    benchmarks: Vec<BenchmarkInfo>,
}

impl BenchmarkGroup {
    #[inline]
    pub fn new(name: &str, file: &str, benchmarks: Vec<BenchmarkInfo>) -> Self {
        Self {
            name: name.to_owned(),
            file: file.to_owned(),
            benchmarks,
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &*self.name
    }

    #[inline]
    pub fn file(&self) -> &str {
        &*self.file
    }

    #[inline]
    pub fn benchmarks(&self) -> &[BenchmarkInfo] {
        &*self.benchmarks
    }
}

pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = ptr::read_volatile(&dummy);
        mem::forget(dummy);
        ret
    }
}
