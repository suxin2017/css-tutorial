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

// ANCHOR: lexer
#[derive(Debug)]
pub struct Lexer<'a> {
    pos_index: usize,
    peek_token: Option<Token>,
    cur_char: Option<char>,
    pub source_code: &'a str,
    source_code_length: usize,
    chars: std::str::CharIndices<'a>,
}
// ANCHOR_END: lexer

impl<'a> Lexer<'a> {
    pub fn new(source_code: &'a str) -> Self {
        let mut lexer = Self {
            chars: source_code.char_indices(),
            pos_index: 0,
            cur_char: None,
            peek_token: None,
            source_code_length: source_code.len(),
            source_code,
        };
        lexer.advance();
        lexer
    }

    pub fn peek_ch(&mut self) -> Option<char> {
        self.chars.clone().nth(0).map(|x| x.1)
    }
    pub fn cur_char(&mut self) -> Option<char> {
        return self.cur_char;
    }
    pub fn advance(&mut self) {
        if let Some((pos, ch)) = self.chars.next() {
            // dbg!(pos, ch);
            self.pos_index = pos;
            self.cur_char = Some(ch);
        } else {
            self.pos_index += 1;
            self.cur_char = None;
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
                            // dbg!(ch, self.peek_ch(), self.cur_char);

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
            match ch {
                ch @ ('(' | ')' | ',' | ':' | ';' | '<' | '>' | '[' | '\\' | ']' | '{' | '}'
                | '=') => {
                    let start_pos = self.pos_index;
                    // 匹配特殊的的符号
                    let token_type = match ch {
                        '(' => TokenType::LeftParenthesis,
                        ')' => TokenType::RightParenthesis,
                        ',' => TokenType::Comma,
                        ':' => TokenType::Colon,
                        ';' => TokenType::Semi,
                        '<' => TokenType::LessThan,
                        '>' => TokenType::MoreThan,
                        '[' => TokenType::LeftSquareBracket,
                        '\\' => TokenType::ReverseSolidus,
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
                '/' => return self.try_comment(),
                '!' => {
                    let start_pos = self.pos_index;
                    self.advance();
                    loop {
                        if self.get_peek_token_by_type(TokenType::Comment) {
                            self.eat_token();
                        } else {
                            break;
                        }
                    }
                    if self.get_peek_token_by_type(TokenType::IdentToken) {
                        self.eat_token();
                        let end_pos = self.pos_index;
                        return Token(TokenType::Important, Range::new(start_pos, end_pos));
                    } else {
                        panic!("get at ! important error")
                    }
                }
                '^' => {
                    let start_pos = self.pos_index;
                    self.advance();
                    if matches!(self.cur_char(), Some('=')) {
                        self.advance();
                        let end_pos = self.pos_index;
                        return Token(TokenType::Exclude, Range::new(start_pos, end_pos));
                    }
                }
                '*' => {
                    let start_pos = self.pos_index;
                    self.advance();
                    if matches!(self.cur_char(), Some('=')) {
                        self.advance();
                        let end_pos = self.pos_index;
                        return Token(TokenType::AllMatch, Range::new(start_pos, end_pos));
                    } else {
                        let end_pos = self.pos_index;
                        return Token(TokenType::Asterisk, Range::new(start_pos, end_pos));
                    }
                }
                '~' => {
                    let start_pos = self.pos_index;
                    self.advance();
                    if matches!(self.cur_char(), Some('=')) {
                        self.advance();
                        let end_pos = self.pos_index;
                        return Token(TokenType::Includes, Range::new(start_pos, end_pos));
                    }
                }
                '|' => {
                    self.advance();
                    let start_pos = self.pos_index;
                    if matches!(self.peek_ch(), Some('=')) {
                        self.advance();
                        let end_pos = self.pos_index;
                        return Token(TokenType::Dashmatch, Range::new(start_pos, end_pos));
                    }
                }
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

                ch if ch.is_ascii_digit() || ch == '.' || ch == '+' || ch == '-' => {
                    let start_pos = self.pos_index;
                    let mut token = self.try_digit();
                    if token.check_type(TokenType::Digital) {
                        if matches!(self.cur_char(), Some('%')) {
                            self.advance();
                            let end_pos = self.pos_index;

                            token =
                                Token(TokenType::PercentageToken, Range::new(start_pos, end_pos))
                        } else if matches!(self.cur_char(),Some(ch) if !ch.is_whitespace() && !matches!(ch,';'|')'))
                        {
                            if self.get_peek_token_by_type(TokenType::IdentToken) {
                                self.eat_token();
                                let end_pos = self.pos_index;
                                return Token(TokenType::Dimension, Range::new(start_pos, end_pos));
                            }
                        }
                    }
                    if token.check_type(TokenType::Minus)
                        && (self.get_peek_token_by_type(TokenType::IdentToken)
                            || self.get_peek_token_by_type(TokenType::FunctionToken))
                    {
                        let token = self.eat_token();
                        let end_pos = self.pos_index;
                        return Token(token.0, Range::new(start_pos, end_pos));
                    }

                    return token;
                }
                _ => {
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
                        continue;
                    }
                    // 	([!#$%&*-~]|{nonascii}|{escape})*
                    match escape_ch {
                        ch if !ch.is_ascii() || ch.is_ascii_alphanumeric() => {
                            self.advance();
                        }
                        '!' | '#' | '$' | '%' | '&' | '*' | '-' | '~' | '.' | '/' | '?' | '_'
                        | '+' => {
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
            if matches!(escape_ch,'a'..='z'|'A'..='Z' | '0'..='9' |'_' |'-'| '\u{0080}'..) {
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

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    macro_rules! test_token {
        ($x:expr,$y:expr) => {
            let mut lexer = Lexer::new($x);
            let token = lexer.eat_token();
            dbg!(token.get_source_code(&$x));
            dbg!(&token);
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
    fn test_ident1_token() {
        test_token!(
            r#" -webkit-linear-gradient(45deg, rgba(255, 255, 255, .15) 25%, transparent 25%, transparent 50%, rgba(255, 255, 255, .15) 50%, rgba(255, 255, 255, .15) 75%, transparent 75%, transparent)"#,
            TokenType::FunctionToken
        );
    }

    #[test]
    fn test_zero_token() {
        test_token!(r#"0"#, TokenType::Digital);
    }

    #[test]
    fn test_num_token() {
        test_token!(r#".1"#, TokenType::Digital);
    }

    #[test]
    fn test_comment_token() {
        test_token!(r#"/** abc */"#, TokenType::Comment);
    }

    #[test]
    fn test_func_token() {
        test_token!(r#"-abc-sadf("#, TokenType::FunctionToken);
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

    #[test]
    fn test_important_token() {
        test_token!(r#" ! important"#, TokenType::Important);
    }

    #[test]
    fn test_plus_token() {
        test_token!(r#"+.checkbo"#, TokenType::Plus);
    }

    #[test]
    fn test_num1_token() {
        test_token!(r#".123"#, TokenType::Digital);
    }

    #[test]
    fn test_length_token() {
        test_token!(r#"123px"#, TokenType::Dimension);
    }

    #[test]
    fn test_body_token() {
        test_token!(r#"body"#, TokenType::IdentToken);
    }

    #[test]
    fn test_ident2_token() {
        test_token!(r#"body-color"#, TokenType::IdentToken);
    }

    #[test]
    fn test_string2_token() {
        test_token!(r#""\002a""#, TokenType::Str);
    }

    #[test]
    fn test_all_match_token() {
        test_token!(r#"*="#, TokenType::AllMatch);
    }

    #[test]
    fn test_url1_token() {
        test_token!(
            r#"url(../fonts\0123/glyphicons-halflings-regular.eot?#iefix)"#,
            TokenType::UrlToken
        );
    }
}
