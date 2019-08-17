use proc_macro::{Delimiter, Ident, Literal};
use std::fmt::{self, Display};

#[derive(Clone, Debug)]
pub enum Token {
    Open(Delimiter),
    Close(Delimiter),
    Punct(char),
    Joint,
    Ident(Ident),
    Literal(Literal),
    Keyword(Keyword),
}

impl Display for Token {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Open(Delimiter::Parenthesis) => formatter.write_str("`(`"),
            Token::Open(Delimiter::Brace) => formatter.write_str("`{`"),
            Token::Open(Delimiter::Bracket) => formatter.write_str("`[`"),
            Token::Open(Delimiter::None) => formatter.write_str("None-delimiter"),
            Token::Close(Delimiter::Parenthesis) => formatter.write_str("`)`"),
            Token::Close(Delimiter::Brace) => formatter.write_str("`}`"),
            Token::Close(Delimiter::Bracket) => formatter.write_str("`]`"),
            Token::Close(Delimiter::None) => formatter.write_str("None-delimiter"),
            Token::Punct(ch) => write!(formatter, "`{}`", ch),
            Token::Joint => formatter.write_str("joint-op"),
            Token::Ident(ref ident) => write!(formatter, "`{}`", ident),
            Token::Literal(ref lit) => write!(formatter, "`{}`", lit),
            Token::Keyword(ref kw) => write!(formatter, "{}", kw),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Keyword {
    Lambda,
}

impl Display for Keyword {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Keyword::Lambda => write!(formatter, "`λ`"),
        }
    }
}
