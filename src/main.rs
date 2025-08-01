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
    var x = 5.12;
    var y = 10;
    var s = \"Hello, World!\";
    var b = true;
    var n = false;
    var z = x + y;
    z + x;
    var result = z * 2;
    ";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let stmts = parser.parse();
    let result = eval_program(&stmts);
    println!("{:#?}", result);
}
