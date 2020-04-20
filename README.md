# Welcome to smbench 👋
[![Build Status](https://travis-ci.org/Kogia-sima/smbench.svg?branch=master)](https://travis-ci.org/Kogia-sima/smbench)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/Kogia-sima/smbench/blob/master/LICENSE)
[![Twitter: Kogia\_sima](https://img.shields.io/twitter/follow/Kogia\_sima.svg?style=social)](https://twitter.com/Kogia\_sima)

lightweight benchmark framework for Rust

## Features

- Very small overhead
- Estimate memory usage
- Lightweight (does not depend on a heavy crate)
- Highly extensive and customizable

## Usage

#### 1. Add smbench in your Cargo.toml

```toml
[dev-dependencies]
smbench = { git = "https://github.com/Kogia-sima/smbench" }
```

#### 2. Add source code for benchmark in `benches/` directory.

```rust
// benches/example.rs

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

// Define global allocator to trace memory allocation
smbench_trace_memory!();

smbench_group!(benchmark, fibonacci_20, heap_allocation);
smbench_main!(benchmark);
```

#### 3. Setup your `Cargo.toml` such that cargo is able to execute the benchmark

```toml
[[bench]]
name = "example"
harness = false
```

#### 4. Run benchmark from console

```console
$ cargo bench -- --benchmem
    Finished bench [optimized] target(s) in 0.02s
     Running target/release/deps/example-a698a4f124b5c06c
OS Type: linux
CPU Architecture: x86_64
CPU Model Name: Intel(R) Core(TM) i5-8265U CPU @ 1.60GHz
SMBench Version: 0.1.0

# benchmark (benches/example.rs)
Benchmark              Time                  95% CI         Allocation
----------------------------------------------------------------------
fibonacci_20      9.9045 ns  [9.8952 ns, 9.9138 ns]     0 B (0 allocs)
heap_allocation   34.422 ns  [34.234 ns, 34.610 ns]    40 B (1 allocs)
```

## Run tests

```sh
cargo test
```

## Author

👤 **Kogia sima**

* Twitter: [@Kogia\_sima](https://twitter.com/Kogia\_sima)
* Github: [@Kogia-sima](https://github.com/Kogia-sima)

## 🤝 Contributing

Contributions, issues and feature requests are welcome!

Feel free to check [issues page](https://github.com/Kogia-sima/smbench/issues). 

## Show your support

Give a ⭐️ if this project helped you!


## 📝 License

Copyright © 2020 [Ryohei Machida](https://github.com/Kogia-sima).

This project is [MIT](https://github.com/Kogia-sima/smbench/blob/master/LICENSE) licensed.

***
_This README was generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_
