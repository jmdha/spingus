pub mod goal;
pub mod init;
pub mod objects;
mod token;

use annotate_snippets::{AnnotationType, Renderer, Slice, Snippet, SourceAnnotation};

use logos::{Lexer, Logos};

use crate::shared::{line_num, Result};

use self::{
    goal::{parse_goal, Goal},
    init::{parse_init, Init},
    objects::{parse_objects, Objects},
    token::Token,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Problem<'source> {
    pub name: Option<&'source str>,
    pub domain: Option<&'source str>,
    pub objects: Option<Objects<'source>>,
    pub init: Option<Init<'source>>,
    pub goal: Option<Goal<'source>>,
}

fn parse_name<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> Result<&'a str> {
    let name = match lexer.next() {
        Some(token) => match token {
            Ok(Token::Name(name)) => name,
            _ => return Err(("unexpected token".to_owned(), lexer.span())),
        },
        None => return Err(("unexpected end of input".to_owned(), lexer.span())),
    };
    match lexer.next() {
        Some(token) => match token {
            Ok(Token::RParen) => {}
            _ => return Err(("unexpected token".to_owned(), lexer.span())),
        },
        None => return Err(("unexpected end of input".to_owned(), lexer.span())),
    };
    Ok(name)
}

fn parse_problem<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> Result<Problem<'a>> {
    let mut name = None;
    let mut domain = None;
    let mut objects = None;
    let mut init = None;
    let mut goal = None;

    // Handles "(define" in the beginning
    {
        let _ = lexer
            .next()
            .ok_or(("missing opening parenthesis".to_owned(), lexer.span()))?;
        let _ = lexer
            .next()
            .ok_or(("missing opening define".to_owned(), lexer.span()))?;
    }

    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::LParen) => {}
            Ok(Token::RParen) => break,
            Ok(token) => {
                return Err((
                    format!("expected opening parenthesis, found '{}'", token),
                    lexer.span(),
                ))
            }
            _ => return Err(("expected opening parenthesis".to_owned(), lexer.span())),
        }
        let token = lexer
            .next()
            .ok_or(("erranaous opening parenthesis".to_owned(), lexer.span()))?;

        match token {
            Ok(Token::ProblemName) => name = Some(parse_name(lexer)?),
            Ok(Token::DomainName) => domain = Some(parse_name(lexer)?),
            Ok(Token::Objects) => objects = Some(parse_objects(lexer)?),
            Ok(Token::Init) => init = Some(parse_init(lexer)?),
            Ok(Token::Goal) => goal = Some(parse_goal(lexer)?),
            _ => return Err(("unexpected token".to_owned(), lexer.span())),
        }
    }

    Ok(Problem {
        name,
        domain,
        objects,
        init,
        goal,
    })
}

pub fn try_parse(input: &str) -> Result<Problem> {
    let mut lexer = Token::lexer(input);
    parse_problem(&mut lexer)
}

pub fn parse(input: &str) -> Problem {
    match try_parse(input) {
        Ok(problem) => problem,
        Err((msg, span)) => {
            let snippet = Snippet {
                title: None,
                footer: vec![],
                slices: vec![Slice {
                    source: input,
                    line_start: line_num(input, span.start),
                    origin: None,
                    annotations: vec![SourceAnnotation {
                        range: (span.start, span.end),
                        label: &msg,
                        annotation_type: AnnotationType::Error,
                    }],
                    fold: true,
                }],
            };
            panic!("{}", Renderer::styled().render(snippet));
        }
    }
}
