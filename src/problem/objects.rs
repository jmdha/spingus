use logos::Lexer;

use crate::shared::Result;

use super::token::Token;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Object<'source> {
    pub name: &'source str,
    pub type_name: Option<&'source str>,
}

pub type Objects<'source> = Vec<Object<'source>>;

pub(super) fn parse_objects<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> Result<Objects<'a>> {
    let mut objects = Vec::new();

    let mut object_names: Vec<&'a str> = Vec::new();
    let mut awaiting_type = false;
    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::Name(name)) => match awaiting_type {
                true => {
                    for object in object_names.iter() {
                        objects.push(Object {
                            name: object,
                            type_name: Some(name),
                        });
                    }
                    object_names.clear();
                    awaiting_type = false;
                }
                false => object_names.push(name),
            },
            Ok(Token::TypeSeparator) => awaiting_type = true,
            Ok(Token::RParen) => break,
            Ok(token) => return Err((format!("unexpected token '{}'", token), lexer.span())),
            _ => return Err(("error".to_string(), lexer.span())),
        }
    }

    match awaiting_type {
        true => return Err(("errant type separator".to_owned(), lexer.span())),
        false => {
            for object in object_names.into_iter() {
                objects.push(Object {
                    name: object,
                    type_name: None,
                });
            }
        }
    }

    Ok(objects)
}
