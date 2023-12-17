use std::iter;

use criterion::criterion_main;

use rand::Rng;

mod benchmarks;

criterion_main! {
    benchmarks::plan::benches,
}

fn random_name() -> String {
    // no numbers to avoid number first (or - and _)
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    iter::repeat_with(one_char).take(5).collect()
}
