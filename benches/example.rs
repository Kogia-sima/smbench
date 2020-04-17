use smbench::*;

#[inline]
fn fibonacci(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

fn fibonacci_20(b: &mut Bencher) {
    b.iter(|| fibonacci(black_box(20)));
}

fn heap_allocation(b: &mut Bencher) {
    b.iter(|| Vec::<u32>::with_capacity(10));
}

smbench_trace_memory!();
smbench_group!(benches, fibonacci_20, heap_allocation);
smbench_main!(benches);
