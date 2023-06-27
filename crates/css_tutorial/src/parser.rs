use crate::ast::AstTreeBuilder;
use crate::lexer::Lexer;
use crate::token::Token;
use crate::token_type::TokenType;

const CHARSET_SYM: &str = "@charset";
const IMPORT_SYM: &str = "@import";
const PAGE_SYM: &str = "@page";

const MEDIA_SYM: &str = "@media";
// const FONT_FACE_SYM: &str = "@font-face";
const KEY_FRAMES: &str = "@keyframes";
const W_KEY_FRAMES: &str = "@-webkit-keyframes";
const M_KEY_FRAMES: &str = "@-moz-keyframes";
const O_KEY_FRAMES: &str = "@-o-keyframes";
const SUPPORTS: &str = "@supports";

// ANCHOR: parser
#[derive(Debug)]
pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    builder: &'a mut AstTreeBuilder<TokenType>,
}
// ANCHOR_END: parser

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>, builder: &'a mut AstTreeBuilder<TokenType>) -> Self {
        Self { lexer, builder }
    }
    // ANCHOR: lexer_wrapper
    pub fn peek(&mut self) -> Option<&Token> {
        loop {
            let token = self.lexer.get_peek_token();
            if token.is_some_and(|t| t.check_type(TokenType::Comment)) {
                self.advance();
            } else {
                break;
            }
        }

        return self.lexer.get_peek_token();
    }
    pub fn advance(&mut self) {
        let node = self.lexer.eat_token();
        // dbg!(node);
        // self.peek().unwrap().print_detail(self.lexer.source_code);
        self.builder.token(node);
    }

    pub fn check_token_and_advance(&mut self, token_type: TokenType) {
        self.check_token(token_type);
        self.advance()
    }

    pub fn check_token(&mut self, token_type: TokenType) {
        if !self.check_token_type(token_type) {
            let source = self.lexer.source_code;
            panic!(
                "expect token type is {:?} but get token type {:?}",
                token_type,
                self.lexer.get_peek_token().unwrap()
            );
        }
    }

    pub fn check_token_type(&mut self, token_type: TokenType) -> bool {
        if let Some(token) = self.peek() {
            return token.check_type(token_type);
        }
        true
    }

    //ANCHOR_END:lexer_wrapper
    // ANCHOR: entry
    pub fn parse(&mut self) {
        self.builder.start_node(TokenType::Stylesheets);
        self.parse_entry();
        self.builder.finish_node();
        self.builder.finish();
    }

    fn parse_entry(&mut self) {
        while let Some(token) = self.peek() {
            let token_type = &token.r#type;
            match token_type {
                TokenType::AtKeywordToken => {
                    self.parse_at_rule();
                }
                TokenType::CDCToken | TokenType::CDOToken => {
                    self.advance();
                }
                TokenType::EOF => {
                    return;
                }
                TokenType::RightCurlyBracket => {
                    return;
                }
                _ => self.parse_rule(),
            }
        }
    }
    // ANCHOR_END: entry

    pub fn parse_rule(&mut self) {
        self.builder.start_node(TokenType::RuleList);

        self.parse_selector_list();

        self.parse_declaration_list();

        self.builder.finish_node();
    }

    pub fn parse_selector_list(&mut self) {
        self.builder.start_node(TokenType::SelectorList);
        self.parse_selector();
        loop {
            if self.check_token_type(TokenType::Comma) {
                self.advance();

                self.parse_selector();
            } else {
                break;
            }
        }
        self.builder.finish_node();
    }

    pub fn parse_selector(&mut self) {
        loop {
            if self.check_token_type(TokenType::EOF)
                || self.check_token_type(TokenType::LeftCurlyBracket)
                || self.check_token_type(TokenType::RightParenthesis)
                || self.check_token_type(TokenType::Comma)
                || self.check_token_type(TokenType::RightCurlyBracket)
            {
                break;
            }
            self.builder.start_node(TokenType::Selector);

            self.parse_simple_select();

            if self.check_token_type(TokenType::Plus)
                || self.check_token_type(TokenType::MoreThan)
                || self.check_token_type(TokenType::Wave)
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
        if self.check_token_type(TokenType::LeftCurlyBracket) {
            self.builder.start_node(TokenType::DeclarationList);
            self.check_token_and_advance(TokenType::LeftCurlyBracket);
            if self.check_token_type(TokenType::IdentToken)
                && self.peek().is_some_and(|t| t.get_source_code() != '&'.to_string())
            {
                self.parse_declaration();

                // [ ';' S* declaration? ]*
                loop {
                    if !self.check_token_type(TokenType::Semi) {
                        break;
                    }
                    self.check_token_and_advance(TokenType::Semi);
                    self.parse_declaration();
                }
            } else {
                self.parse_entry();
            }

            self.check_token_and_advance(TokenType::RightCurlyBracket);
            self.builder.finish_node();
        }
    }

    pub fn parse_declaration(&mut self) {
        if self.check_token_type(TokenType::IdentToken)
            || self.check_token_type(TokenType::Asterisk)
        {
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
        //兼容ie
        if self.check_token_type(TokenType::Asterisk) {
            self.advance();
        }
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
        if let Some(token) = self.peek() {
            match token.r#type {
                TokenType::Digital
                | TokenType::Dimension
                | TokenType::Str
                | TokenType::IdentToken
                | TokenType::UrlToken
                | TokenType::Dot
                | TokenType::Colon
                | TokenType::HashToken
                | TokenType::Plus
                | TokenType::Minus
                | TokenType::Asterisk
                | TokenType::ForwardSlash
                | TokenType::PercentageToken => {
                    self.builder.start_node(TokenType::Term);
                    self.advance();
                    self.builder.finish_node();
                    return true;
                }
                TokenType::AtKeywordToken => {
                    self.parse_variable_declaration(false);
                }
                TokenType::FunctionToken => {
                    self.builder.start_node(TokenType::Term);
                    self.parse_function();
                    self.builder.finish_node();
                    return true;
                }
                TokenType::LeftParenthesis => {
                    self.advance();
                    loop {
                        if let Some(token) = self.peek() {
                            match token.r#type {
                                TokenType::RightParenthesis => {
                                    self.advance();
                                    break;
                                }
                                _ => {
                                    self.parse_term();
                                }
                            }
                        } else {
                            break;
                        }
                    }
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

        self.parse_entry();

        self.check_token_and_advance(TokenType::RightCurlyBracket);
    }
    pub fn parse_simple_at_rule(&mut self) {
        self.parse_declaration_list();
    }
    pub fn token_eq_str(&mut self, str: &str) -> bool {
        let source = self.lexer.source_code;
        if let Some(token) = self.peek() {
            let ident_str = token.get_source_code();
            return ident_str.eq_ignore_ascii_case(str);
        }
        false
    }

    pub fn parse_at_rule(&mut self) {
        if self.token_eq_str(IMPORT_SYM) {
            self.parse_import_token();
            return;
        }
        if self.token_eq_str(CHARSET_SYM) {
            self.parse_charset();
            return;
        }
        if self.token_eq_str(PAGE_SYM) {
            self.parse_page();
            return;
        }
        let is_nest_at_rule = self.token_eq_str(MEDIA_SYM)
            || self.token_eq_str(KEY_FRAMES)
            || self.token_eq_str(W_KEY_FRAMES)
            || self.token_eq_str(O_KEY_FRAMES)
            || self.token_eq_str(M_KEY_FRAMES)
            || self.token_eq_str(SUPPORTS);

        let is_variable =
            !is_nest_at_rule && self.lexer.check_peek_peek_token_by_type(TokenType::Colon);
        if is_variable {
            self.parse_variable_declaration(true);
            return;
        }

        self.builder.start_node(TokenType::AtRule);
        self.check_token_and_advance(TokenType::AtKeywordToken);
        self.builder.start_node(TokenType::AtRuleParams);
        loop {
            if let Some(node) = self.peek() {
                match node.r#type {
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
        if is_nest_at_rule {
            self.parse_nest_at_rule();
        } else {
            self.parse_simple_at_rule();
        }

        self.builder.finish_node();
    }

    fn parse_variable_declaration(&mut self, force: bool) {
        self.builder.start_node(TokenType::VariableDeclaration);
        self.builder.start_node(TokenType::Variable);
        self.check_token_and_advance(TokenType::AtKeywordToken);
        self.builder.finish_node();
        if self.check_token_type(TokenType::Colon) || force {
            self.check_token_and_advance(TokenType::Colon);
            self.parse_expr();
        }
        if self.check_token_type(TokenType::Semi) || force {
            self.check_token_and_advance(TokenType::Semi);
        }
        self.builder.finish_node();
    }

    pub fn parse_simple_select(&mut self) {
        self.builder.start_node(TokenType::SimpleSelect);
        self.parse_element_name();
        self.parse_hash();
        self.parse_class();
        self.parse_attrib();
        self.parse_pseudo();
        // dbg!(self.peek());
        // // selector function;
        if self.check_token_type(TokenType::Dot) {
            self.advance();
            self.parse_function();
        }

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
            if self.check_token_type(TokenType::HashToken)
                || self.check_token_type(TokenType::PercentageToken)
            {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn parse_class(&mut self) {
        loop {
            if self.check_token_type(TokenType::Dot)
                && self
                    .lexer
                    .check_peek_peek_token_by_type(TokenType::IdentToken)
            {
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
                    match node.r#type {
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
                    match node.r#type {
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

                    self.parse_expr();
                    self.parse_selector_list();
                    self.check_token_and_advance(TokenType::RightParenthesis);

                    self.builder.finish_node();
                }
            } else {
                break;
            }
        }
    }
}
