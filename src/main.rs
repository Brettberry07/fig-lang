mod lexer;
mod token;

use lexer::Lexer;

fn main() {
    let input = r#"
        var x4: int = 42;
        fn greet(name: str) -> str {
            return "hello ${name}";
        }
    "#;

    let mut lexer = Lexer::new(input);
    loop {
        let tok = lexer.next_token();
        println!("{:?}", tok);
        if matches!(tok, token::Token::EOF) {
            break;
        }
    }
}
