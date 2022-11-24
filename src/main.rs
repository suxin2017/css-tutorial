use std::fs;

use css_tutorial::{ast::AstTreeBuilder, lexer::Lexer, parser::Parser};
fn main() {
    let binding = fs::read_to_string("test.css").unwrap();
    let mut lexer = Lexer::new(&binding);
    let mut builder = AstTreeBuilder::new();
    let mut parser = Parser::new(&mut lexer, &mut builder);
    parser.parse();
}
