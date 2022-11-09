use crate::{range::Range, token::Token};

#[derive(Debug)]
pub enum ErrorKind {
    EscapeError(Range),
    DigitError(Range),
    IdentTokenError(Range),
    StringTokenError(Range),
}
pub(crate) type LexResult<T> = Result<T, ErrorKind>;

// ANCHOR: lexer
pub struct Lexer<'a> {
    range: Range,
    source_code: &'a str,
    source_code_length: usize,
    chars: std::str::CharIndices<'a>,
}
// ANCHOR_END: lexer

impl<'a> Lexer<'a> {
    pub fn new(source_code: &'a str) -> Self {
        Self {
            range: Range::default(),
            chars: source_code.char_indices(),
            source_code_length: source_code.len(),
            source_code,
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        let cur_index = self.range.index();
        self.chars.clone().nth(cur_index + 1).map(|x| x.1)
    }

    pub fn advance(&mut self) {
        let cur_index = self.range.index();
        if cur_index < self.source_code_length {
            let cur_char = self.chars.nth(cur_index).map(|c| c.1);
            self.range.advance_start();
            if let Some(cur_char) = cur_char {
                match cur_char {
                    '\n' => {
                        self.range.advance_start_line();
                    }
                    '\r' => {
                        self.range.advance_start_line();
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn escape_char(&mut self) -> LexResult<char> {
        todo!()
    }
    pub fn string_token(&mut self) -> LexResult<Token> {
        let mut result = String::new();

        if let Some(ch) = self.cur_char() {
            let mut f = |ch: char| -> bool {
                if let Some((escape_ch, is_escape)) = self.escape() {
                    result.push(escape_ch);
                    if escape_ch == ch && is_escape {
                        return true;
                    }
                } else {
                    return true;
                }
                return false;
            };
            match ch {
                '\'' => loop {
                    if f('\'') {
                        break;
                    }
                },
                '"' => loop {
                    if f('"') {
                        break;
                    }
                },
                _ => {}
            }
        }

        Ok(Token::Str(result, self.range))
    }
    pub fn try_digit(&mut self) -> LexResult<Token> {
        let mut result = String::new();

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
                            Ok(Token::Plus(self.range))
                        } else {
                            Ok(Token::Minus(self.range))
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
            return Ok(Token::Digital(result, self.range));
        }

        Err(ErrorKind::DigitError(self.range))
    }

    pub fn cur_char(&mut self) -> Option<char> {
        let cur_index = self.range.index();
        if cur_index < self.source_code_length {
            if let Some((_, cur_char)) = self.source_code.char_indices().clone().nth(cur_index) {
                // !("{:#?}{cur_index}", cur_char);
                dbg!(cur_char);
                return Some(cur_char);
            }
        }
        None
    }
    //ANCHOR:get_token
    pub fn get_token(&mut self) -> LexResult<Token> {
        while let Some(ch) = self.cur_char() {
            if ch.is_whitespace() {
                self.advance();
                continue;
            }
            match ch {
                '#' | '(' | ')' | ',' | '.' | ':' | ';' | '<' | '@' | '[' | '\\' | ']' | '{'
                | '}' => {
                    // 匹配特殊的的符号
                    let token = match ch {
                        '#' => Some(Token::NumberSign(self.range)),
                        '(' => Some(Token::LeftParen(self.range)),
                        ')' => Some(Token::RightParen(self.range)),
                        ',' => Some(Token::Comma(self.range)),
                        '.' => Some(Token::Dot(self.range)),
                        ':' => Some(Token::Colon(self.range)),
                        ';' => Some(Token::Semi(self.range)),
                        '<' => Some(Token::LessThan(self.range)),
                        '@' => Some(Token::At(self.range)),
                        '[' => Some(Token::LeftSquareBracket(self.range)),
                        '\\' => Some(Token::ReverseSolidus(self.range)),
                        ']' => Some(Token::RightCurlyBracket(self.range)),
                        '{' => Some(Token::LeftCurlyBracket(self.range)),
                        '}' => Some(Token::RightCurlyBracket(self.range)),
                        _ => None,
                    };

                    if token.is_some() {
                        self.advance();
                        return Ok(token.unwrap());
                    }
                }
                // 获取数字
                '+' | '-' => return self.try_digit(),

                // 获取字符串
                '\'' | '"' => return self.string_token(),

                ch if ch.is_ascii_digit() => return self.try_digit(),
                _ => return self.ident_token(),
            }
        }

        return Ok(Token::EOF);
    }
    //ANCHOR_END: get_token

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
                self.advance();
                return Some((ch, false));
            }
        }
        None
    }

    fn ident_token(&mut self) -> Result<Token, ErrorKind> {
        let mut result = String::new();

        while let Some(ch) = self.cur_char() {
            if ch.is_whitespace() {
                return Ok(Token::IdentToken(result, self.range));
            } else {
                result.push(ch);
                self.advance();
            }
        }

        Err(ErrorKind::IdentTokenError(self.range))
    }
}

#[cfg(test)]
mod tests {
    use crate::token;

    use super::*;

    #[test]
    fn test_symbol_token() {
        let mut lexer = Lexer::new(r#""\'()+,-.:;<@[\]{}""#);
        loop {
            match lexer.get_token() {
                token => {
                    if let Ok(token) = token {
                        println!("{:#?}", token);

                        if Token::EOF == token {
                            break;
                        }
                    } else {
                        println!("{:#?}", token);
                        break;
                    }
                }
            }
        }

        assert!(true)
    }

    #[test]
    fn test_digit_token() {
        let mut lexer = Lexer::new(r#"1.1e"#);
        match lexer.get_token() {
            Ok(token) => {
                println!("{}", token.str());
            }
            Err(e) => {
                println!("{:#?}", e);
            }
        }
        assert!(true)
    }

    #[test]
    fn test_peek() {
        let mut lexer = Lexer::new(r#"1.1e"#);
        let mut c = "123456".char_indices();
        println!("{}", c.clone().nth(0).unwrap().1);
        println!("{}", c.clone().nth(0).unwrap().1);
        println!("{}", c.clone().nth(0).unwrap().1);
        println!("{}", c.clone().nth(0).unwrap().1);
        println!("{}", c.nth(0).unwrap().1);
        println!("{}", c.nth(0).unwrap().1);
        println!("{}", c.nth(0).unwrap().1);
        println!("{}", c.nth(0).unwrap().1);
        // assert_eq!(Some('.'), lexer.peek());
        // lexer.advance();
        // assert_eq!(Some('1'), lexer.peek());
        // lexer.advance();
        // assert_eq!(Some('.'), lexer.peek());
        // lexer.advance();
        // assert_eq!(Some('.'), lexer.peek());
    }

    #[test]
    fn test_string() {
        let mut lexer = Lexer::new(
            r#""abc
        ""#,
        );
        match lexer.get_token() {
            Ok(token) => {
                println!("{}", token.str());
            }
            Err(e) => {
                println!("{:#?}", e);
            }
        }
    }
}
