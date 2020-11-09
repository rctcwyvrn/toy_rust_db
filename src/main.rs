#![feature(try_blocks)]
use std::io::Write;
use std::io::{self};

use toy_rust_db::perform_query;

fn main() {
    repl();
}

fn repl() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        let mut input = String::new();
        let io: Result<_, io::Error> = try {
            print!("db> ");
            stdout.flush()?;
            stdin.read_line(&mut input)?;
        };

        if let Err(e) = io {
            eprintln!("IOError: {}", e);
        }

        let query_res = perform_query(input);

        match query_res {
            Ok(results) => println!("{}", results),
            Err(query_err) => eprintln!("{}", query_err),
        }
    }
}
