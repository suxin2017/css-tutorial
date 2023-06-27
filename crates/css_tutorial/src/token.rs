use crate::{
    ast::{AstNode, AstNodeType},
    range::Range,
    token_type::TokenType,
};

// ANCHOR: token
#[derive(Debug, Clone)]
pub struct Token{
    pub r#type:TokenType, 
    loc: Range,
    raw: String,
}

impl Token {
    pub fn check_type(&self, token_type: TokenType) -> bool {
        return self.r#type == token_type;
    }

    pub fn new(token_type: TokenType,loc: Range,raw: String)->Self{
        Self{
            r#type:token_type,
            loc,
            raw 
        }
    }

    pub fn get_source_code<'a>(&'a self) -> &str {
        &self.raw
    }
}

// ANCHOR_END: token

// impl Token {
//     pub fn print_detail(&self, raw: &str) {
//         let prestr = &raw[..self.1.start_pos];
//         let line = prestr.chars().filter(|x| x == &'\n').count();
//         println!(
//             "当前token {:?} 内容 {:?} at line {}",
//             self,
//             self.get_source_code(&raw),
//             line
//         );
//     }
// }

impl From<Token> for AstNode<TokenType> {
    fn from(token: Token) -> Self {
        Self {
            node_type: AstNodeType(token.r#type),
            range: token.loc,
            raw: token.raw,
            children: None,
        }
    }
}
