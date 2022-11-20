use crate::range::Range;

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    EOF,
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
    KeyWord,
    Str,
    Digital,
    IdentToken,
    Comment,
    FunctionToken,
    PercentageToken,
    AtKeywordToken,
    HashToken,
    UrlToken,
    CDOToken,
    CDCToken,
}

#[derive(Debug)]
pub enum SyntaxNode {
    CommentNode(CommentNode),
    Stylesheet(Stylesheet),
    Rule(Rule),
    Token(Token),
    Select(Select),
    ChartSet(ChartSet),
    Import(Import),
    Medium(Medium),
    Function(Function),
    Expression(Expression),
    Term(Term),
}

#[derive(Debug)]
pub struct Function {
    pub expr: Expression,
}

#[derive(Debug)]
pub struct Expression {
    pub terms: Term,
}

#[derive(Debug)]
pub struct Term {
    // 怎么不加Box也能用啊
    pub token: Box<SyntaxNode>,
}



#[derive(Debug)]
pub struct CommentNode {
    pub(crate) node: Token,
}

#[derive(Debug)]
pub struct Stylesheet {
    pub(crate) nodes: Vec<SyntaxNode>,
}
#[derive(Debug)]
pub struct Rule {
    pub(crate) rules: Vec<SyntaxNode>,
}
#[derive(Debug)]
pub struct Select {
    pub tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct ChartSet {
    pub token: Token,
}

#[derive(Debug)]
pub struct Import {
    pub token: Vec<SyntaxNode>,
}

#[derive(Debug)]
pub struct Medium {
    pub token: Token,
}
