use crate::ast::Exp;
use std::fmt::{self, Display};

impl Display for Exp {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exp::Var(.., name) => write!(fmt, "{}", name),
            Exp::Abs(.., name, body) => write!(fmt, "λ{}.{}", name, body),
            Exp::App(.., fun, arg) => write!(fmt, "{} {}", fun, arg),
        }
    }
}
