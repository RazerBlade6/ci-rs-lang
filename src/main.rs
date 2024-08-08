
mod token;
mod scanner;
mod expr;
mod parser;

use std::{env, io::{self, BufRead, Write}, process::{exit, Command}};
use scanner::Scanner;
use token::*;
use parser::*;
use expr::Expr;

fn run_prompt() -> Result<(), String> {
    let esc_key = match env::consts::OS {
        "windows" => "CTRL + Z",
        _ => "CTRL + D"
    };
    println!("Welcome to the Lox Interpreter!\nPress {} to exit", esc_key);

    let mut stdin: io::StdinLock<'static> = io::stdin().lock();
    let mut stdout: io::StdoutLock<'static> = io::stdout().lock();

    loop {
        let mut buffer = String::new();
        print!(r#">>>` "#);
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

        println!("You Entered: {buffer}");
        if &buffer[0..4] == "sys." {
            Command::new("powershell")
                .arg(&buffer[4..].trim())
                .spawn()
                .expect("Failed to run item")
                .wait()
                .expect("Failed to wait");
        } else {
            run(buffer.trim());
        }
    }
}

fn run_file(_src: &str) -> Result<(), String> {
    todo!("Ability to read files will be added in future DLC for only $99.99!")
}

fn run(src: &str) {
    let mut scanner: Scanner = Scanner::new(src);
    let tokens: Vec<Token> = scanner.scan_tokens();
    let mut parser: Parser = Parser::new(tokens);
    let expr: Expr = parser.parse().expect("Currently Not Implemented");
    println!("Parsed Expr is: {}", expr.to_string())
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
