use std::fs;

use spingus::problem;

use rstest::*;

#[rstest]
#[case("barman-agile")]
#[case("barman-satisficing")]
#[case("blocks-typed")]
#[case("blocks-untyped")]
#[case("child-snack-agile")]
#[case("child-snack-satisficing")]
#[case("driverlog-automatic")]
#[case("driverlog-hand-coded")]
#[case("elevator-typed")]
#[case("elevator-untyped")]
#[case("freecell-typed")]
#[case("freecell-untyped")]
#[case("grid")]
#[case("gripper")]
#[case("hiking-sequential-agile")]
#[case("logistics-typed")]
#[case("logistics-untyped")]
#[case("logistics")]
#[case("movie")]
#[case("mystery")]
#[case("satellite")]
fn parse_problem(#[case] domain_name: &str) {
    let problem_path = format!("tests/data/{}/instances/", domain_name);
    let files = fs::read_dir(problem_path).unwrap();
    for file in files {
        if let Ok(content) = fs::read_to_string(file.unwrap().path()) {
            let parse_result = problem::parse_problem(&content);
            if let Ok(problem) = parse_result {
                assert!(!problem.name.is_empty());
            } else if let Err(err) = parse_result {
                panic!(
                    "Could not parse problem: \"{}\".\nWith error: \"{}\"",
                    domain_name, err
                );
            }
        }
    }
}
