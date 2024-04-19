pub struct AST {}

pub(crate) fn emit_ast(tokens: Tokens) -> Result<AST, SyntaxError> {
    for t in tokens {
        println!("{}", t.kind)
    }

    Ok(AST {})
}
