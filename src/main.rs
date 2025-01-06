extern crate seu9cc;

use seu9cc::gen_ir::gen_ir;
use seu9cc::irdump::dump_ir;
use seu9cc::parse::parse;
use seu9cc::preprocess::Preprocessor;
use seu9cc::regalloc::alloc_regs;
use seu9cc::sema::sema;
use seu9cc::token::tokenize;

use std::env;
use std::process;

fn usage() -> ! {
    eprintln!("Usage: seu9cc [-t] <file>");
    process::exit(1)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        usage();
    }

    let mut dump_ir1 = false;
    let mut dump_tokens = false;
    let path;

    if args[1] == "-t" {
        if args.len() < 3 {
            usage();
        }
        dump_tokens = true;
        path = &args[2];
    } else {
        path = &args[1];
        dump_ir1 = true;
    }

    // Tokenize and parse.
    let tokens = tokenize(path.clone(), &mut Preprocessor::new());

    if dump_tokens {
        for token in &tokens {
            println!(
                "ty:{:?}, start:{}, end:{}",
                token.ty, token.start, token.end
            );
        }
        return;
    }

    let nodes = parse(&tokens);
    let (nodes, globals) = sema(nodes);
    let mut fns = gen_ir(nodes);

    if dump_ir1 {
        dump_ir(&fns);
    }

    // gen_x86(globals, fns);
}
