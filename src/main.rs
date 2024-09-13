mod callable;
mod environment;
mod expr;
mod interpreter;
mod native;
mod parser;
mod resolver;
mod scanner;
mod stmt;
mod token;

use crate::{
    interpreter::Interpreter, parser::Parser, resolver::Resolver, scanner::Scanner, stmt::Stmt,
    token::Token,
};
use std::{
    env, fs,
    io::{self, Write},
    process::exit,
};

fn run_prompt() -> Result<(), String> {
    let esc_key = match env::consts::OS {
        "windows" => "CTRL + Z",
        _ => "CTRL + D",
    };
    println!("Welcome to the Lox Interpreter!\nPress {esc_key} to exit");

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut interpreter: Interpreter = Interpreter::new();

    loop {
        let mut src = String::new();
        print!(">>> ");
        stdout.flush().unwrap();

        match stdin.read_line(&mut src) {
            Ok(n) => {
                if n == 0 {
                    println!("\nInterpreter Quit");
                    return Ok(());
                }
            }
            Err(_) => return Err(String::from("Failed to read input")),
        }

        let src = src.trim();

        if src.is_empty() {
            println!("");
            continue;
        }

        match run(src, &mut interpreter) {
            Ok(_) => (),
            Err(msg) => println!("\nERROR:\n{msg}\n"),
        };
    }
}

fn run_file(path: &str) -> Result<(), String> {
    let mut interpreter = Interpreter::new();
    match fs::read_to_string(path) {
        Ok(src) => run(src.as_str(), &mut interpreter)?,
        Err(msg) => return Err(msg.to_string()),
    };

    Ok(())
}

fn run(src: &str, interpreter: &mut Interpreter) -> Result<(), String> {
    let mut scanner: Scanner = Scanner::new(src);
    let tokens: &Vec<Token> = scanner.scan_tokens()?;

    let mut parser: Parser = Parser::new(&tokens);
    let statements: Vec<Stmt> = parser.parse()?;

    let mut resolver = Resolver::new();
    let locals = resolver.resolve(&statements)?;

    interpreter.resolve(locals);
    interpreter.interpret(statements.iter().collect())?;

    Ok(())
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => match run_prompt() {
            Ok(_) => (),
            Err(msg) => println!("Error:\n {msg}"),
        },
        2 => match run_file(&args[1]) {
            Ok(_) => (),
            Err(msg) => println!("Error:\n {msg}"),
        },
        _ => {
            println!("[Error] please use as lox ___");
            exit(64)
        }
    }
}
