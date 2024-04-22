use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::detailer_bench::benches,
}
