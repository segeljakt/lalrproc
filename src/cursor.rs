use crate::span::Span;
use crate::token::{Keyword, Token};
use proc_macro::token_stream::IntoIter as TokenIter;
use proc_macro::{self, Delimiter, Ident, Punct, Spacing, TokenStream, TokenTree};
use std::iter::Peekable;

pub struct Cursor {
    stack: Vec<Frame>,
    joint: Option<Span>,
    tail: Option<Ident>,
}

struct Frame {
    iter: Peekable<TokenIter>,
    span: Span,
    delimiter: Delimiter,
}

impl Frame {
    fn new(stream: TokenStream, span: Span, delimiter: Delimiter) -> Self {
        Frame {
            iter: stream.into_iter().peekable(),
            span,
            delimiter,
        }
    }
}

impl Cursor {
    pub fn new(input: TokenStream) -> Self {
        Cursor {
            stack: vec![Frame::new(input, Span::default(), Delimiter::None)],
            joint: None,
            tail: None,
        }
    }
    /// Drop all tokens on the specified line
    fn strip_line_comment(&mut self) {
        if let Some(mut top) = self.stack.pop() {
            if let Some(TokenTree::Punct(tt)) = top.iter.peek() {
                if is_comment(tt) {
                    let mut line = tt.span().start().line;
                    top.iter.next();
                    self.stack.push(top);
                    while let Some(mut top) = self.stack.pop() {
                        while let Some(tt) = top.iter.peek() {
                            if tt.span().start().line == line {
                                top.iter.next();
                            } else if let TokenTree::Punct(tt) = tt {
                                if is_comment(tt) {
                                    line = tt.span().start().line;
                                    top.iter.next();
                                } else {
                                    self.stack.push(top);
                                    return;
                                }
                            } else {
                                self.stack.push(top);
                                return;
                            }
                        }
                    }
                } else {
                    self.stack.push(top);
                }
            } else {
                self.stack.push(top);
            }
        }
    }
}

impl Iterator for Cursor {
    type Item = (Span, Token, Span);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(span) = self.joint.take() {
            Some((span, Token::Joint, span))
        } else if let Some(tail) = self.tail.take() {
            let span = Span(tail.span());
            Some((span, Token::Ident(tail), span))
        } else {
            self.strip_line_comment();
            let mut top = self.stack.pop()?;
            if let Some(tt) = top.iter.next() {
                let span = Span(tt.span());
                self.stack.push(top);
                let token = match tt {
                    TokenTree::Group(group) => {
                        let delimiter = group.delimiter();
                        self.stack.push(Frame::new(group.stream(), span, delimiter));
                        Token::Open(delimiter)
                    }
                    TokenTree::Punct(punct) => {
                        if let Spacing::Joint = punct.spacing() {
                            self.joint = Some(span);
                        }
                        Token::Punct(punct.as_char())
                    }
                    TokenTree::Ident(ident) => {
                        if let Some((kw, tail)) = keyword(&ident) {
                            self.tail = tail;
                            Token::Keyword(kw)
                        } else {
                            Token::Ident(ident)
                        }
                    }
                    TokenTree::Literal(lit) => Token::Literal(lit),
                };
                Some((span, token, span))
            } else if self.stack.is_empty() {
                None
            } else {
                Some((top.span, Token::Close(top.delimiter), top.span))
            }
        }
    }
}

/// Tries to convert ident into a keyword
/// Certain keywords can be followed by others
fn keyword(ident: &Ident) -> Option<(Keyword, Option<Ident>)> {
    let span = ident.span();
    let s = ident.to_string();
    match s.chars().next() {
        Some(c) => {
            let kw = match c {
                'λ' => Keyword::Lambda,
                _ => None?,
            };
            let tail = &s[c.len_utf8()..];
            if tail.is_empty() {
                Some((kw, None))
            } else {
                Some((kw, Some(Ident::new(&tail, span))))
            }
        }
        None => None,
    }
}

fn is_comment(punct: &Punct) -> bool {
    punct.as_char() == '#'
}
