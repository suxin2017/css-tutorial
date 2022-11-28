#[cfg(test)]
mod test_lexer {

    use css_tutorial::{lexer::Lexer, token_type::TokenType};

    //ANCHOR:test_token
    macro_rules! test_token {
        ($x:expr,$y:expr) => {
            let mut lexer = Lexer::new($x);
            let token = lexer.eat_token();
            dbg!(token.get_source_code(&$x));
            dbg!(&token);
            assert!(token.check_type($y));
        };
    }
    //ANCHOR_END:test_token

    // ANCHOR:lexer_test_example

    #[test]
    fn test_simple_symbol() {
        test_token!(r#"("#, TokenType::LeftParenthesis);
    }
    #[test]
    fn test_comment() {
        test_token!(
            r#"/*
        sadfadf
        */"#,
            TokenType::Comment
        );
    }

    #[test]
    fn test_num_token() {
        test_token!(r#".1"#, TokenType::Digital);
    }
    // ANCHOR_END:lexer_test_example

    #[test]
    fn test_comment_token() {
        test_token!(r#"/** abc */"#, TokenType::Comment);
    }

    #[test]
    fn test_ident_token() {
        test_token!(r#"abc"#, TokenType::IdentToken);
    }

    #[test]
    fn test_ident1_token() {
        test_token!(
            r#" -webkit-linear-gradient(45deg, rgba(255, 255, 255, .15) 25%, transparent 25%, transparent 50%, rgba(255, 255, 255, .15) 50%, rgba(255, 255, 255, .15) 75%, transparent 75%, transparent)"#,
            TokenType::FunctionToken
        );
    }

    #[test]
    fn test_zero_token() {
        test_token!(r#"0"#, TokenType::Digital);
    }

    #[test]
    fn test_func_token() {
        test_token!(r#"-abc-sadf("#, TokenType::FunctionToken);
    }

    #[test]
    fn test_at_token() {
        test_token!(r#"@abc"#, TokenType::AtKeywordToken);
    }

    #[test]
    fn test_string_token() {
        test_token!(r#""abc""#, TokenType::Str);
    }

    #[test]
    fn test_url_token() {
        test_token!(r#" url(links.css)"#, TokenType::UrlToken);
    }

    #[test]
    fn test_complexe_url_token() {
        test_token!(
            r#" url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='90' height='45'%3E%3Cpath d='M10 10h60' stroke='%2300F' stroke-width='5'/%3E%3Cpath d='M10 20h60' stroke='%230F0' stroke-width='5'/%3E%3Cpath d='M10 30h60' stroke='red' stroke-width='5'/%3E%3C/svg%3E")"#,
            TokenType::UrlToken
        );
    }

    #[test]
    fn test_important_token() {
        test_token!(r#" ! important"#, TokenType::Important);
    }

    #[test]
    fn test_plus_token() {
        test_token!(r#"+.checkbo"#, TokenType::Plus);
    }

    #[test]
    fn test_num1_token() {
        test_token!(r#".123"#, TokenType::Digital);
    }

    #[test]
    fn test_length_token() {
        test_token!(r#"123px"#, TokenType::Dimension);
    }

    #[test]
    fn test_body_token() {
        test_token!(r#"body"#, TokenType::IdentToken);
    }

    #[test]
    fn test_ident2_token() {
        test_token!(r#"body-color"#, TokenType::IdentToken);
    }

    #[test]
    fn test_string2_token() {
        test_token!(r#""\002a""#, TokenType::Str);
    }

    #[test]
    fn test_all_match_token() {
        test_token!(r#"*="#, TokenType::AllMatch);
    }

    #[test]
    fn test_url1_token() {
        test_token!(
            r#"url(../fonts\0123/glyphicons-halflings-regular.eot?#iefix)"#,
            TokenType::UrlToken
        );
    }

    #[test]
    fn test_simple_lexer() {
        let source = r#".embed-responsive-16by9 {
            padding-bottom: 56.25%
    }
        "#;
        let mut lexer = Lexer::new(source);
        loop {
            let token = lexer.eat_token();
            if token.check_type(TokenType::EOF) {
                break;
            }
        }
    }
}
