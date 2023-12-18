use core::fmt;

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\r\n\f]+|;.*")]
pub(super) enum Token<'a> {
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[regex("(?i)define")]
    Definition,

    #[regex("(?i)problem")]
    ProblemName,

    #[regex(":(?i)domain")]
    DomainName,

    #[regex(":(?i)objects")]
    Objects,

    #[regex(":(?i)init")]
    Init,

    #[regex(":(?i)goal")]
    Goal,

    #[regex("(?i)and")]
    And,

    #[regex("(?i)or")]
    Or,

    #[regex("(?i)not")]
    Not,

    #[token("-")]
    TypeSeparator,

    #[regex("[a-zA-Z][a-zA-Z0-9-_]*")]
    Name(&'a str),
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Definition => write!(f, "DEFINITION"),
            Token::ProblemName => write!(f, "PROBLEM_NAME"),
            Token::DomainName => write!(f, "DOMAIN_NAME"),
            Token::Objects => write!(f, "OBJECT_LIST"),
            Token::Init => write!(f, "INIT_LIST"),
            Token::Goal => write!(f, "GOAL_CONDITION"),
            Token::And => write!(f, "AND"),
            Token::Or => write!(f, "OR"),
            Token::Not => write!(f, "NOT"),
            Token::TypeSeparator => write!(f, "TYPE_SEPARATOR"),
            Token::Name(name) => write!(f, "{}", name),
        }
    }
}
