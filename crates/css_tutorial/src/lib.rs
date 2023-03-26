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
