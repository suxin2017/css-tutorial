use crate::{
    ast::{AstNode, AstNodeType},
    range::Range,
    token_type::TokenType,
};

// ANCHOR: token
#[derive(Debug, Clone, Copy)]
pub struct Token(pub TokenType, pub Range);

impl Token {
    pub fn check_type(&self, token_type: TokenType) -> bool {
        return self.0 == token_type;
    }

    pub fn get_source_code<'a>(&'a self, raw: &'a str) -> &str {
        &raw[self.1.start_pos..self.1.end_pos]
    }
}

// ANCHOR_END: token

impl Token {
    pub fn print_detail(&self, raw: &str) {
        let prestr = &raw[..self.1.start_pos];
        let line = prestr.chars().filter(|x| x == &'\n').count();
        println!(
            "当前token {:?} 内容 {:?} at line {}",
            self,
            self.get_source_code(&raw),
            line
        );
    }
}

impl From<Token> for AstNode<TokenType> {
    fn from(token: Token) -> Self {
        Self {
            node_type: AstNodeType(token.0),
            range: token.1,
            children: None,
        }
    }
}
