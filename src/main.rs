use std::env;
use std::path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len: usize = args.len();

    if args_len > 2 {
        println!("Usage: jlox [script]");
        std::process::exit(64);
    } else if args_len == 2 {
        // run file path -- should be the source code file
        run_file(path::Path::new(&args[1].to_string()));
    } else {
        // run the prompt from the cli
        run_prompt();
    }

    // dbg!(args);
}

fn repl() {}

fn run_file(path: &path::Path) {
    println!("{:?}", path.as_os_str());
}

fn run_prompt() {
    println!("Run prompt.");
}

fn run() {}
