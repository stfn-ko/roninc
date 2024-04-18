mod roninc;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    roninc::compile(args);
}