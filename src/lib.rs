pub mod ast;
pub mod lexer;
pub mod parser;
pub mod range;
pub mod token;
pub mod token_type;

use ast::AstTreeBuilder;
use lexer::Lexer;
use parser::Parser;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(input: String) -> String {
    let mut lexer = Lexer::new(&input);
    let mut builder = AstTreeBuilder::new();
    let mut parser = Parser::new(&mut lexer, &mut builder);
    parser.parse();
    let serialized = serde_json::to_string(&builder.ast_tree).unwrap();
    serialized
}
