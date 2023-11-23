use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1},
    multi::{fold_many0, many1, separated_list1},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

use crate::shared::{named, spaced};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Parameter {
    Untyped {
        name: String,
    },
    Typed {
        name: String,
        type_name: String,
    },
    Either {
        name: String,
        type_names: Vec<String>,
    },
}

pub type Parameters = Vec<Parameter>;
fn parse_either(input: &str) -> IResult<&str, Parameters> {
    let (remainder, parameters) = separated_pair(
        separated_list1(multispace0, named),
        delimited(multispace0, char('-'), multispace0),
        delimited(
            char('('),
            preceded(tag("either "), separated_list1(multispace1, named)),
            char(')'),
        ),
    )(input)?;
    Ok((
        remainder,
        parameters
            .0
            .iter()
            .map(|parameter| Parameter::Either {
                name: parameter.to_string(),
                type_names: parameters.1.clone(),
            })
            .collect(),
    ))
}
fn parse_typed(input: &str) -> IResult<&str, Parameters> {
    let (remainder, parameters) = separated_pair(
        separated_list1(multispace0, named),
        delimited(multispace0, char('-'), multispace0),
        named,
    )(input)?;
    Ok((
        remainder,
        parameters
            .0
            .iter()
            .map(|parameter| Parameter::Typed {
                name: parameter.to_string(),
                type_name: parameters.1.to_string(),
            })
            .collect(),
    ))
}

fn parse_untyped(input: &str) -> IResult<&str, Parameters> {
    let (remainder, parameters) = many1(preceded(multispace0, named))(input)?;

    Ok((
        remainder,
        parameters
            .iter()
            .map(|parameter| Parameter::Untyped {
                name: parameter.to_string(),
            })
            .collect(),
    ))
}

pub(super) fn parse_parameters(input: &str) -> IResult<&str, Parameters> {
    fold_many0(
        spaced(alt((parse_either, parse_typed, parse_untyped))),
        Vec::new,
        |mut acc: Vec<_>, mut item| {
            acc.append(&mut item);
            acc
        },
    )(input)
}

pub fn parameters_to_string(parameters: &Parameters) -> String {
    let mut s: String = " ".to_string();

    for parameter in parameters {
        let parameter_s;
        match parameter {
            Parameter::Untyped { name } => {
                parameter_s = name.to_string();
            }
            Parameter::Typed { name, type_name } => {
                parameter_s = format!("{} - {}", name, type_name);
            }
            Parameter::Either { name, type_names } => {
                let mut s = "".to_string();
                type_names
                    .iter()
                    .for_each(|n| s.push_str(&format!(" {}", n)));
                parameter_s = format!("{} - (either {})", name, format!("(either{})", s));
            }
        }
        s.push_str(&parameter_s);
        s.push_str(" ");
    }

    s
}

#[test]
fn test() {
    assert_eq!(
        Ok((
            "",
            vec![Parameter::Untyped {
                name: "?p".to_string()
            }]
        )),
        parse_parameters("?p")
    );
    assert_eq!(
        Ok((
            "",
            vec![
                Parameter::Untyped {
                    name: "?p1".to_string()
                },
                Parameter::Untyped {
                    name: "?p2".to_string()
                }
            ]
        )),
        parse_parameters("?p1 ?p2")
    );
    assert_eq!(
        Ok((
            "",
            vec![Parameter::Typed {
                name: "?p".to_string(),
                type_name: "type".to_string()
            }]
        )),
        parse_parameters("?p - type")
    );
    assert_eq!(
        Ok((
            "",
            vec![
                Parameter::Typed {
                    name: "?p1".to_string(),
                    type_name: "type".to_string()
                },
                Parameter::Typed {
                    name: "?p2".to_string(),
                    type_name: "type".to_string()
                }
            ]
        )),
        parse_parameters("?p1 ?p2 - type")
    );
    assert_eq!(
        Ok((
            "",
            vec![
                Parameter::Typed {
                    name: "?p1".to_string(),
                    type_name: "type".to_string()
                },
                Parameter::Typed {
                    name: "?p2".to_string(),
                    type_name: "type".to_string()
                }
            ]
        )),
        parse_parameters("?p1 - type ?p2 - type")
    );
    assert_eq!(
        Ok((
            "",
            vec![
                Parameter::Typed {
                    name: "?p1".to_string(),
                    type_name: "type1".to_string()
                },
                Parameter::Typed {
                    name: "?p2".to_string(),
                    type_name: "type2".to_string()
                }
            ]
        )),
        parse_parameters("?p1 - type1 ?p2 - type2")
    );
    assert_eq!(
        Ok((
            "",
            vec![
                Parameter::Typed {
                    name: "?p1".to_string(),
                    type_name: "type".to_string()
                },
                Parameter::Untyped {
                    name: "?p2".to_string(),
                }
            ]
        )),
        parse_parameters("?p1 - type ?p2")
    );
    assert_eq!(
        Ok((
            "",
            vec![Parameter::Either {
                name: "?p1".to_string(),
                type_names: vec!["type_a".to_string(), "type_b".to_string()]
            }]
        )),
        parse_parameters("?p1 - (either type_a type_b)")
    );
}
