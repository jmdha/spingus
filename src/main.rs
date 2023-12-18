use std::fs;

use spingus::problem;

fn main() {
    let content = fs::read_to_string(
        "/home/jamadaha/GitHub/spingus/tests/data/elevator-untyped/instances/instance-1.pddl",
    )
    .unwrap();
    problem::parse(&content);
}
