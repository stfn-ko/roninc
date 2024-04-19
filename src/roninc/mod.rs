mod error;
mod lexer;
mod parser;

pub fn compile(args: Vec<String>) {
    if args.len() < 2 {
        eprintln!("ronin >> no compiler arguments provided");
        return;
    }

    let tokens = match lexer::emit_tokens(&args[1]) {
        Ok(res) => res,
        Err(err) => {
            for e in err {
                println!("{}", e)
            }

            return;
        }
    };

    let ast = match parser::emit_ast(tokens) {
        Ok(res) => res,
        Err(err) => {
            for e in err {
                println!("{}", e)
            }

            return;
        }
    };
}
