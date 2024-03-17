pub mod lexer;
pub mod token;
mod error;

pub fn compile(args: Vec<String>) {
    if args.len() < 2 {
        eprintln!("roninc::compile >> no args provided");
        return;
    }

    let ts = match lexer::emit_tokens(&args[1]) {
        Ok(res) => {
            for t in res {
                println!("{:#?}", t);
            }
        }
        Err(err) => {
            eprintln!("{err}");
            return;
        }
    };

    ts
}