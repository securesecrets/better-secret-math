use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::rebase::benches,
    benchmarks::core::benches,
}
