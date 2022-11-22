use css_tutorial::{ast::AstTreeBuilder, lexer::Lexer, parser::Parser};
fn main() {
    let mut lexer = Lexer::new(r#"@charset "utf8";"#);
    let mut builder = AstTreeBuilder::new();
    let mut parser = Parser::new(&mut lexer, &mut builder);
    let syntax_token = parser.parse();
    dbg!(syntax_token);
    dbg!(builder.ast_tree);
}
