use std::string;

use crate::range::Range;

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
    IdentToken(String, Range),
}

impl Token {
    pub fn str(&self) -> &str {
        match self {
            Token::At(_) => return "@",
            Token::Digital(string, _) => return &string,
            Token::Str(string,_) => return &string,
            _ => "",
        }
    }
}
