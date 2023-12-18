//! # Spingus
//!
//! _A simple, fast, and robust PDDL parser_
//!
//! **Spingus** has three goals:
//!
//! + *Robust* - Is tested on commenly used PDDL domains and problems
//! + *Fast* - Optimised and benchmarked to be as fast as possible
//! + *Simple* - Avoids rarely used PDDL syntax in favor of easy of use
//!
//! ## Example
//! ```rust
//! use spingus::problem::objects::Object;
//! let input = "(define (problem prob)
//!                   (:objects o1)
//!                   (:init (p o1))
//!                   (:goal (not (p o3)))
//!              )";
//! let problem = spingus::problem::parse(&input);
//! assert_eq!(problem.name, Some("prob"));
//! assert_eq!(problem.domain, None);
//! assert_eq!(problem.objects, Some(vec![Object { name: "o1", type_name: None }]));
//! //...
//! ```
//!
//! ## Benchmark
//! Benchmarked on a i5-13600k with [Criterion](https://github.com/bheisler/criterion.rs)
//!
//! |               | Throughput | Time to Parse (see [here](https://github.com/jamadaha/spingus/tree/master/benches/benchmarks)) |
//! |---------------|------------|---------------|
//! | Domain        | 96 MiB/s   |   4 µs |
//! | Problem       | 726 MiB/s  | 726 ns |
//! | Plan          | 727 MiB/s  | 378 ns |
//!

/// Contains things related to PDDL domain files
pub mod domain;

/// Contains things related to PDDL plan files
///
/// These files are used as a solution to PDDL domain + problem
///
/// ## Example plan file
///
/// (action1 obj1 obj2)
///
/// (action1 obj2 obj1)
///
/// ; cost = 2 (unit cost)
///
pub mod plan;

/// Contains things related to PDDL problem files
pub mod problem;

mod shared;
mod term;
