use crate::bench::Bencher;

use std::fmt;

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

#[cfg(feature = "real_blackbox")]
pub use core::hint::black_box;

#[cfg(not(feature = "real_blackbox"))]
pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}

#[cfg(feature = "json")]
pub(crate) fn create_output_dir() -> Option<std::path::PathBuf> {
    use std::path::PathBuf;
    use std::process::Command;

    let output = Command::new("cargo")
        .args(&["metadata", "--format-version", "1"])
        .output()
        .ok()?;

    let stdout = unsafe { String::from_utf8_unchecked(output.stdout) };
    let start = stdout.rfind("\"target_directory\":")? + 20;
    let end = start + stdout[start..].find("\"")?;
    let mut dir = PathBuf::from(&stdout[start..end]);
    dir.push("smbench");

    if !dir.exists() {
        std::fs::create_dir(&dir).unwrap();
    }
    Some(dir)
}
