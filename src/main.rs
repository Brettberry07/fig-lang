mod evalulator;
mod enviorment;
mod helper;
mod lexer;
mod parser;
mod token;

use lexer::Lexer;
use parser::Parser;
use evalulator::eval_program;

fn main() {
    let input = "
    var x = 5;
    var y = 10;
    var s = \"Hello, World!\";
    var z = x + y;
    var result = z * 2;
    result - 5;
    ";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let stmts = parser.parse();
    let result = eval_program(&stmts);
    println!("{:#?}", result);
}
