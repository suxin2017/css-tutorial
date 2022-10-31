use crate::token::Token;

#[derive(Debug)]
pub enum ErrorKind {
    EscapeError(Range),
    DigitError(Range),
}

pub(crate) type LexResult<T> = Result<T, ErrorKind>;

pub struct Lexer {
    range: Range,
    source_code: String,
    source_code_length: usize,
}

impl Lexer {
    pub fn new(source_code: String) -> Self {
        Self {
            range: Range::default(),
            source_code_length: source_code.len(),
            source_code,
        }
    }

    pub fn cur_char(&mut self) -> Option<char> {
        let cur_index = self.range.index();
        if cur_index < self.source_code_length {
            let cur_char = self.source_code.chars().nth(cur_index);
            println!("{:#?}{cur_index}", cur_char);
            return cur_char;
        }
        None
    }

    pub fn advance(&mut self) {
        let cur_index = self.range.index();
        if cur_index < self.source_code_length {
            let cur_char = self.source_code.chars().nth(cur_index);
            self.range.advance_start();
            if let Some(cur_char) = cur_char {
                if cur_char == '\n' {
                    self.range.advance_start_line();
                }
            }
        }
    }

    pub fn escape_char(&mut self) -> LexResult<char> {
        todo!()
    }
    pub fn string_token(&mut self) -> LexResult<Token> {
        // open sign
        // if !self.pause_start_advance {
        //     self.pause_start_advance = true;
        // } else {
        // }
        todo!()
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

    pub fn next_token(&mut self) -> LexResult<Token> {
        while let Some(ch) = self.cur_char() {
            if ch.is_whitespace() {
                self.advance();
                continue;
            }
            match ch {
                // '\'' | '"' => return self.string_token(),
                '#' => return Ok(Token::NumberSign(self.range)),
                '(' => return Ok(Token::LeftParen(self.range)),
                ')' => return Ok(Token::RightParen(self.range)),
                '+' | '-' => return self.try_digit(),
                ch if ch.is_ascii_digit() => return self.try_digit(),
                ',' => return Ok(Token::Comma(self.range)),
                '.' => return Ok(Token::Dot(self.range)),
                ':' => return Ok(Token::Colon(self.range)),
                ';' => return Ok(Token::Semi(self.range)),
                '<' => return Ok(Token::LessThan(self.range)),
                '@' => return Ok(Token::At(self.range)),
                '[' => return Ok(Token::LeftSquareBracket(self.range)),
                '\\' => return Ok(Token::ReverseSolidus(self.range)),
                ']' => return Ok(Token::RightCurlyBracket(self.range)),
                '{' => return Ok(Token::LeftCurlyBracket(self.range)),
                '}' => return Ok(Token::RightCurlyBracket(self.range)),

                _ => (),
            }
            self.advance();
        }

        return Ok(Token::EOF);
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Range {
    start_pos_index: usize,
    end_pos_index: usize,
    start_index: usize,
    start_line: usize,
    end_index: usize,
    end_line: usize,
}

impl Range {
    pub fn index(&self) -> usize {
        if self.end_pos_index > self.start_pos_index {
            return self.end_pos_index;
        }
        self.start_pos_index
    }

    pub fn advance_start(&mut self) -> usize {
        self.start_pos_index += 1;
        self.start_index += 1;
        self.start_pos_index
    }

    pub fn advance_start_line(&mut self) {
        self.start_line += 1;
        self.start_index = 0;
    }

    pub fn advance_end(&mut self) -> usize {
        self.end_pos_index += 1;
        self.end_index += 1;
        self.end_pos_index
    }
    pub fn advance_end_line(&mut self) {
        self.end_line += 1;
        self.end_index = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_token() {
        let mut lexer = Lexer::new(r#""\'()+,-.:;<@[\]{}"#.to_string());
        while let Ok(token) = lexer.next_token() {
            println!("{:#?}", token);

            if Token::EOF == token {
                break;
            }
        }
        assert!(true)
    }

    #[test]
    fn test_digit_token() {
        let mut lexer = Lexer::new(r#"123 4"#.to_string());
        match lexer.next_token() {
            Ok(token) => {
                println!("{}", token.str());
            }
            Err(e) => {
                println!("{:#?}", e);
            }
        }
        assert!(true)
    }
}
