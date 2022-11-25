use std::backtrace::Backtrace;

use crate::ast::AstTreeBuilder;
use crate::lexer::Lexer;
use crate::token::Token;
use crate::token_type::TokenType;

const CHARSET_SYM: &str = "@charset";
const IMPORT_SYM: &str = "@import";
const PAGE_SYM: &str = "@page";

const MEDIA_SYM: &str = "@media";
const FONT_FACE_SYM: &str = "@font-face";
const KEY_FRAMES: &str = "@keyframes";
const W_KEY_FRAMES: &str = "@-webkit-keyframes";
const O_KEY_FRAMES: &str = "@-o-keyframes";

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

    pub fn check_token(&mut self, token_type: TokenType) {
        if !self.check_token_type(token_type) {
            panic!(
                "expect token type is {:?} but get token type {:?}",
                token_type,
                self.lexer.get_peek_token().unwrap().0
            );
        }
    }

    pub fn check_token_type(&mut self, token_type: TokenType) -> bool {
        if !self.lexer.check_peek_token_by_type(token_type) {
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
            let Token(token_type, _) = token;
            match token_type {
                TokenType::AtKeywordToken => {
                    self.parse_at_rule();
                }
                TokenType::Comment => {
                    self.parse_comment();
                }
                TokenType::CDCToken | TokenType::CDOToken => {
                    self.advance();
                }
                TokenType::EOF => {
                    self.builder.finish_node();
                    self.builder.finish();
                    return;
                }
                _ => self.parse_rule(),
            }
        }
    }

    pub fn parse_comment(&mut self) {
        self.check_token_and_advance(TokenType::Comment);
    }

    pub fn parse_rule(&mut self) {
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

    pub fn parse_selector(&mut self) {
        loop {
            if self.check_token_type(TokenType::EOF)
                || self.check_token_type(TokenType::LeftCurlyBracket)
                || self.check_token_type(TokenType::Comma)
            {
                break;
            }
            self.builder.start_node(TokenType::Selector);

            self.parse_simple_select();

            if self.check_token_type(TokenType::Plus) || self.check_token_type(TokenType::MoreThan)
            {
                self.advance();
            }
            self.builder.finish_node();
        }
    }
    pub fn parse_charset(&mut self) {
        self.builder.start_node(TokenType::ChartSet);

        self.advance();

        self.check_token_and_advance(TokenType::Str);

        self.check_token_and_advance(TokenType::Semi);

        self.builder.finish_node();
    }

    pub fn parse_import_token(&mut self) {
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

    pub fn parse_media_list(&mut self) {
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

    pub fn parse_function(&mut self) {
        self.builder.start_node(TokenType::Function);

        self.check_token_and_advance(TokenType::FunctionToken);
        self.parse_expr();
        self.check_token_and_advance(TokenType::RightParenthesis);

        self.builder.finish_node();
    }

    pub fn parse_page(&mut self) {
        self.builder.start_node(TokenType::Page);

        self.advance();

        if self.check_token_type(TokenType::Colon) {
            self.advance();

            self.check_token_and_advance(TokenType::IdentToken);
        }

        self.parse_declaration_list();

        self.builder.finish_node();
    }

    pub fn parse_declaration_list(&mut self) {
        self.builder.start_node(TokenType::DeclarationList);
        self.check_token_and_advance(TokenType::LeftCurlyBracket);
        self.parse_declaration();

        // [ ';' S* declaration? ]*
        loop {
            if !self.check_token_type(TokenType::Semi) {
                break;
            }
            self.check_token_and_advance(TokenType::Semi);
            self.parse_declaration();
        }
        self.check_token_and_advance(TokenType::RightCurlyBracket);
        self.builder.finish_node();
    }

    pub fn parse_declaration(&mut self) {
        if self.check_token_type(TokenType::IdentToken) {
            self.builder.start_node(TokenType::Declaration);

            self.parse_property();

            self.check_token_and_advance(TokenType::Colon);
            self.parse_expr();

            self.parse_prio();
            self.builder.finish_node();
        }
    }

    pub fn parse_property(&mut self) {
        self.builder.start_node(TokenType::Property);
        self.check_token_and_advance(TokenType::IdentToken);
        self.builder.finish_node();
    }

    pub fn parse_expr(&mut self) {
        self.builder.start_node(TokenType::Expression);
        self.parse_term();
        loop {
            if self.check_token_type(TokenType::Comma)
                || self.check_token_type(TokenType::ForwardSlash)
                || self.check_token_type(TokenType::Equal)
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

    pub fn parse_term(&mut self) -> bool {
        // todo: add hexcolor and number 系列
        if let Some(token) = self.peek() {
            match token.0 {
                TokenType::Digital
                | TokenType::Dimension
                | TokenType::Str
                | TokenType::IdentToken
                | TokenType::UrlToken
                | TokenType::Dot
                | TokenType::Colon
                | TokenType::HashToken
                | TokenType::Plus
                | TokenType::PercentageToken => {
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

    pub fn parse_prio(&mut self) {
        if self.check_token_type(TokenType::Important) {
            self.advance();
        }
    }

    pub fn parse_nest_at_rule(&mut self) {
        self.check_token_and_advance(TokenType::LeftCurlyBracket);
        loop {
            if self.check_token_type(TokenType::RightCurlyBracket) {
                break;
            }
            self.parse_rule();
        }

        self.check_token_and_advance(TokenType::RightCurlyBracket);
    }
    pub fn parse_simple_at_rule(&mut self) {
        self.parse_declaration_list();
    }
    pub fn parse_at_rule(&mut self) {
        if let Some(token) = self.peek() {
            if self.token_eq_str(&token, IMPORT_SYM) {
                self.parse_import_token();
                return;
            } else if self.token_eq_str(&token, CHARSET_SYM) {
                self.parse_charset();
                return;
            } else if self.token_eq_str(&token, PAGE_SYM) {
                self.parse_page();
                return;
            }

            self.builder.start_node(TokenType::AtRule);
            self.check_token_and_advance(TokenType::AtKeywordToken);
            self.builder.start_node(TokenType::AtRuleParams);
            loop {
                if let Some(node) = self.peek() {
                    match node.0 {
                        TokenType::LeftCurlyBracket | TokenType::EOF => {
                            break;
                        }
                        _ => {
                            self.advance();
                        }
                    }
                } else {
                    break;
                }
            }
            self.builder.finish_node();
            if self.token_eq_str(&token, MEDIA_SYM)
                || self.token_eq_str(&token, KEY_FRAMES)
                || self.token_eq_str(&token, W_KEY_FRAMES)
                || self.token_eq_str(&token, O_KEY_FRAMES)
            {
                self.parse_nest_at_rule();
            } else {
                self.parse_simple_at_rule();
            }

            self.builder.finish_node();
        }
    }

    pub fn parse_simple_select(&mut self) {
        self.builder.start_node(TokenType::SimpleSelect);
        self.parse_element_name();
        self.parse_hash();
        self.parse_class();
        self.parse_attrib();
        self.parse_pseudo();

        self.builder.finish_node();
    }

    pub fn parse_element_name(&mut self) {
        loop {
            if self.check_token_type(TokenType::IdentToken)
                || self.check_token_type(TokenType::Asterisk)
            {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn parse_hash(&mut self) {
        loop {
            if self.check_token_type(TokenType::HashToken) {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn parse_class(&mut self) {
        loop {
            if self.check_token_type(TokenType::Dot) {
                self.builder.start_node(TokenType::Class);
                self.advance();
                self.check_token_and_advance(TokenType::IdentToken);
                self.builder.finish_node();
            } else {
                break;
            }
        }
    }

    pub fn parse_attrib(&mut self) {
        loop {
            if self.check_token_type(TokenType::LeftSquareBracket) {
                self.builder.start_node(TokenType::Attrib);

                self.advance();
                self.check_token_and_advance(TokenType::IdentToken);

                if let Some(node) = self.peek() {
                    match node.0 {
                        TokenType::Equal
                        | TokenType::Includes
                        | TokenType::Dashmatch
                        | TokenType::Exclude
                        | TokenType::AllMatch => {
                            self.advance();
                        }
                        _ => {}
                    }
                }

                if let Some(node) = self.peek() {
                    match node.0 {
                        TokenType::IdentToken | TokenType::Str => {
                            self.advance();
                        }
                        _ => {}
                    }
                }
                self.check_token_and_advance(TokenType::RightSquareBracket);
                self.builder.finish_node();
            } else {
                break;
            }
        }
    }

    pub fn parse_pseudo(&mut self) {
        loop {
            if self.check_token_type(TokenType::Colon) {
                self.advance();
                if self.check_token_type(TokenType::Colon) {
                    self.advance();
                }

                if self.check_token_type(TokenType::IdentToken) {
                    self.advance();
                }
                if self.check_token_type(TokenType::FunctionToken) {
                    self.builder.start_node(TokenType::Function);

                    self.check_token_and_advance(TokenType::FunctionToken);

                    self.parse_simple_select();
                    self.check_token_and_advance(TokenType::RightParenthesis);

                    self.builder.finish_node();
                }
            } else {
                break;
            }
        }
    }
}
