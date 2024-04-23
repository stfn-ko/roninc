use super::error::{RoninErrors, SyntaxError};
use super::lexer::token::Tokens;

pub struct AST {}

pub(crate) fn emit_ast(tokens: Tokens) -> Result<AST, RoninErrors<SyntaxError>> {
    for t in tokens {
        println!("{:#?}", t.kind)
    }

    Ok(AST {})
}
