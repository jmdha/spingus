mod token;

use annotate_snippets::{AnnotationType, Renderer, Slice, Snippet, SourceAnnotation};

use logos::{Lexer, Logos};

use crate::shared::{line_num, Result};

use self::token::Token;

/// Denotes a single step in a plan, where a step consists of an action and arguments
pub type Step<'a> = (&'a str, Vec<&'a str>);

/// Denotes a sequence of steps
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

/// Tries to parse a string into a plan
///
/// ## Example
/// ```rust
/// let input = "(p1 o1 o2 o3)";
/// let plan = spingus::plan::try_parse(&input);
/// assert_eq!(plan, Ok(vec![("p1", vec!["o1", "o2", "o3"])]));
/// ```
pub fn try_parse(input: &str) -> Result<Plan> {
    let mut lexer = Token::lexer(input);
    parse_plan(&mut lexer)
}

/// Parses a string into a plan
///
/// Panics in case of a parser error and outputs error to stdout
///
/// ## Example
/// ```rust
/// let input = "(p1 o1 o2 o3)";
/// let plan = spingus::plan::parse(&input);
/// assert_eq!(plan, vec![("p1", vec!["o1", "o2", "o3"])]);
/// ```
pub fn parse(input: &str) -> Plan {
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
