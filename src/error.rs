use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Error, PartialEq)]
pub enum PettyParseError {
    Node,
    Expr,
    Ident,
    Literal,
    Float,
    FloatDigit,
    TermExpr,
}

impl fmt::Display for PettyParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
