use crate::token::TokenType;
use crate::token::TokenType::{Comment, HashToken, UrlToken};
use crate::{range::Range, token::Token};

#[derive(Debug)]
pub enum ErrorKind {
    EscapeError,
    DigitError,
    IdentTokenError,
    StringTokenError,
    CommentTokenError,
}

pub(crate) type LexResult<T> = Result<T, ErrorKind>;

// ANCHOR: lexer
#[derive(Debug)]
pub struct Lexer<'a> {
    pos_index: usize,
    peek_token: Option<Token>,
    cur_token: Option<Token>,
    pub source_code: &'a str,
    source_code_length: usize,
    chars: std::str::CharIndices<'a>,
}
// ANCHOR_END: lexer

impl<'a> Lexer<'a> {
    pub fn new(source_code: &'a str) -> Self {
        Self {
            chars: source_code.char_indices(),
            pos_index: 0,
            peek_token: None,
            cur_token: None,
            source_code_length: source_code.len(),
            source_code,
        }
    }

    pub fn peek_ch(&mut self) -> Option<char> {
        let cur_index = self.pos_index;
        // dbg!(
        //     cur_index,
        //     self.chars.clone().nth(cur_index + 1).map(|x| x.1)
        // );
        self.chars.clone().nth(cur_index + 1).map(|x| x.1)
    }

    pub fn advance(&mut self) {
        let cur_index = self.pos_index;
        if cur_index < self.source_code_length {
            self.pos_index += 1;
        }
    }

    pub fn string_token(&mut self) -> Token {
        let mut result = String::new();
        let start_pos = self.pos_index;

        if let Some(ch) = self.cur_char() {
            match ch {
                '\'' => {
                    self.advance();
                    loop {
                        if let Some((escape_ch, _)) = self.escape() {
                            result.push(escape_ch);
                            self.advance();
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
                        if let Some((escape_ch, _)) = self.escape() {
                            result.push(escape_ch);
                            self.advance();
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

    fn try_comment(&mut self) -> Token {
        if let Some(ch) = self.cur_char() {
            if ch == '/' {
                let start_pos = self.pos_index;

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
                }
            }
        }
        panic!("parse comment error")
    }

    pub fn try_digit(&mut self) -> Token {
        let mut result = String::new();
        let start_pos = self.pos_index;
        if let Some(ch) = self.cur_char() {
            if matches!(ch, '+' | '-') {
                result.push(ch);
                self.advance();
            }

            // digit
            if let Some(ch1) = self.cur_char() {
                match ch1 {
                    ch1 if ch1.is_ascii_digit() => {
                        result.push(ch1);
                        self.advance();

                        while let Some(ch1) = self.cur_char() {
                            if ch1.is_ascii_digit() {
                                result.push(ch1);
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        if let Some(ch1) = self.cur_char() {
                            if matches!(ch1, '.') {
                                result.push(ch1);
                                self.advance();
                            }
                        }
                    }
                    '.' => {
                        result.push(ch1);
                        self.advance();
                    }
                    _ => {
                        return if ch == '+' {
                            Token(TokenType::Plus, Range::new(start_pos, start_pos + 1))
                        } else {
                            Token(TokenType::Minus, Range::new(start_pos, start_pos + 1))
                        };
                    }
                }
                if let Some(ch1) = self.cur_char() {
                    if ch1.is_ascii_digit() {
                        result.push(ch1);
                        self.advance();
                    }
                }
                while let Some(ch1) = self.cur_char() {
                    if ch1.is_ascii_digit() {
                        result.push(ch1);
                        self.advance();
                    } else {
                        break;
                    }
                }
                if let Some(ch2) = self.cur_char() {
                    if matches!(ch2, 'e' | 'E') {
                        result.push(ch2);
                        self.advance();
                        if let Some(ch1) = self.cur_char() {
                            if matches!(ch1, '+' | '-') {
                                result.push(ch1);
                                self.advance();
                            }
                        }
                        while let Some(ch1) = self.cur_char() {
                            if ch1.is_ascii_digit() {
                                result.push(ch1);
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
        panic!("parse digit error");
    }

    pub fn cur_char(&mut self) -> Option<char> {
        let cur_index = self.pos_index;
        if cur_index < self.source_code_length {
            if let Some((_, cur_char)) = self.source_code.char_indices().clone().nth(cur_index) {
                // dbg!(cur_char);
                return Some(cur_char);
            }
        }
        None
    }

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

    pub fn get_peek_token(&mut self) -> Option<Token> {
        if let None = self.peek_token {
            self.peek_token = Some(self.get_token());
        } else {
            return self.peek_token;
        }
        return self.peek_token;
    }

    pub fn eat_token(&mut self) -> Token {
        if let Some(peek_token) = self.peek_token {
            self.peek_token = None;
            return peek_token;
        }
        return self.get_token();
    }

    pub fn get_peek_token_by_type(&mut self, token_type: TokenType) -> bool {
        if let Some(token) = self.get_peek_token() {
            return token.check_type(token_type);
        }
        false
    }
    //ANCHOR:get_token
    pub fn get_token(&mut self) -> Token {
        while let Some(ch) = self.cur_char() {
            if ch.is_whitespace() {
                self.advance();
                continue;
            }
            dbg!(ch);
            match ch {
                ch @ ('(' | ')' | ',' | '.' | ':' | ';' | '<' | '[' | '\\' | ']' | '{' | '}') => {
                    let start_pos = self.pos_index;
                    // 匹配特殊的的符号
                    let token_type = match ch {
                        '(' => TokenType::LeftParenthesis,
                        ')' => TokenType::RightParenthesis,
                        ',' => TokenType::Comma,
                        '.' => TokenType::Dot,
                        ':' => TokenType::Colon,
                        ';' => TokenType::Semi,
                        '<' => TokenType::LessThan,
                        '[' => TokenType::LeftSquareBracket,
                        '\\' => TokenType::ReverseSolidus,
                        ']' => TokenType::RightCurlyBracket,
                        '{' => TokenType::LeftCurlyBracket,
                        '}' => TokenType::RightCurlyBracket,
                        _ => {
                            panic!("不可能发生")
                        }
                    };

                    self.advance();

                    let end_pos = self.pos_index;
                    return Token(token_type, Range::new(start_pos, end_pos));
                }
                '/' => return self.try_comment(),

                // 获取字符串
                '\'' | '"' => return self.string_token(),
                '@' => {
                    let start_pos = self.pos_index;
                    self.advance();
                    if self.get_peek_token_by_type(TokenType::IdentToken) {
                        self.eat_token();
                        let end_pos = self.pos_index;
                        return Token(TokenType::AtKeywordToken, Range::new(start_pos, end_pos));
                    } else {
                        panic!("get at key word error")
                    }
                }
                '#' => {
                    let start_pos = self.pos_index;
                    self.advance();
                    self.match_word();
                    let end_pos = self.pos_index;
                    return Token(HashToken, Range::new(start_pos, end_pos));
                }

                ch if ch.is_ascii_digit() || ch == '+' || ch == '-' => {
                    let start_pos = self.pos_index;
                    let mut token = self.try_digit();

                    if matches!(self.peek_ch(), Some('%')) {
                        self.advance();
                        let end_pos = self.pos_index;

                        token = Token(TokenType::PercentageToken, Range::new(start_pos, end_pos))
                    }
                    return token;
                }
                _ => {
                    let start_pos = self.pos_index;
                    let mut token = self.ident_token();
                    dbg!(token.get_source_code(self.source_code));
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
            }
        }

        return Token(TokenType::EOF, Range::new(self.pos_index, self.pos_index));
    }

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
                        self.advance();
                        continue;
                    }
                    // 	([!#$%&*-~]|{nonascii}|{escape})*
                    match escape_ch {
                        ch if !ch.is_ascii() || ch.is_ascii_alphanumeric() => {
                            self.advance();
                        }
                        '!' | '#' | '$' | '%' | '&' | '*' | '-' | '~' | '.' => {
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

    fn match_word(&mut self) {
        while let Some((escape_ch, _)) = self.escape() {
            if matches!(escape_ch,'a'..='z'|'A'..='Z'|'_' | '\u{0080}'..) {
                // result.push(escape_ch);
                // dbg!(escape_ch);
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

                loop {
                    if let Some(ch) = self.cur_char() {
                        match ch {
                            'a'..='f' | 'A'..='F' | '0'..='9' => {
                                result.push(ch);
                                self.advance();
                            }
                            _ => break,
                        }
                    }
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
                    panic!("ident token is empty")
                }
            }
            let end_pos = self.pos_index;
            return Token(TokenType::IdentToken, Range::new(start_pos, end_pos));
        }
        let end_pos = self.pos_index;
        return Token(TokenType::IdentToken, Range::new(start_pos, end_pos));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_token {
        ($x:expr,$y:expr) => {
            let mut lexer = Lexer::new($x);
            let token = lexer.eat_token();
            assert!(token.check_type($y));
        };
    }

    #[test]
    fn test_comment() {
        test_token!(
            r#"/*
        sadfadf
        */"#,
            TokenType::Comment
        );
    }

    #[test]
    fn test_ident_token() {
        test_token!(r#"abc"#, TokenType::IdentToken);
    }

    #[test]
    fn test_func_token() {
        test_token!(r#"abc("#, TokenType::FunctionToken);
    }

    #[test]
    fn test_at_token() {
        test_token!(r#"@abc"#, TokenType::AtKeywordToken);
    }

    #[test]
    fn test_string_token() {
        test_token!(r#""abc""#, TokenType::Str);
    }

    #[test]
    fn test_url_token() {
        test_token!(r#" url(links.css)"#, TokenType::UrlToken);
    }

    #[test]
    fn test_complexe_url_token() {
        test_token!(
            r#" url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='90' height='45'%3E%3Cpath d='M10 10h60' stroke='%2300F' stroke-width='5'/%3E%3Cpath d='M10 20h60' stroke='%230F0' stroke-width='5'/%3E%3Cpath d='M10 30h60' stroke='red' stroke-width='5'/%3E%3C/svg%3E")"#,
            TokenType::UrlToken
        );
    }
}
