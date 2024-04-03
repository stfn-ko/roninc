use core::fmt;

pub struct AST {}

pub struct SyntaxError {}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

pub(crate) fn emit_ast(tokens: Vec<super::lexer::token::Token>) -> Result<AST, SyntaxError> {
    todo!()
}
