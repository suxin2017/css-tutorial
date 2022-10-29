use std::str::Chars;

use crate::token::Token;

pub enum ErrorKind {
    EscapeError,
}

pub(crate) type LexResult<T> = Result<T, ErrorKind>;

pub struct Lexer {
    range: Range,
    pause_start_advance: bool,
    source_code: String,
}

impl Lexer {
    pub fn new(source_code: String) -> Self {
        Self {
            source_code,
            pause_start_advance: false,
            range: Range::default(),
        }
    }

    pub fn cur_char(&mut self) -> Option<char> {
        let cur_index = self.range.index();
        if cur_index < self.source_code.len() {
            let cur_char = self.source_code.remove(cur_index);

            self.range.advance_start();
            if cur_char == '\n' {
                self.range.advance_start_line();
            }
            return Some(cur_char);
        }
        None
    }

    pub fn escape_char(&mut self) -> LexResult<char> {
		
		todo!()
	}
    pub fn string_token(&mut self) -> LexResult<Token> {
		// open sign
		if !self.pause_start_advance {
			self.pause_start_advance = true;
		}else{

		}
        todo!()
    }

    pub fn next_token(&mut self) -> LexResult<Token> {
        while let Some(ch) = self.cur_char() {
            if ch.is_whitespace() {
                continue;
            }
            match ch {
                ';' => return Ok(Token::SEMI(self.range)),
                ',' => return Ok(Token::COMMA(self.range)),
                '.' => return Ok(Token::DOT(self.range)),
                ':' => return Ok(Token::COLON(self.range)),
                '(' => return Ok(Token::LPAREN(self.range)),
                '\'' | '"' => return self.string_token(),
                _ => (),
            }
        }

        return Ok(Token::EOF);
    }
}

#[derive(Default, Clone, Copy)]
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

