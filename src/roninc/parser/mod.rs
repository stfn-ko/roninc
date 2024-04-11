use super::lexer::Tokens;
use core::fmt;
use std::array;

pub struct AST {}

pub struct SyntaxError {}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "syntax error")
    }
}

pub(crate) fn emit_ast(tokens: Tokens) -> Result<AST, SyntaxError> {
    for t in tokens {
        println!("{}", t.kind)
    }

    Ok(AST {})
}
