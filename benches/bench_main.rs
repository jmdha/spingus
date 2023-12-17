use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::plan::benches,
    benchmarks::problem::benches,
    benchmarks::domain::benches
}
