use serde::{Deserialize, Serialize};

use crate::{
    ast::{AstNode, AstNodeType},
    range::Range,
};

// ANCHOR: lexer_token_type
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

    /** '-' */
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

    /** = */
    Equal,

    /** '*' */
    Asterisk,

    /* |= */
    Dashmatch,

    /** ~ */
    Wave,
    /* ~= */
    Includes,
    /* <!-- */
    CDOToken,

    /** --> */
    CDCToken,

    /** > */
    MoreThan,

    /** ^= */
    Exclude,

    /** *= */
    AllMatch,

    /** 复杂token */
    Str,
    Digital,
    IdentToken,
    Dimension,
    Comment,
    FunctionToken,
    AtKeywordToken,
    HashToken,
    UrlToken,
    PercentageToken,
    // ANCHOR_END: lexer_token_type

    // ast node type
    Stylesheet,
    Rule,
    Token,
    Selector,
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
    AtRule,
    AtRuleParams,
    ElementName,
    SimpleSelect,
    Class,
    Attrib,
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
