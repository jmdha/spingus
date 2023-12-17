use std::iter;

use criterion::criterion_main;

use rand::Rng;

mod benchmarks;

criterion_main! {
    benchmarks::plan::benches,
    benchmarks::problem::benches
}
