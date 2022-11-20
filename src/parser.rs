use crate::lexer::Lexer;
use crate::token::{
    ChartSet, CommentNode, Import, Medium, Rule, Select, Stylesheet, SyntaxNode, Token, TokenType,
};

const CHARSET_SYM: &str = "@charset";
const IMPORT_SYM: &str = "@import";

#[derive(Debug)]
struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
}

impl<'a> Parser<'a> {
    fn new(lexer: &'a mut Lexer<'a>) -> Self {
        Self { lexer }
    }

    pub fn peek(&mut self) -> Option<Token> {
        self.lexer.get_peek_token()
    }
    pub fn advance(&mut self) -> Token {
        return self.lexer.eat_token();
    }

    pub fn check_token_and_advance(&mut self, token_type: TokenType) -> Token {
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
    pub fn parse(mut self) -> SyntaxNode {
        let mut stylesheet = Stylesheet { nodes: Vec::new() };
        while let Some(token) = self.peek() {
            dbg!(token);
            let Token(token_type, _) = token;
            match token_type {
                TokenType::AtKeywordToken => {
                    dbg!(token.get_source_code(self.lexer.source_code));
                    if self.token_eq_str(&token, IMPORT_SYM) {
                        stylesheet.nodes.push(self.parse_import_token());
                    }

                    if self.token_eq_str(&token, CHARSET_SYM) {
                        stylesheet.nodes.push(self.parse_charset());
                    }
                }
                TokenType::Comment => {
                    let comment = self.parse_comment();
                    stylesheet.nodes.push(comment);
                }
                TokenType::CDCToken | TokenType::CDOToken => {
                    self.advance();
                }
                TokenType::IdentToken => {}
                TokenType::EOF => {
                    return SyntaxNode::Stylesheet(stylesheet);
                }
                _ => self.parse_rule(),
            }
        }
        return SyntaxNode::Stylesheet(stylesheet);
    }

    fn parse_comment(&mut self) -> SyntaxNode {
        let comment_token = CommentNode {
            node: self.peek().unwrap(),
        };
        self.lexer.eat_token();
        return SyntaxNode::CommentNode(comment_token);
    }
    fn parse_rule(&mut self) {
        let mut rule = Rule { rules: Vec::new() };
        if let Some(Token(token_type, _)) = self.peek() {
            match token_type {
                TokenType::LeftCurlyBracket => {
                    todo!()
                }
                _ => {
                    let selector = self.parse_selector();
                }
            }
        }
        panic!("")
    }
    fn parse_component(&mut self) -> SyntaxNode {
        if let Some(token) = self.peek() {
            let Token(token_type, _) = token;
            match token_type {
                _ => {
                    let token = SyntaxNode::Token(token);
                    self.lexer.eat_token();
                    return token;
                }
            }
        }
        panic!("parse component error")
    }
    fn parse_selector(&mut self) -> Vec<SyntaxNode> {
        let mut selector = Vec::new();

        loop {
            match self.peek() {
                Some(Token(token_type, _)) => match token_type {
                    TokenType::LeftCurlyBracket => {
                        return selector;
                    }
                    _ => {
                        let mut select = Select { tokens: Vec::new() };
                        let token = self.parse_component();
                        selector.push(SyntaxNode::Select(select));
                    }
                },
                None => {
                    panic!("expected {} found none", "{");
                }
            }
        }
    }
    fn parse_charset(&mut self) -> SyntaxNode {
        self.advance();
        let result = SyntaxNode::ChartSet(ChartSet {
            token: self.check_token_and_advance(TokenType::Str),
        });
        self.check_token_and_advance(TokenType::Semi);
        return result;
    }

    fn parse_import_token(&mut self) -> SyntaxNode {
        self.advance();
        let mut import_token = Import { token: vec![] };
        if self.check_token_type(TokenType::Str) || self.check_token_type(TokenType::UrlToken) {
            import_token.token.push(SyntaxNode::Token(self.advance()));

            if self.check_token_type(TokenType::IdentToken) {
                import_token.token.append(&mut self.parse_media_list())
            }
            self.check_token_and_advance(TokenType::Semi);
        } else {
            panic!(
                "expect token type is string | url, but found token {:?}",
                self.peek()
            )
        }

        return SyntaxNode::Import(import_token);
    }

    fn parse_media_list(&mut self) -> Vec<SyntaxNode> {
        let mut media_list = vec![];
        let medium = self.check_token_and_advance(TokenType::IdentToken);
        media_list.push(SyntaxNode::Medium(Medium { token: medium }));

        loop {
            if self.check_token_type(TokenType::Comma) {
                self.advance();

                let medium = self.check_token_and_advance(TokenType::IdentToken);
                media_list.push(SyntaxNode::Medium(Medium { token: medium }));
            } else {
                break;
            }
        }

        media_list
    }

    fn parse_function(&mut self)->SyntaxNode{
        
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::token::SyntaxNode;

    #[test]
    fn charset_test() {
        let mut lexer = Lexer::new(r#"@charset "utf8";"#);
        let mut parser = Parser::new(&mut lexer);
        let syntax_token = parser.parse();
        dbg!(syntax_token);
    }

    #[test]
    fn import_url_test() {
        let mut lexer = Lexer::new(
            r#"@import "custom.css";
            @import url("bluish.css")"#,
        );
        let mut parser = Parser::new(&mut lexer);
        let syntax_token = parser.parse();
        dbg!(syntax_token);
    }

    #[test]
    fn import_string_test() {
        let mut lexer = Lexer::new(r#"@import "custom.css";"#);
        let mut parser = Parser::new(&mut lexer);
        let syntax_token = parser.parse();
        dbg!(syntax_token);
    }
}
