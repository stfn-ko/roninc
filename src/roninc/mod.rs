pub mod lexer;
pub mod token;

pub fn compile(args: Vec<String>) {
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