pub mod common;
pub mod lexer;
pub mod parser;
use std::env;
use std::process;

fn usage() -> ! {
    eprintln!("Usage: seu9tk <file>");
    process::exit(1)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        usage();
    }

    let file_name = &args[1];
    let file_content = std::fs::read_to_string(file_name).unwrap();
    let mut lexer = lexer::Lexer::new(&file_content, file_name, &file_content);
    lexer.scan();
    let tokens = lexer.tokens;
    for token in tokens {
        println!("{:?}", token.kind);
    }
}
