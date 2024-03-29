use std::env;

const VERSION: &str = "0.0.1";

struct Args {
    chmod: String,
    path: String,
    make_parent: bool,
    context: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let specs = include_bytes!("../specs.md");
    if args.iter().any(|i| i == "--version") {
        print!("version: {}", VERSION);
    }

    if args.iter().any(|i| i == "--help") {
        print!("{}", String::from_utf8_lossy(specs));
    }
}
