#![feature(proc_macro_span)]
#![feature(proc_macro_diagnostic)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::empty_enum,
    clippy::match_same_arms,
    clippy::module_name_repetitions,
    clippy::use_self
)]

extern crate proc_macro;
use lalrpop_util::lalrpop_mod;
mod ast;
mod cursor;
mod error;
mod exp;
mod span;
mod token;
lalrpop_mod!(parse);

use crate::cursor::Cursor;
use crate::error::NoUserError;
use crate::parse::ExpParser;
use crate::span::Span;
use crate::token::Token;
use lalrpop_util::ParseError;
use proc_macro::{Delimiter, Group, Literal, TokenStream, TokenTree};
use std::iter::{self, FromIterator};

#[proc_macro]
pub fn exp(input: TokenStream) -> TokenStream {
    let tt = match ExpParser::new().parse(Cursor::new(input)) {
        Ok(ast) => TokenTree::Literal(Literal::string(&ast.to_string())),
        Err(err) => {
            error::emit(err);
            TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::new()))
        }
    };
    TokenStream::from_iter(iter::once(tt))
}
