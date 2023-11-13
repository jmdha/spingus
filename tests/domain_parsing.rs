use std::fs;

use spingus::domain;

use rstest::*;

#[rstest]
#[case("barman-agile")]
#[case("barman-mco14-strips")]
#[case("barman-satisficing")]
#[case("blocks-typed")]
#[case("blocks-untyped")]
#[case("childsnack")]
#[case("child-snack-agile")]
#[case("child-snack-satisficing")]
#[case("driverlog-automatic")]
#[case("driverlog-hand-coded")]
#[case("elevator-typed")]
#[case("elevator-untyped")]
#[case("ferry")]
#[case("floortile")]
#[case("freecell-typed")]
#[case("freecell-untyped")]
#[case("grid")]
#[case("gripper")]
#[case("hiking-sequential-agile")]
#[case("logistics")]
#[case("logistics-typed")]
#[case("logistics-untyped")]
#[case("miconic")]
#[case("movie")]
#[case("mystery")]
#[case("rovers")]
#[case("satellite")]
#[case("sokoban")]
#[case("spanner")]
#[case("storage")]
#[case("transport")]
#[case("zenotravel")]
fn parse_domain(#[case] domain_name: &str) {
    if let Ok(str) = fs::read_to_string(format!("tests/data/{}/domain.pddl", domain_name)) {
        let parse_result = domain::parse_domain(&str);
        if let Ok(dom) = parse_result {
            assert!(!dom.name.is_empty());
        } else if let Err(err) = parse_result {
            panic!(
                "Could not parse domain: \"{}\".\nWith error: \"{}\"",
                domain_name, err
            );
        }
    } else {
        panic!("Could not open");
    }
}
