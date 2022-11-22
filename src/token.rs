use serde::{Deserialize, Serialize};

use crate::{
    ast::{AstNode, AstNodeType},
    range::Range,
};

#[derive(Debug, Clone, Copy)]
pub struct Token(pub TokenType, pub Range);

impl Token {
    pub fn check_type(&self, token_type: TokenType) -> bool {
        dbg!(self.0 == token_type);
        return self.0 == token_type;
    }

    pub fn get_source_code<'a>(&'a self, raw: &'a str) -> &str {
        &raw[self.1.start_pos_index..self.1.end_pos_index]
    }
}

#[test]
fn check_token_type() {
    assert!(Token(TokenType::EOF, Range::default()).check_type(TokenType::EOF))
}

#[test]
fn get_source_code() {
    assert_eq!(
        Token(TokenType::EOF, Range::new(0, 2)).get_source_code("raw"),
        "ra"
    )
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum TokenType {
    EOF = 1,
    /** ;*/
    Semi,
    /** . */
    Dot,
    /** , */
    Comma,
    /** : */
    Colon,
    /** ( */
    LeftParenthesis,
    /** ) */
    RightParenthesis,

    /** + */
    Plus,

    /** - */
    Minus,

    /** < */
    LessThan,

    /** @ */
    At,

    /** [ */
    LeftSquareBracket,

    /** \ */
    ReverseSolidus,

    /** ] */
    RightSquareBracket,

    /** { */
    LeftCurlyBracket,
    /**} */
    RightCurlyBracket,

    /** # */
    NumberSign,

    /** / */
    ForwardSlash,
    KeyWord,
    Str,
    Digital,
    IdentToken,
    Dimension,
    Comment,
    FunctionToken,
    PercentageToken,
    AtKeywordToken,
    HashToken,
    UrlToken,
    CDOToken,
    CDCToken,
    LengthToken,

    // ast node type
    Stylesheet,
    Rule,
    Token,
    Select,
    ChartSet,
    Import,
    Medium,
    Function,
    Expression,
    Term,
    MediumList,
    Page,
    Property,
    Declaration,
    Important,
    // '/' or ','
    Operator,
    RuleList,
    DeclarationList,
}

impl Default for TokenType {
    fn default() -> Self {
        TokenType::EOF
    }
}

impl From<TokenType> for AstNode<TokenType> {
    fn from(token_type: TokenType) -> Self {
        Self {
            node_type: AstNodeType(token_type),
            range: Range::default(),
            children: None,
        }
    }
}
