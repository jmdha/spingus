use logos::Lexer;

use crate::shared::Result;

use super::token::Token;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Fact<'a> {
    pub predicate: &'a str,
    pub objects: Vec<&'a str>,
}

pub type Init<'a> = Vec<Fact<'a>>;

fn parse_fact<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> Result<Fact<'a>> {
    let predicate = match lexer.next() {
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

    Ok(Fact { predicate, objects })
}

pub(super) fn parse_init<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> Result<Init<'a>> {
    let mut init = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::LParen) => init.push(parse_fact(lexer)?),
            Ok(Token::RParen) => break,
            _ => return Err(("unexpected token".to_owned(), lexer.span())),
        }
    }

    Ok(init)
}
