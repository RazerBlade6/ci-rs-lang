
mod token;
mod scanner;
mod expr;
mod interpreter;
mod parser;
mod stmt;

use std::{env, fs, io::{self, BufRead, Write}, process::exit};
use interpreter::Interpreter;
use scanner::Scanner;
use stmt::Stmt;
use token::*;
use parser::*;
use expr::{Expr, LitValue};

fn run_prompt() -> Result<(), String> {
    let esc_key = match env::consts::OS {
        "windows" => "CTRL + Z",
        _ => "CTRL + D"
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

        run(buffer.trim(), &mut interpreter);
    }
}

fn run_file(path: &str) -> Result<(), String> {
    let mut interpreter = Interpreter::new();
    match fs::read_to_string(path) {
        Ok(src) => run(&src, &mut interpreter),
        Err(msg) => return Err(msg.to_string())
    }
}

fn run(src: &str, interpreter: &mut Interpreter) -> Result<(), String> {
    let mut scanner: Scanner = Scanner::new(src);
    let tokens: Vec<Token> = scanner.scan_tokens();
    let mut parser: Parser = Parser::new(tokens);
    let statements: Vec<Stmt> = parser.parse().unwrap();
    for statement in statements {
        match statement {
            Stmt::Expression { expr } => {interpreter.interpret(expr)?;},
            _ => todo!()
        }
    }
    println!("Result is: {}", result.to_string());

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

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
