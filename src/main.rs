mod evalulator;
mod enviorment;
mod helper;
mod lexer;
mod parser;
mod token;

use std::{fs};
use helper::{Type};
use lexer::Lexer;
use parser::Parser;
use evalulator::eval_program;

fn main() {
    let filename = "main.fg";

    let source = fs::read_to_string(filename)
        .expect("Failed to read file");

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);

    let stmts = parser.parse();
    let result = eval_program(&stmts);
    match result {
        Type::Null => {},
        _ => println!("Final result: {:?}", result),
    }
}
