use criterion::{criterion_group, Criterion};
use spingus::domain::parse_domain;

pub const DOMAIN: &'static str = r#"
    (define (domain d1)
        (:requirements :typing)
        (:types
            type1 type2 - object
        )
        (:predicates
            (pred1 ?obj - type1)
            (pred2 ?obj1 - type1 ?obj2 - type2)
        )
        (:action action1
            :parameters (?p1 - type1 ?p2 - type2)
            :effect 
                (and
                    (pred2 ?p1 ?p2)
                )
        )
    )
"#;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse-domain");
    group.throughput(criterion::Throughput::Bytes(DOMAIN.len() as u64));
    group.bench_function("parse-domain", |b| b.iter(|| parse_domain(DOMAIN)));
    group.finish();
}

criterion_group!(benches, bench);
