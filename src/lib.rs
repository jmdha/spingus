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
pub mod problem;
mod shared;
mod term;
