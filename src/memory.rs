use std::alloc::{GlobalAlloc, Layout, System};
use std::cmp;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicUsize, Ordering};

use crate::black_box;

static TRACE_MEMORY: AtomicBool = AtomicBool::new(false);
static HEAP_SIZE: AtomicI64 = AtomicI64::new(0);
static MAX_HEAP_SIZE: AtomicI64 = AtomicI64::new(0);
static ALLOC_COUNTS: AtomicUsize = AtomicUsize::new(0);
static ALLOC_SIZE: AtomicUsize = AtomicUsize::new(0);

#[doc(hidden)]
pub struct TraceAllocator;

unsafe impl GlobalAlloc for TraceAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = System.alloc(layout);
        if TRACE_MEMORY.load(Ordering::Acquire) && !ptr.is_null() {
            ALLOC_COUNTS.fetch_add(1, Ordering::Relaxed);
            ALLOC_SIZE.fetch_add(layout.size(), Ordering::Relaxed);

            let alloc_size = layout.size() as i64;
            let new = HEAP_SIZE.fetch_add(alloc_size, Ordering::SeqCst) + alloc_size;
            if new > MAX_HEAP_SIZE.load(Ordering::SeqCst) {
                MAX_HEAP_SIZE.store(new, Ordering::SeqCst);
            }
        }

        ptr
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        if TRACE_MEMORY.load(Ordering::Acquire) && !ptr.is_null() {
            HEAP_SIZE.fetch_sub(layout.size() as i64, Ordering::SeqCst);
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MemoryUsage {
    pub max_heap_size: usize,
    pub alloc_counts: usize,
    pub alloc_size: usize,
}

pub fn benchmark(f: &mut dyn FnMut()) -> MemoryUsage {
    // initialize counters
    TRACE_MEMORY.store(true, Ordering::SeqCst);
    let initial = HEAP_SIZE.load(Ordering::SeqCst);
    MAX_HEAP_SIZE.store(initial, Ordering::SeqCst);
    ALLOC_COUNTS.store(0, Ordering::SeqCst);
    ALLOC_SIZE.store(0, Ordering::SeqCst);

    // run the function
    black_box(f());

    // calculate the benchmark result
    let max_heap_size = cmp::max(MAX_HEAP_SIZE.load(Ordering::SeqCst) - initial, 0) as usize;
    let alloc_counts = ALLOC_COUNTS.load(Ordering::SeqCst);
    let alloc_size = ALLOC_SIZE.load(Ordering::SeqCst);

    TRACE_MEMORY.store(false, Ordering::SeqCst);

    MemoryUsage {
        max_heap_size,
        alloc_counts,
        alloc_size,
    }
}

#[inline]
pub fn benchmark_is_reliable() -> bool {
    let result = benchmark(&mut || {
        black_box(Vec::<u32>::with_capacity(3));
    });

    let expected = MemoryUsage {
        max_heap_size: 12,
        alloc_counts: 1,
        alloc_size: 12,
    };

    result == expected
}
