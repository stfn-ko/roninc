mod parser;

pub fn compile(args: Vec<String>) {
    if args.len() < 2 {
        eprintln!("ronin >> no compiler arguments provided");
        return;
    }

    let ast = match parser::emit_ast(&args[1]) {
        Ok(res) => res,
        Err(err) => {
            for e in err {
                println!("{}", e)
            }

            return;
        }
    };
}
