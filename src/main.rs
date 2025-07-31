mod evalulator;
mod helper;
mod lexer;
mod parser;
mod token;

use lexer::Lexer;
use parser::Parser;
use evalulator::eval;

fn main() {
    let input = "4 + 2 * (3 - 1) / 2";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let expr = parser.parse();
    let result = eval(&expr);
    println!("{:#?}", result);
}
