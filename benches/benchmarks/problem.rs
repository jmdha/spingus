use criterion::{criterion_group, Criterion};
use spingus::problem;

pub const PROBLEM: &'static str = r#"
    (define (problem p1)
        (:domain d1)
        (:objects 
            obj1 obj2 obj3 - type1
            obj4 obj5 obj6 - type2
        )
        (:init
               (pred1 obj1) (pred1 obj2) (pred1 obj3)
               (pred2 obj1 obj4) (pred2 obj2 obj5) (pred3 obj3 obj6)
        )
        (:goal 
            (and 
                (pred2 obj1 obj5) 
                (pred2 obj2 obj6) 
                (pred2 obj3 obj4)
            )
        )
    )
"#;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse-problem");
    group.throughput(criterion::Throughput::Bytes(PROBLEM.len() as u64));
    group.bench_function("parse-problem", |b| b.iter(|| problem::parse(PROBLEM)));
    group.finish();
}

criterion_group!(benches, bench);
