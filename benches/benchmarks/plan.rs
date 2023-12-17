use criterion::{criterion_group, BenchmarkId, Criterion};
use spingus::plan;

pub const PLAN: &'static str = r#"
    (abcde abcde abcde)
    (abcde abcde abcde)
    (abcde abcde abcde)
    (abcde abcde abcde)
    (abcde abcde abcde)
    (abcde abcde abcde)
    (abcde abcde abcde)
    (abcde abcde abcde)
    (abcde abcde abcde)
    (abcde abcde abcde)
    (abcde abcde abcde)
    (abcde abcde abcde)
    "#;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse-plan");
    group.throughput(criterion::Throughput::Bytes(PLAN.len() as u64));
    group.bench_function("parse-plan", |b| b.iter(|| plan::parse(PLAN)));
    group.finish();
}

criterion_group!(benches, bench);
