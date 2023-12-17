use criterion::{criterion_group, Criterion};
use spingus::problem::parse_problem;

pub const PROBLEM: &'static str = r#"
    (define (problem get-paid)
        (:domain briefcase-world)
        (:init
               ; types: locations
               (place home) (place office)
               ; types: objects
               (object p) (object d) (object b)
               ; setup
               (at B home) (at P home) (at D home) (in P))
        (:goal (and (at B office) (at D office) (at P home)))
    )
    "#;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse-problem");
    group.throughput(criterion::Throughput::Bytes(PROBLEM.len() as u64));
    group.bench_function("parse-problem", |b| b.iter(|| parse_problem(PROBLEM)));
    group.finish();
}

criterion_group!(benches, bench);
