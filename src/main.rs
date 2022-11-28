use std::fs;

use css_tutorial::{ast::AstTreeBuilder, lexer::Lexer, parser::Parser, token_type::TokenType};
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
