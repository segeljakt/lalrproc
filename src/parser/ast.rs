use crate::span::Span;
pub use proc_macro::Ident as Name;

#[derive(Debug)]
pub enum Exp {
    Var(Span, Name),
    Abs(Span, Name, Box<Exp>),
    App(Span, Box<Exp>, Box<Exp>),
}
