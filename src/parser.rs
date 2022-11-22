use crate::ast::AstTreeBuilder;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

const CHARSET_SYM: &str = "@charset";
const IMPORT_SYM: &str = "@import";
const PAGE_SYM: &str = "@page";
const IMPORTANT_SYM: &str = "!important";

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    builder: &'a mut AstTreeBuilder<TokenType>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>, builder: &'a mut AstTreeBuilder<TokenType>) -> Self {
        Self { lexer, builder }
    }

    pub fn peek(&mut self) -> Option<Token> {
        self.lexer.get_peek_token()
    }
    pub fn advance(&mut self) {
        let node = self.lexer.eat_token();
        self.builder.token(node);
    }

    pub fn check_token_and_advance(&mut self, token_type: TokenType) {
        self.check_token(token_type);
        self.advance()
    }

    fn check_token(&mut self, token_type: TokenType) {
        if !self.check_token_type(token_type) {
            panic!(
                "expect token type is {:?} but get token type {:?}",
                token_type,
                self.lexer.get_peek_token().unwrap().0
            );
        }
    }

    fn check_token_type(&mut self, token_type: TokenType) -> bool {
        if !self.lexer.get_peek_token().unwrap().check_type(token_type) {
            dbg!(self.lexer.get_peek_token(), token_type);
            return false;
        }
        true
    }

    pub fn token_eq_str(&self, token: &Token, str: &str) -> bool {
        let ident_str = token.get_source_code(self.lexer.source_code);
        ident_str.eq_ignore_ascii_case(str)
    }

    pub fn parse(mut self) {
        self.builder.start_node(TokenType::Stylesheet);
        while let Some(token) = self.peek() {
            dbg!(token);
            let Token(token_type, _) = token;
            match token_type {
                TokenType::AtKeywordToken => {
                    if self.token_eq_str(&token, IMPORT_SYM) {
                        self.parse_import_token();
                    }

                    if self.token_eq_str(&token, CHARSET_SYM) {
                        self.parse_charset();
                    }

                    if self.token_eq_str(&token, PAGE_SYM) {
                        self.parse_page();
                    }
                }
                TokenType::Comment => {
                    self.parse_comment();
                }
                TokenType::CDCToken | TokenType::CDOToken => {
                    self.advance();
                }
                TokenType::IdentToken => {}
                TokenType::EOF => {
                    dbg!(&self.builder);
                    self.builder.finish_node();
                    self.builder.finish();
                    return;
                }
                _ => self.parse_rule(),
            }
        }
    }

    fn parse_comment(&mut self) {
        self.check_token_and_advance(TokenType::Comment);
    }

    fn parse_rule(&mut self) {
        self.builder.start_node(TokenType::RuleList);
        self.parse_selector();
        loop {
            if self.check_token_type(TokenType::Comma) {
                self.advance();

                self.parse_selector();
            } else {
                break;
            }
        }

        self.parse_declaration_list();

        self.builder.finish_node();
    }

    fn parse_selector(&mut self) {
        self.builder.start_node(TokenType::Select);

        // TODO: simplet select
        //  : simple_selector [ combinator selector | S+ [ combinator? selector ]? ]?
        self.builder.finish_node();
    }
    fn parse_charset(&mut self) {
        self.builder.start_node(TokenType::ChartSet);

        self.advance();

        self.check_token_and_advance(TokenType::Str);

        self.check_token_and_advance(TokenType::Semi);

        self.builder.finish_node();
    }

    fn parse_import_token(&mut self) {
        self.builder.start_node(TokenType::Import);
        self.advance();

        if self.check_token_type(TokenType::Str) || self.check_token_type(TokenType::UrlToken) {
            self.advance();

            if self.check_token_type(TokenType::IdentToken) {
                self.parse_media_list();
            }
        } else {
            panic!(
                "expect token type is string | url, but found token {:?}",
                self.peek()
            )
        }
        self.builder.finish_node();
    }

    fn parse_media_list(&mut self) {
        self.builder.start_node(TokenType::MediumList);

        self.check_token_and_advance(TokenType::IdentToken);

        loop {
            if self.check_token_type(TokenType::Comma) {
                self.advance();

                self.check_token_and_advance(TokenType::IdentToken);
            } else {
                break;
            }
        }
        self.builder.finish_node();
    }

    fn parse_function(&mut self) {
        self.builder.start_node(TokenType::Function);

        self.check_token_and_advance(TokenType::FunctionToken);

        self.parse_expr();

        self.check_token_and_advance(TokenType::RightParenthesis);

        self.builder.finish_node();
    }

    pub(crate) fn parse_page(&mut self) {
        self.builder.start_node(TokenType::Page);

        self.advance();

        // pseudo_page?
        if self.check_token_type(TokenType::Colon) {
            self.advance();

            self.check_token_and_advance(TokenType::IdentToken);
        }

        self.parse_declaration_list();

        self.builder.finish_node();
    }

    fn parse_declaration_list(&mut self) {
        self.builder.start_node(TokenType::DeclarationList);
        self.check_token_and_advance(TokenType::LeftCurlyBracket);
        if self.check_token_type(TokenType::IdentToken) {
            self.parse_declaration();

            // [ ';' S* declaration? ]*
            loop {
                if !self.check_token_type(TokenType::Semi) {
                    break;
                }
                self.check_token_and_advance(TokenType::Semi);
                if self.check_token_type(TokenType::IdentToken) {
                    self.parse_declaration();
                }
            }
        }
        self.check_token_and_advance(TokenType::RightCurlyBracket);
        self.builder.finish_node();
    }

    fn parse_declaration(&mut self) {
        self.builder.start_node(TokenType::Declaration);

        self.parse_property();

        self.check_token_and_advance(TokenType::Colon);

        self.parse_expr();

        self.builder.finish_node();
    }

    fn parse_property(&mut self) {
        self.builder.start_node(TokenType::Property);
        self.check_token_and_advance(TokenType::IdentToken);
        self.builder.finish_node();
    }

    fn parse_expr(&mut self) {
        self.builder.start_node(TokenType::Expression);
        self.parse_term();

        loop {
            if self.check_token_type(TokenType::Semi)
                || self.check_token_type(TokenType::ForwardSlash)
            {
                self.builder.start_node(TokenType::Operator);
                self.advance();
                self.builder.finish_node();
            }

            if !self.parse_term() {
                break;
            };
        }

        self.builder.finish_node();
    }

    fn parse_term(&mut self) -> bool {
        // todo: add hexcolor and number 系列
        if let Some(token) = self.peek() {
            match token.0 {
                TokenType::Digital
                | TokenType::Dimension
                | TokenType::Str
                | TokenType::IdentToken
                | TokenType::UrlToken => {
                    self.builder.start_node(TokenType::Term);

                    self.advance();
                    self.builder.finish_node();
                    return true;
                }
                TokenType::FunctionToken => {
                    self.builder.start_node(TokenType::Term);

                    self.parse_function();
                    self.builder.finish_node();
                    return true;
                }
                _ => {}
            }
        }
        false
    }

    fn parse_prio(&mut self) {
        self.check_token_and_advance(TokenType::Important);
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::AstTreeBuilder;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn charset_test() {
        let mut lexer = Lexer::new(r#"@charset "utf8";"#);
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_charset();
        builder.finish();
        dbg!(builder.ast_tree);
    }

    #[test]
    fn import_url_test() {
        let mut lexer = Lexer::new(
            r#"@import "custom.css";
            @import url("bluish.css");"#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);

        parser.parse_import_token();
        builder.finish();
        dbg!(builder.ast_tree);
    }

    #[test]
    fn import_string_test() {
        let mut lexer = Lexer::new(r#"@import "custom.css";"#);
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_import_token();
        builder.finish();
        dbg!(builder.ast_tree);
    }

    #[test]
    fn comment_test() {
        let mut lexer = Lexer::new(
            r#"/* adfsdf
        
        
        
        
        */"#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_comment();
        builder.finish();
        dbg!(builder.ast_tree);
    }

    #[test]
    fn function_test() {
        let mut lexer = Lexer::new(r#"a(b(123))"#);
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_function();
        builder.finish();
        dbg!(builder.ast_tree);
    }

    #[test]
    fn declaration_list_test() {
        let mut lexer = Lexer::new(
            r#"{
            a: 123
        }"#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_declaration_list();
        builder.finish();
        dbg!(builder.ast_tree);
    }
}
