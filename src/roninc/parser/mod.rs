mod error;
mod file_handler;
mod lexer;
mod token;

use error::{RoninError, RoninErrors, SyntaxError};
use std::process::exit;
use token::Tokens;

pub struct AST {
    pub statements: Vec<Statement>,
}

pub struct Statement {
    pub kind: StatementKind,
}

pub enum StatementKind {
    Expression(ExpressionKind),
}

pub enum ExpressionKind {
    Number(i64),
    B,
}

pub(crate) fn emit_ast(path: &str) -> Result<(), RoninErrors<SyntaxError>> {
    let tokens = match lexer::emit_tokens(path) {
        Ok(r) => r,
        Err(err) => {
            for e in err {
                eprintln!("{:#?}", e)
            }

            exit(0);
        }
    };

    for t in tokens {
        println!("{:#?}", t);
    }

    Ok(())
}
