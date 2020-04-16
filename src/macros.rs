#[macro_export]
macro_rules! smbench_group {
    ($group:ident, $($bench:path),*) => {
        #[inline]
        pub fn $group() -> $crate::BenchmarkGroup {
            use $crate::BenchmarkInfo;

            let benches = [
                $(
                    BenchmarkInfo::new(
                        stringify!($bench),
                        $bench,
                    )
                ),*
            ];

            $crate::BenchmarkGroup::new(
                stringify!($group),
                file!(),
                benches.to_vec()
            )
        }
    };
    ($group:ident, $($bench:path,)*) => {
        smbench_group!($($r),*);
    }
}

#[macro_export]
macro_rules! smbench_main {
    ($($group:path),*) => {
        fn main() {
            use $crate::BenchmarkConfig;
            use $crate::App;
            use $crate::ConsoleReporter;
            use std::sync::Arc;

            let config = Arc::new(BenchmarkConfig::from_args());
            let mut app = App::from_config(config);
            app.add_reporter(ConsoleReporter::new());

            $(
                app.bench_group(&$group());
            )*
        }
    };
    ($($group:path,)*) => {
        smbench_main!($($group),*);
    }
}

#[macro_export]
macro_rules! smbench_trace_memory {
    () => {
        #[global_allocator]
        static ALLOC: $crate::memory::TraceAllocator = $crate::memory::TraceAllocator;
    };
}
