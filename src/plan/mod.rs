mod token;

use logos::{Lexer, Logos};

use crate::shared::Result;

use self::token::Token;

pub type Step<'a> = (&'a str, Vec<&'a str>);
pub type Plan<'a> = Vec<Step<'a>>;

fn parse_step<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> Result<Step<'a>> {
    let action = match lexer.next() {
        Some(token) => match token {
            Ok(Token::Name(name)) => name,
            _ => return Err(("unexpected token".to_owned(), lexer.span())),
        },
        None => return Err(("unexpected end of input".to_owned(), lexer.span())),
    };
    let mut objects = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::Name(name)) => objects.push(name),
            Ok(Token::RParen) => break,
            _ => return Err(("unexpected token".to_owned(), lexer.span())),
        }
    }

    Ok((action, objects))
}

fn parse_plan<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> Result<Plan<'a>> {
    let mut steps = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::LParen) => steps.push(parse_step(lexer)?),
            _ => return Err(("unexpected token".to_owned(), lexer.span())),
        }
    }

    Ok(steps)
}

pub fn try_parse(input: &str) -> Result<Plan> {
    let mut lexer = Token::lexer(input);
    parse_plan(&mut lexer)
}

pub fn parse(input: &str) -> Plan {
    try_parse(input).unwrap()
}

#[cfg(test)]
mod test {
    use crate::plan::parse;

    #[test]
    fn plan_parse() {
        assert_eq!(parse("(a)"), vec![("a", vec![])]);
        assert_eq!(parse("(a b c)"), vec![("a", vec!["b", "c"])]);
        assert_eq!(parse("(a)(b)"), vec![("a", vec![]), ("b", vec![])]);
        assert_eq!(parse("(a)\n(b)"), vec![("a", vec![]), ("b", vec![])]);
        assert_eq!(
            parse("(a b)(c d)"),
            vec![("a", vec!["b"]), ("c", vec!["d"])]
        );
    }
}
