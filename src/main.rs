use std::fs;

use css_tutorial::token_type::TokenType;
use css_tutorial::lexer::Lexer;
fn main() {
    let binding = fs::read_to_string("test2.css").unwrap();
    let mut lexer = Lexer::new(&binding);
    loop {
        let token = lexer.eat_token();
        if token.check_type(TokenType::EOF) {
            break;
        }
    }
}
