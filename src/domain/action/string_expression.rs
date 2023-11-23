use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{char, multispace0},
    multi::{many0, many1},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

use crate::{
    shared::{named, spaced},
    term::{parse_term, Term},
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum StringExpression {
    Predicate(Term),
    Equal(Vec<String>),
    And(StringExpressions),
    Or(StringExpressions),
    Not(Box<StringExpression>),
    Imply(Box<StringExpression>, Box<StringExpression>),
}
pub type StringExpressions = Vec<StringExpression>;
fn parse_predicate(input: &str) -> IResult<&str, StringExpression> {
    let (remainder, term) = parse_term(input)?;
    Ok((remainder, StringExpression::Predicate(term)))
}

fn parse_equal(input: &str) -> IResult<&str, StringExpression> {
    let (remainder, _) = preceded(multispace0, tag_no_case("="))(input)?;
    println!("{}", remainder);
    let (remainder, children) = many1(preceded(multispace0, named))(remainder)?;
    Ok((remainder, StringExpression::Equal(children)))
}

fn parse_and(input: &str) -> IResult<&str, StringExpression> {
    let (remainder, _) = preceded(multispace0, tag_no_case("and"))(input)?;
    let (remainder, children) = many0(parse_expression)(remainder)?;
    Ok((remainder, StringExpression::And(children)))
}

fn parse_or(input: &str) -> IResult<&str, StringExpression> {
    let (remainder, _) = preceded(multispace0, tag_no_case("or"))(input)?;
    let (remainder, children) = many1(parse_expression)(remainder)?;
    Ok((remainder, StringExpression::Or(children)))
}
fn parse_not(input: &str) -> IResult<&str, StringExpression> {
    let (remainder, _) = preceded(multispace0, tag_no_case("not"))(input)?;
    let (remainder, child) = parse_expression(remainder)?;
    Ok((remainder, StringExpression::Not(Box::new(child))))
}
fn parse_imply(input: &str) -> IResult<&str, StringExpression> {
    let (remainder, _) = preceded(multispace0, tag_no_case("imply"))(input)?;
    let (remainder, (antecendent, consequent)) =
        separated_pair(parse_expression, multispace0, parse_expression)(remainder)?;
    Ok((
        remainder,
        StringExpression::Imply(Box::new(antecendent), Box::new(consequent)),
    ))
}

pub(super) fn parse_expression(input: &str) -> IResult<&str, StringExpression> {
    delimited(
        spaced(char('(')),
        alt((
            parse_and,
            parse_or,
            parse_not,
            parse_equal,
            parse_imply,
            parse_predicate,
        )),
        spaced(char(')')),
    )(input)
}

#[test]
fn test() {
    assert_eq!(
        Ok((
            "",
            StringExpression::Predicate(Term {
                name: "predicate".to_string(),
                parameters: vec![]
            })
        )),
        parse_expression("(predicate)")
    );
    assert_eq!(
        Ok((
            "",
            StringExpression::Predicate(Term {
                name: "predicate".to_string(),
                parameters: vec!["?a".to_string()]
            })
        )),
        parse_expression("(predicate ?a)")
    );
    assert_eq!(
        Ok((
            "",
            StringExpression::Predicate(Term {
                name: "predicate".to_string(),
                parameters: vec!["?a".to_string(), "?b".to_string()]
            })
        )),
        parse_expression("(predicate ?a ?b)")
    );
    assert_eq!(
        Ok((
            "",
            StringExpression::Not(Box::new(StringExpression::Predicate(Term {
                name: "predicate".to_string(),
                parameters: vec!["?a".to_string()]
            })))
        )),
        parse_expression("(not (predicate ?a))")
    );
    assert_eq!(
        Ok((
            "",
            StringExpression::And(vec![StringExpression::Predicate(Term {
                name: "predicate".to_string(),
                parameters: vec!["?a".to_string()]
            })])
        )),
        parse_expression("(and (predicate ?a))")
    );
    assert_eq!(
        Ok((
            "",
            StringExpression::And(vec![
                StringExpression::Predicate(Term {
                    name: "predicate".to_string(),
                    parameters: vec!["?a".to_string()]
                }),
                StringExpression::Predicate(Term {
                    name: "predicate".to_string(),
                    parameters: vec!["?b".to_string()]
                })
            ])
        )),
        parse_expression("(and (predicate ?a) (predicate ?b))")
    );
    assert_eq!(
        Ok((
            "",
            StringExpression::Or(vec![StringExpression::Predicate(Term {
                name: "predicate".to_string(),
                parameters: vec!["?a".to_string()]
            })])
        )),
        parse_expression("(or (predicate ?a))")
    );
    assert_eq!(
        Ok((
            "",
            StringExpression::Or(vec![
                StringExpression::Predicate(Term {
                    name: "predicate".to_string(),
                    parameters: vec!["?a".to_string()]
                }),
                StringExpression::Predicate(Term {
                    name: "predicate".to_string(),
                    parameters: vec!["?b".to_string()]
                }),
            ])
        )),
        parse_expression("(or (predicate ?a) (predicate ?b))")
    );
    assert_eq!(
        Ok((
            "",
            StringExpression::Equal(vec!["?a".to_string(), "?b".to_string(),])
        )),
        parse_expression("(= ?a ?b)")
    );
    assert_eq!(
        Ok((
            "",
            StringExpression::Imply(
                Box::new(StringExpression::Predicate(Term {
                    name: "predicate".to_string(),
                    parameters: vec!["?a".to_string()]
                })),
                Box::new(StringExpression::Predicate(Term {
                    name: "predicate".to_string(),
                    parameters: vec!["?b".to_string()]
                }))
            )
        )),
        parse_expression("(imply (predicate ?a) (predicate ?b))")
    );
    assert_eq!(
        Ok((
            "",
            StringExpression::Imply(
                Box::new(StringExpression::Predicate(Term {
                    name: "closer".to_string(),
                    parameters: vec!["?a2".to_string(), "?a1".to_string()]
                })),
                Box::new(StringExpression::Predicate(Term {
                    name: "free".to_string(),
                    parameters: vec!["?a2".to_string(), "?t".to_string()]
                }))
            )
        )),
        parse_expression("(imply (closer ?a2 ?a1) (free ?a2 ?t))")
    );
}
