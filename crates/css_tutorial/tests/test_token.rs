#[cfg(test)]
mod test_token {

    use css_tutorial::{range::Range, token::Token, token_type::TokenType};

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
}
