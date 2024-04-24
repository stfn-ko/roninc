mod error;
mod file_handler;
mod lexer;
mod token;

use error::{RoninErrors, SyntaxError};
use std::process::exit;

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
