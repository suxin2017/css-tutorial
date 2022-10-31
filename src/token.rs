use crate::lexer::Range;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    EOF,
    /** ;*/
    Semi(Range),
    /** . */
    Dot(Range),
    /** , */
    Comma(Range),
    /** : */
    Colon(Range),
    /** ( */
    LeftParen(Range),
    /** ) */
    RightParen(Range),

    /** + */
    Plus(Range),

    /** - */
    Minus(Range),

    /** < */
    LessThan(Range),

    /** @ */
    At(Range),

    /** [ */
    LeftSquareBracket(Range),

    /** \ */
    ReverseSolidus(Range),

    /** ] */
    RightSquareBracket(Range),

    /** { */
    LeftCurlyBracket(Range),
    /**} */
    RightCurlyBracket(Range),

    /** # */
    NumberSign(Range),
    KeyWord(String, Range),
    Str(String, Range),
    Digital(String, Range),
}

impl Token {
    pub fn str(&self) -> String {
        match self {
            Token::At(_) => return '@'.to_string(),
            Token::Digital(string, _) => return string.clone(),
            _ => "".to_string(),
        }
    }
}
