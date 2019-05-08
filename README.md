# Result

Benchmark Results

```rust
(rust:nightly) ↪ cargo  bench
    Finished release [optimized] target(s) in 0.01s
     Running target/release/deps/cargo_bench_example-17c8978f50aea212

running 3 tests
test tests::benchmark_first  ... bench:     145,077 ns/iter (+/- 15,543)
test tests::benchmark_second ... bench:     141,911 ns/iter (+/- 21,191)
test tests::benchmark_third  ... bench:     426,223 ns/iter (+/- 78,973)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out
```