mod expr; 
mod interpreter;
mod parser;
mod scanner;
mod stmt;
mod token;
mod environment;

use interpreter::Interpreter;
use parser::*;
use scanner::Scanner;
use std::{
    env, fs,
    io::{self, BufRead, Write},
    process::exit,
};
use stmt::Stmt;
use token::*;
// use expr::{Expr, LitValue};

fn run_prompt() -> Result<(), String> {
    let esc_key = match env::consts::OS {
        "windows" => "CTRL + Z",
        _ => "CTRL + D",
    };
    println!("Welcome to the Lox Interpreter!\nPress {} to exit", esc_key);

    let mut stdin: io::StdinLock<'static> = io::stdin().lock();
    let mut stdout: io::StdoutLock<'static> = io::stdout().lock();
    let mut interpreter: Interpreter = Interpreter::new();

    loop {
        let mut buffer = String::new();
        print!(r#">>> "#);
        stdout.flush().unwrap();

        match stdin.read_line(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    println!("\nInterpreter Quit");
                    return Ok(());
                }
            }
            Err(_) => return Err(String::from("Failed to read input")),
        }

        if &buffer == "\n" || &buffer == "\r\n" {
            println!("");
            continue;
        }

        match run(buffer.trim(), &mut interpreter) {
            Ok(_) => (),
            Err(msg) => println!("\nERROR:\n{msg}\n"),
        };
    }
}

fn run_file(path: &str) -> Result<(), String> {
    let mut interpreter = Interpreter::new();
    match fs::read_to_string(path) {
        Ok(src) => run (src.as_str(), &mut interpreter)?,
        Err(msg) => return Err(msg.to_string()),
    };

    Ok(())
}

fn run(src: &str, interpreter: &mut Interpreter) -> Result<(), String> {
    let mut scanner: Scanner = Scanner::new(src);
    scanner.scan_tokens()?;
    let tokens: Vec<Token> = scanner.tokens;
    let mut parser: Parser = Parser::new(tokens);
    let statements: Vec<Stmt> = parser.parse()?;
    interpreter.interpret(statements.iter().collect())?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => match run_prompt() {
            Ok(_) => exit(0),
            Err(msg) => println!("Error {msg}"),
        },
        2 => match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => println!("Error {msg}"),
        },
        _ => {
            println!("[Error] please use as lox ___");
            exit(1)
        }
    }
}
