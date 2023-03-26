use core::panic;
use std::collections::VecDeque;

use crate::token_type::TokenType::{self, HashToken};
use crate::{range::Range, token::Token};

#[derive(Debug)]
pub enum ErrorKind {
    EscapeError,
    DigitError,
    IdentTokenError,
    StringTokenError,
    CommentTokenError,
}

// ANCHOR: lexer
#[derive(Debug)]
pub struct Lexer<'a> {
    /** 当前光标位置 */
    pos_index: usize,
    /** 下一个Token */
    peek_token: Option<Token>,
    peek_peek_token: Option<Token>,

    /** 当前字符 */
    cur_char: Option<char>,
    /** 字符切片 */
    chars: std::str::CharIndices<'a>,

    /** 原始输入*/
    pub source_code: &'a str,
}
// ANCHOR_END: lexer

impl<'a> Lexer<'a> {
    // ANCHOR: new
    pub fn new(source_code: &'a str) -> Self {
        let mut lexer = Self {
            chars: source_code.char_indices(),
            pos_index: 0,
            cur_char: None,
            peek_token: None,
            peek_peek_token: None,
            source_code,
        };
        lexer.advance();
        lexer
    }
    //ANCHOR_END: new
    // ANCHOR:  handle_char
    // 查看前一个字符
    pub fn peek_ch(&mut self) -> Option<char> {
        // 思考：有没有什么更好的优化手段？
        self.chars.clone().nth(0).map(|x| x.1)
    }
    // 查看当前字符
    pub fn cur_char(&mut self) -> Option<char> {
        return self.cur_char;
    }
    // 移动光标
    pub fn advance(&mut self) {
        if let Some((pos, ch)) = self.chars.next() {
            self.pos_index = pos;
            self.cur_char = Some(ch);
        } else {
            self.pos_index += 1;
            self.cur_char = None;
        }
    }
    // ANCHOR_END:  handle_char

    pub fn get_peek_token(&mut self) -> Option<&Token> {
        if let None = self.peek_token {
            if let Some(peek_peek_token) = self.peek_peek_token {
                self.peek_peek_token = None;
                self.peek_token = Some(peek_peek_token);
            } else {
                self.peek_token = Some(self.get_token());
            }
        } else {
            return self.peek_token.as_ref();
        }
        return self.peek_token.as_ref();
    }

    pub fn get_peek_peek_token(&mut self) -> Option<&Token> {
        if let None = self.peek_peek_token {
            self.peek_peek_token = Some(self.get_token());
        } else {
            return self.peek_peek_token.as_ref();
        }
        return self.peek_peek_token.as_ref();
    }

    pub fn eat_token(&mut self) -> Token {
        if let Some(peek_token) = self.peek_token {
            self.peek_token = None;
            return peek_token;
        }
        if let Some(peek_peek_token) = self.peek_peek_token {
            self.peek_peek_token = None;
            return peek_peek_token;
        }
        let token = self.get_token();
        return token;
    }

    pub fn check_peek_token_by_type(&mut self, token_type: TokenType) -> bool {
        if let Some(token) = self.get_peek_token() {
            return token.check_type(token_type);
        }
        false
    }
    pub fn check_peek_peek_token_by_type(&mut self, token_type: TokenType) -> bool {
        if let Some(token) = self.get_peek_peek_token() {
            return token.check_type(token_type);
        }
        false
    }
    //ANCHOR:get_token
    fn get_token(&mut self) -> Token {
        while let Some(ch) = self.cur_char() {
            if ch.is_whitespace() {
                self.advance();
                continue;
            }
            match ch {
                '(' | ')' | ',' | ':' | ';' | '<' | '>' | '[' | ']' | '{' | '}' | '=' => {
                    return self.parse_simple_symbol(ch)
                }
                '/' => return self.try_comment(),
                '!' => return self.parse_exclamation(),
                ch @ ('^' | '*' | '~' | '|') => return self.parse_attr_rule(ch),
                '\'' | '"' => return self.string_token(),
                '@' => {
                    return self.parse_at_word();
                }
                '#' => return self.parse_hash(),
                ch if ch.is_ascii_digit() || ch == '.' || ch == '+' || ch == '-' => {
                    return self.parse_digit_token();
                }
                _ => {
                    return self.parse_ident_token();
                }
            }
        }

        return Token(TokenType::EOF, Range::new(self.pos_index, self.pos_index));
    }
    // ANCHOR_END: get_token

    pub fn string_token(&mut self) -> Token {
        let mut result = String::new();
        let start_pos = self.pos_index;

        if let Some(ch) = self.cur_char() {
            match ch {
                '\'' => {
                    self.advance();
                    loop {
                        if let Some((escape_ch, is_escape)) = self.escape() {
                            result.push(escape_ch);
                            if !is_escape {
                                self.advance();
                            }
                            if escape_ch == '\'' {
                                let end_pos = self.pos_index;
                                return Token(TokenType::Str, Range::new(start_pos, end_pos));
                            }
                        } else {
                            panic!("parse string token error");
                        }
                    }
                }
                '"' => {
                    self.advance();
                    loop {
                        if let Some((escape_ch, is_escape)) = self.escape() {
                            result.push(escape_ch);
                            if !is_escape {
                                self.advance();
                            }
                            if escape_ch == '"' {
                                let end_pos = self.pos_index;
                                return Token(TokenType::Str, Range::new(start_pos, end_pos));
                            }
                        } else {
                            panic!("parse string token error");
                        }
                    }
                }
                _ => {}
            }
        }

        panic!("parse string token error")
    }

    // ANCHOR: try_comment
    fn try_comment(&mut self) -> Token {
        if let Some(ch) = self.cur_char() {
            let start_pos = self.pos_index;

            if ch == '/' {
                if matches!(self.peek_ch(), Some('*')) {
                    self.advance();
                    self.advance();

                    while let Some(ch) = self.cur_char() {
                        if ch == '*' && matches!(self.peek_ch(), Some('/')) {
                            self.advance();
                            self.advance();

                            let end_pos = self.pos_index;
                            return Token(TokenType::Comment, Range::new(start_pos, end_pos));
                        } else {
                            self.advance()
                        }
                    }
                } else {
                    self.advance();
                    let end_pos = self.pos_index;
                    return Token(TokenType::ForwardSlash, Range::new(start_pos, end_pos));
                }
            }
        }
        panic!("parse comment error")
    }
    // ANCHOR_END: try_comment

    // ANCHOR:try_digit
    pub fn try_digit(&mut self) -> Token {
        let start_pos = self.pos_index;
        if let Some(ch) = self.cur_char() {
            if matches!(ch, '+' | '-' | '.') {
                self.advance();
            }

            // digit
            if let Some(ch1) = self.cur_char() {
                match ch1 {
                    ch1 if ch1.is_ascii_digit() => {
                        self.advance();

                        while let Some(ch1) = self.cur_char() {
                            if ch1.is_ascii_digit() {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        if let Some(ch1) = self.cur_char() {
                            if matches!(ch1, '.') {
                                self.advance();
                            }
                        }
                    }
                    '.' => {
                        if let Some(peek_ch) = self.peek_ch() {
                            if peek_ch.is_ascii_digit() {
                                self.advance();
                            } else {
                                return Token(
                                    TokenType::Plus,
                                    Range::new(start_pos, start_pos + 1),
                                );
                            }
                        }
                    }
                    _ => {
                        return if ch == '+' {
                            Token(TokenType::Plus, Range::new(start_pos, start_pos + 1))
                        } else if ch == '.' {
                            Token(TokenType::Dot, Range::new(start_pos, start_pos + 1))
                        } else {
                            Token(TokenType::Minus, Range::new(start_pos, start_pos + 1))
                        };
                    }
                }
                if let Some(ch1) = self.cur_char() {
                    if ch1.is_ascii_digit() {
                        self.advance();
                    }
                }
                while let Some(ch1) = self.cur_char() {
                    if ch1.is_ascii_digit() {
                        self.advance();
                    } else {
                        break;
                    }
                }
                if let Some(ch2) = self.cur_char() {
                    if matches!(ch2, 'e' | 'E') {
                        self.advance();
                        if let Some(ch1) = self.cur_char() {
                            if matches!(ch1, '+' | '-') {
                                self.advance();
                            }
                        }
                        while let Some(ch1) = self.cur_char() {
                            if ch1.is_ascii_digit() {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                }
            }

            let end_pos = self.pos_index;
            return Token(TokenType::Digital, Range::new(start_pos, end_pos));
        }
        panic!("parse digit error {:?} {:?}", self.cur_char, self.pos_index);
    }
    //ANCHOR_END:try_digit

    pub fn skip_whitespace(&mut self) {
        loop {
            if let Some(ch) = self.cur_char() {
                if ch.is_whitespace() {
                    self.advance()
                } else {
                    break;
                }
            }
        }
    }

    fn parse_ident_token(&mut self) -> Token {
        let start_pos = self.pos_index;
        let mut token = self.ident_token();
        if let Some(value) = self.parse_url_token(&mut token, start_pos) {
            return value;
        }
        if matches!(self.cur_char(), Some('(')) {
            self.advance();
            let end_pos = self.pos_index;
            token = Token(TokenType::FunctionToken, Range::new(start_pos, end_pos));
        };
        return token;
    }
    fn parse_at_word(&mut self) -> Token {
        let start_pos = self.pos_index;
        self.advance();

        self.ident_token();

        let end_pos = self.pos_index;
        return Token(TokenType::AtKeywordToken, Range::new(start_pos, end_pos));
    }

    fn parse_hash(&mut self) -> Token {
        let start_pos = self.pos_index;
        self.advance();
        self.match_word();
        let end_pos = self.pos_index;
        return Token(HashToken, Range::new(start_pos, end_pos));
    }

    fn parse_digit_token(&mut self) -> Token {
        let start_pos = self.pos_index;
        let mut token = self.try_digit();
        if token.check_type(TokenType::Digital) {
            if matches!(self.cur_char(), Some('%')) {
                self.advance();
                let end_pos = self.pos_index;

                token = Token(TokenType::PercentageToken, Range::new(start_pos, end_pos))
            } else if matches!(self.cur_char(),Some(ch) if !ch.is_whitespace() && !matches!(ch,';'|')'|'}'))
            {
                if self.check_peek_token_by_type(TokenType::IdentToken) {
                    self.eat_token();
                    let end_pos = self.pos_index;
                    return Token(TokenType::Dimension, Range::new(start_pos, end_pos));
                }
            }
        }
        if token.check_type(TokenType::Minus) {
            if let Some(ch) = self.cur_char() {
                if !ch.is_whitespace() && self.check_ch(ch) {
                    if self.check_peek_token_by_type(TokenType::IdentToken)
                        || self.check_peek_token_by_type(TokenType::FunctionToken)
                    {
                        let token = self.eat_token();
                        let end_pos = self.pos_index;
                        return Token(token.0, Range::new(start_pos, end_pos));
                    }
                }
            }
        }
        return token;
    }

    fn parse_attr_rule(&mut self, ch: char) -> Token {
        let start_pos = self.pos_index;
        self.advance();
        let end_pos = self.pos_index;

        if matches!(self.cur_char(), Some('=')) {
            self.advance();
            let end_pos = self.pos_index;

            if ch == '^' {
                return Token(TokenType::Exclude, Range::new(start_pos, end_pos));
            } else if ch == '*' {
                return Token(TokenType::AllMatch, Range::new(start_pos, end_pos));
            } else if ch == '~' {
                return Token(TokenType::Includes, Range::new(start_pos, end_pos));
            } else {
                return Token(TokenType::Dashmatch, Range::new(start_pos, end_pos));
            }
        } else if ch == '*' {
            return Token(TokenType::Asterisk, Range::new(start_pos, end_pos));
        } else if ch == '~' {
            return Token(TokenType::Wave, Range::new(start_pos, end_pos));
        }

        panic!("parse attr rule error {}", self.pos_index);
    }

    fn parse_exclamation(&mut self) -> Token {
        let start_pos = self.pos_index;
        self.advance();
        loop {
            if self.check_peek_token_by_type(TokenType::Comment) {
                self.eat_token();
            } else {
                break;
            }
        }
        if self.check_peek_token_by_type(TokenType::IdentToken) {
            self.eat_token();
            let end_pos = self.pos_index;
            return Token(TokenType::Important, Range::new(start_pos, end_pos));
        }
        panic!("get at ! important error")
    }
    // ANCHOR: parse_simple_symbol
    fn parse_simple_symbol(&mut self, ch: char) -> Token {
        let start_pos = self.pos_index;
        let token_type = match ch {
            '(' => TokenType::LeftParenthesis,
            ')' => TokenType::RightParenthesis,
            ',' => TokenType::Comma,
            ':' => TokenType::Colon,
            ';' => TokenType::Semi,
            '<' => TokenType::LessThan,
            '>' => TokenType::MoreThan,
            '[' => TokenType::LeftSquareBracket,
            ']' => TokenType::RightSquareBracket,
            '{' => TokenType::LeftCurlyBracket,
            '}' => TokenType::RightCurlyBracket,
            '=' => TokenType::Equal,
            _ => {
                panic!("不可能发生")
            }
        };
        self.advance();
        let end_pos = self.pos_index;
        return Token(token_type, Range::new(start_pos, end_pos));
    }
    // ANCHOR_END: parse_simple_symbol

    fn parse_url_token(&mut self, token: &mut Token, start_pos: usize) -> Option<Token> {
        if token.get_source_code(self.source_code) == "url" {
            if matches!(self.cur_char(), Some('(')) {
                self.advance();
            } else {
                panic!("expect char '('")
            }

            self.skip_whitespace();

            if matches!(self.cur_char(), Some('\'') | Some('"')) {
                match self.get_token() {
                    token if token.check_type(TokenType::Str) => {}
                    _ => panic!("get at key word error"),
                }
            } else {
                while let Some((escape_ch, is_escape)) = self.escape() {
                    if is_escape {
                        continue;
                    }
                    // 	([!#$%&*-~]|{nonascii}|{escape})*
                    match escape_ch {
                        ch if !ch.is_ascii() || ch.is_ascii_alphanumeric() => {
                            self.advance();
                        }
                        '!' | '#' | '$' | '%' | '&' | '*' | '-' | '~' | '.' | '/' | '?' | '_'
                        | ':' | ',' | '=' | ';' | '+' => {
                            self.advance();
                        }
                        ')' => {
                            break;
                        }
                        ch => {
                            panic!("bad url token {}", ch);
                        }
                    }
                }
            }

            self.skip_whitespace();

            if matches!(self.cur_char(), Some(')')) {
                self.advance();
                let end_pos = self.pos_index;

                *token = Token(TokenType::UrlToken, Range::new(start_pos, end_pos));
            } else {
                panic!("parse url token error")
            }
        }
        None
    }
    //ANCHOR_END: get_token

    fn check_ch(&self, ch: char) -> bool {
        return matches!(ch,'a'..='z'|'A'..='Z' | '0'..='9' |'_' |'-'| '\u{0080}'..);
    }
    fn match_word(&mut self) {
        while let Some((escape_ch, is_escape)) = self.escape() {
            if is_escape {
            } else if self.check_ch(escape_ch) {
                self.advance();
            } else {
                break;
            }
        }
    }
    fn escape(&mut self) -> Option<(char, bool)> {
        if let Some(ch) = self.cur_char() {
            if ch == '\\' {
                let mut result = String::new();
                self.advance();
                if let Some(ch) = self.cur_char() {
                    match ch {
                        '\n' | '\r' | 'a'..='f' | 'A'..='F' | '0'..='9' => {}
                        ch => {
                            self.advance();
                            return Some((ch, true));
                        }
                    }
                }
                let mut count = 0;

                loop {
                    if count == 6 {
                        break;
                    }
                    if let Some(ch) = self.cur_char() {
                        match ch {
                            'a'..='f' | 'A'..='F' | '0'..='9' => {
                                result.push(ch);
                                self.advance();
                            }
                            _ => break,
                        }
                    }
                    count += 1;
                }
                if let Some(ch) = self.cur_char() {
                    if ch.is_whitespace() {
                        self.advance();
                    }
                }
                return Some((
                    char::from_u32(u32::from_str_radix(result.as_str(), 16).unwrap()).unwrap(),
                    true,
                ));
            } else {
                return Some((ch, false));
            }
        }
        None
    }

    fn ident_token(&mut self) -> Token {
        let start_pos = self.pos_index;

        while let Some(ch) = self.cur_char() {
            if ch == '_' {
                self.advance();
                self.match_word();
            } else if ch == '-' && matches!(self.peek_ch(), Some('-')) {
                self.advance();
                self.advance();
                self.match_word();
            } else {
                self.match_word();
                let end_pos = self.pos_index;
                if start_pos == end_pos {
                    panic!("ident token is empty {}", ch)
                }
            }
            let end_pos = self.pos_index;
            return Token(TokenType::IdentToken, Range::new(start_pos, end_pos));
        }
        let end_pos = self.pos_index;
        return Token(TokenType::IdentToken, Range::new(start_pos, end_pos));
    }
}
