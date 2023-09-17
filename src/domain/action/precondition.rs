use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{alphanumeric1, char, multispace0, multispace1},
    multi::{many0, many1},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

use crate::shared::{named, spaced};

use super::term::{parse_term, Term};

#[derive(Debug, PartialEq)]
pub enum Precondition {
    Predicate(Term),
    Equal(Preconditions),
    And(Preconditions),
    Or(Preconditions),
    Not(Box<Precondition>),
}
pub type Preconditions = Vec<Precondition>;

fn parse_child(input: &str) -> IResult<&str, Precondition> {
    parse_precondition(input)
}

fn parse_children(input: &str) -> IResult<&str, Preconditions> {
    many1(parse_child)(input)
}

fn parse_predicate(input: &str) -> IResult<&str, Precondition> {
    let (remainder, term) = parse_term(input)?;
    Ok((remainder, Precondition::Predicate(term)))
}

fn parse_equal(input: &str) -> IResult<&str, Precondition> {
    let (remainder, _) = preceded(multispace0, tag_no_case("="))(input)?;
    let (remainder, children) = parse_children(remainder)?;
    Ok((remainder, Precondition::Equal(children)))
}

fn parse_and(input: &str) -> IResult<&str, Precondition> {
    let (remainder, _) = preceded(multispace0, tag_no_case("and"))(input)?;
    let (remainder, children) = parse_children(remainder)?;
    Ok((remainder, Precondition::And(children)))
}

fn parse_or(input: &str) -> IResult<&str, Precondition> {
    let (remainder, _) = preceded(multispace0, tag_no_case("or"))(input)?;
    let (remainder, children) = parse_children(remainder)?;
    Ok((remainder, Precondition::Or(children)))
}

fn parse_not(input: &str) -> IResult<&str, Precondition> {
    let (remainder, _) = preceded(multispace0, tag_no_case("not"))(input)?;
    let (remainder, child) = parse_child(remainder)?;
    Ok((remainder, Precondition::Not(Box::new(child))))
}

pub(super) fn parse_precondition(input: &str) -> IResult<&str, Precondition> {
    delimited(
        spaced(char('(')),
        alt((parse_and, parse_or, parse_not, parse_equal, parse_predicate)),
        spaced(char(')')),
    )(input)
}

#[test]
fn test() {
    assert_eq!(
        Ok((
            "",
            Precondition::Predicate(Term {
                name: "predicate".to_string(),
                parameters: vec![]
            })
        )),
        parse_precondition("(predicate)")
    );
    assert_eq!(
        Ok((
            "",
            Precondition::Predicate(Term {
                name: "predicate".to_string(),
                parameters: vec!["a".to_string()]
            })
        )),
        parse_precondition("(predicate ?a)")
    );
    assert_eq!(
        Ok((
            "",
            Precondition::Predicate(Term {
                name: "predicate".to_string(),
                parameters: vec!["a".to_string(), "b".to_string()]
            })
        )),
        parse_precondition("(predicate ?a ?b)")
    );
    assert_eq!(
        Ok((
            "",
            Precondition::Not(Box::new(Precondition::Predicate(Term {
                name: "predicate".to_string(),
                parameters: vec!["a".to_string()]
            })))
        )),
        parse_precondition("(not (predicate ?a))")
    );
    assert_eq!(
        Ok((
            "",
            Precondition::And(vec![Precondition::Predicate(Term {
                name: "predicate".to_string(),
                parameters: vec!["a".to_string()]
            })])
        )),
        parse_precondition("(and (predicate ?a))")
    );
    assert_eq!(
        Ok((
            "",
            Precondition::And(vec![
                Precondition::Predicate(Term {
                    name: "predicate".to_string(),
                    parameters: vec!["a".to_string()]
                }),
                Precondition::Predicate(Term {
                    name: "predicate".to_string(),
                    parameters: vec!["b".to_string()]
                })
            ])
        )),
        parse_precondition("(and (predicate ?a) (predicate ?b))")
    );
    assert_eq!(
        Ok((
            "",
            Precondition::Or(vec![Precondition::Predicate(Term {
                name: "predicate".to_string(),
                parameters: vec!["a".to_string()]
            })])
        )),
        parse_precondition("(or (predicate ?a))")
    );
    assert_eq!(
        Ok((
            "",
            Precondition::Or(vec![
                Precondition::Predicate(Term {
                    name: "predicate".to_string(),
                    parameters: vec!["a".to_string()]
                }),
                Precondition::Predicate(Term {
                    name: "predicate".to_string(),
                    parameters: vec!["b".to_string()]
                }),
            ])
        )),
        parse_precondition("(or (predicate ?a) (predicate ?b))")
    );
    assert_eq!(
        Ok((
            "",
            Precondition::Equal(vec![
                Precondition::Predicate(Term {
                    name: "predicate".to_string(),
                    parameters: vec!["a".to_string()]
                }),
                Precondition::Predicate(Term {
                    name: "predicate".to_string(),
                    parameters: vec!["b".to_string()]
                }),
            ])
        )),
        parse_precondition("(= (predicate ?a) (predicate ?b))")
    );
}
