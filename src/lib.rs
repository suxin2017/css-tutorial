pub mod ast;
pub mod lexer;
pub mod parser;
pub mod range;
pub mod token;

use ast::{AstTree, AstTreeBuilder};
use lexer::Lexer;
use parser::Parser;
use token::TokenType;
use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// pub fn sum() -> String {
//     let mut lexer = Lexer::new(r#"@charset "utf8";"#);
//     let mut builder = AstTreeBuilder::new();
//     let mut parser = Parser::new(&mut lexer, &mut builder);
//     parser.parse();
//     let serialized = serde_json::to_string(&builder.ast_tree).unwrap();
//     serialized
// }
