use logos::Lexer;

use crate::shared::Result;

use super::token::Token;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Goal<'a> {
    Fact {
        predicate: &'a str,
        objects: Vec<&'a str>,
    },
    Not(Box<Goal<'a>>),
    And(Vec<Goal<'a>>),
    Or(Vec<Goal<'a>>),
}

//  NOTE: assumes opening bracket '(' is consumed
fn parse_expression<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> Result<Goal<'a>> {
    let token = lexer
        .next()
        .ok_or(("unexpected end of input".to_owned(), lexer.span()))?;
    match token {
        Ok(Token::Name(name)) => {
            let mut objects = Vec::new();

            while let Some(token) = lexer.next() {
                match token {
                    Ok(Token::Name(name)) => objects.push(name),
                    Ok(Token::RParen) => break,
                    _ => return Err(("unexpected token".to_owned(), lexer.span())),
                }
            }

            return Ok(Goal::Fact {
                predicate: name,
                objects,
            });
        }
        Ok(Token::Not) => {
            let n_token = lexer
                .next()
                .ok_or(("unexpected end of input".to_owned(), lexer.span()))?;
            let expression = match n_token {
                Ok(Token::LParen) => parse_expression(lexer),
                _ => return Err(("unexpected token".to_owned(), lexer.span())),
            }?;
            return Ok(Goal::Not(Box::new(expression)));
        }
        Ok(Token::And) => {
            let mut expressions = Vec::new();

            while let Some(token) = lexer.next() {
                match token {
                    Ok(Token::RParen) => return Ok(Goal::And(expressions)),
                    Ok(Token::LParen) => expressions.push(parse_expression(lexer)?),
                    _ => return Err(("unexpected token".to_owned(), lexer.span())),
                }
            }

            return Err(("unexpected end of input".to_owned(), lexer.span()));
        }
        Ok(Token::Or) => {
            let mut expressions = Vec::new();

            while let Some(token) = lexer.next() {
                match token {
                    Ok(Token::RParen) => return Ok(Goal::Or(expressions)),
                    Ok(Token::LParen) => expressions.push(parse_expression(lexer)?),
                    _ => return Err(("unexpected token".to_owned(), lexer.span())),
                }
            }

            return Err(("unexpected end of input".to_owned(), lexer.span()));
        }
        _ => return Err(("unexpected token".to_owned(), lexer.span())),
    }
}

pub(super) fn parse_goal<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> Result<Goal<'a>> {
    match lexer.next() {
        Some(token) => match token {
            Ok(Token::LParen) => {}
            _ => return Err(("unexpected token".to_owned(), lexer.span())),
        },
        None => return Err(("unexpected end of input".to_owned(), lexer.span())),
    };

    let goal = parse_expression(lexer)?;

    match lexer.next() {
        Some(token) => match token {
            Ok(Token::RParen) => {}
            _ => return Err(("unexpected token".to_owned(), lexer.span())),
        },
        None => return Err(("unexpected end of input".to_owned(), lexer.span())),
    };
    Ok(goal)
}

#[cfg(test)]
mod test {
    use logos::Logos;

    use crate::problem::{
        goal::{parse_expression, parse_goal, Goal},
        token::Token,
    };

    use rstest::*;

    #[rstest]
    #[case("a)", Goal::Fact { predicate: "a", objects: vec![] })]
    #[case("a b c)", Goal::Fact { predicate: "a", objects: vec!["b", "c"] })]
    fn fact_parse(#[case] input: &str, #[case] expected: Goal) {
        let mut lexer = Token::lexer(input);
        assert_eq!(parse_expression(&mut lexer), Ok(expected));
    }

    #[rstest]
    #[case("not (a))", Goal::Not(Box::new(Goal::Fact { predicate: "a", objects: vec![] })))]
    fn not_parse(#[case] input: &str, #[case] expected: Goal) {
        let mut lexer = Token::lexer(input);
        assert_eq!(parse_expression(&mut lexer), Ok(expected));
    }

    #[rstest]
    #[case("(a))", Goal::Fact { predicate: "a", objects: vec![] })]
    #[case("(and (a) (b)))", Goal::And(vec![Goal::Fact { predicate: "a", objects: vec![] }, Goal::Fact { predicate: "b", objects: vec![] }]))]
    fn goal_parse(#[case] input: &str, #[case] expected: Goal) {
        let mut lexer = Token::lexer(input);
        assert_eq!(parse_goal(&mut lexer), Ok(expected));
    }
}
