# Result

Benchmark Results

```rust
(rust:nightly) ↪ cargo bench
    Finished release [optimized] target(s) in 0.01s
     Running target/release/deps/cargo_bench_example-05c7587ebad02c36

running 4 tests
test tests::benchmark_first  ... bench:     145,505 ns/iter (+/- 17,518)
test tests::benchmark_fourth ... bench:      13,395 ns/iter (+/- 1,268)
test tests::benchmark_second ... bench:     141,733 ns/iter (+/- 10,969)
test tests::benchmark_third  ... bench:     419,145 ns/iter (+/- 67,178)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out
```