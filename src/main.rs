mod evalulator;
mod enviorment;
mod helper;
mod lexer;
mod parser;
mod token;

use lexer::Lexer;
use parser::Parser;
use evalulator::eval;

fn main() {
    let input = "var is_cool;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let expr = parser.parse();
    let result = eval(&expr);
    println!("{:#?}", result);
}
