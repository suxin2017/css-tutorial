use crate::lexer::Range;

pub enum Token {
    EOF,
	/** ;*/
    SEMI(Range),
	/** . */
    DOT(Range),
	/** , */
    COMMA(Range),
	/** : */
    COLON(Range),
	/** ( */
    LPAREN(Range),
	/** ) */
    RPAREN(Range),

	KeyWord(String,Range),
	Str(String,Range),
	Digital(String,Range),
}
