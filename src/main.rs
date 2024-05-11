mod token;
mod scanner;
mod expr;
mod parser;

use std::{
    env,
    io::{self, BufRead, Write},
    process::exit,
};

use scanner::*;
use token::*;

fn run_prompt() -> Result<(), String> {
    let mut stdin = io::stdin().lock();

    println!("Welcome to the Lox Interpreter. Press CTRL+D to exit.\n");

    loop {
        let mut buffer = String::new();
        print!("> ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return Err(String::from("Failed to clear output")),
        }

        match stdin.read_line(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    println!("\nInterpreter Quit");
                    return Ok(());
                }
            }
            Err(_) => return Err(String::from("Failed to read input")),
        }

        println!("You Entered: {buffer}");
        run(buffer.trim());
    }
}

fn run_file(_src: &str) -> Result<(), String> {
    todo!("Ability to read files will be added in future dlc for only $99.99!")
}

fn run(src: &str) {
    let mut scanner = Scanner::new(src);
    let tokens: Vec<Token> = scanner.scan_tokens();

    for tok in tokens {
        println!("{}", tok.to_string());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in &args {
        println!("{arg}");
    }

    match args.len() {
        1 => match run_prompt() {
            Ok(_) => exit(0),
            Err(msg) => eprintln!("Error {msg}"),
        },
        2 => match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => eprintln!("Error {msg}"),
        },
        _ => {
            eprintln!("[Error] please use as lox ___");
            exit(1)
        }
    }
}
