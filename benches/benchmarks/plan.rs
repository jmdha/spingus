use criterion::{criterion_group, BenchmarkId, Criterion};
use spingus::sas_plan::parse_sas;

use crate::random_name;

fn generate_plan(length: usize) -> String {
    let mut plan = String::new();

    for _ in 0..length {
        plan.push_str("(");

        // action name
        plan.push_str(&random_name());

        // object names
        for _ in 0..3 {
            plan.push_str(&random_name());
        }

        plan.push_str(")");
    }

    plan
}

fn parse_plan(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_plan");
    for length in [1, 10, 100, 1000, 10000].iter() {
        let plan = generate_plan(*length);
        group.throughput(criterion::Throughput::Bytes(plan.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(length),
            length,
            |b, _: &usize| {
                b.iter(|| parse_sas(&plan));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, parse_plan);
