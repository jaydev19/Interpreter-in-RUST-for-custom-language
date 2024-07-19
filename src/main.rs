// src/main.rs

mod lexer;
mod parser;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use std::io::{self, Write};

fn main() {
    let mut interpreter = Interpreter::new();

    loop {
        print!("LOQ> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let nodes = parser.parse();

        interpreter.interpret(nodes);
    }
}
