#[cfg(test)]
mod tests {

    use css_tutorial::{ast::AstTreeBuilder, lexer::Lexer, parser::Parser};

    #[test]
    fn charset_test() {
        let mut lexer = Lexer::new(r#"@charset "utf8";"#);
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_charset();
        builder.finish();
        dbg!(builder.ast_tree);
    }

    #[test]
    fn import_url_test() {
        let mut lexer = Lexer::new(
            r#"@import "custom.css";
            @import url("bluish.css");"#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);

        parser.parse_import_token();
        builder.finish();
        dbg!(builder.ast_tree);
    }

    #[test]
    fn import_string_test() {
        let mut lexer = Lexer::new(r#"@import "custom.css";"#);
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_import_token();
        builder.finish();
        dbg!(builder.ast_tree);
    }

    #[test]
    fn comment_test() {
        let mut lexer = Lexer::new(
            r#"/* adfsdf
        
        
        
        
        */"#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse();
        dbg!(builder.ast_tree);
    }

    #[test]
    fn function_test() {
        let mut lexer = Lexer::new(r#"a(10px + 100px)"#);
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_function();
        builder.finish();
        dbg!(builder.ast_tree);
    }

    #[test]
    fn declaration_list_test() {
        let mut lexer = Lexer::new(
            r#"{
            a: 123
        }"#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_declaration_list();
        builder.finish();
        dbg!(builder.ast_tree);
    }

    #[test]
    fn declaration_list1_test() {
        let mut lexer = Lexer::new(
            r#"{
            -a: 123
        }"#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_declaration_list();
        builder.finish();
        dbg!(builder.ast_tree);
    }

    #[test]
    fn declaration_list2_test() {
        let mut lexer = Lexer::new(
            r#"{
            a: 0
        }"#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_declaration_list();
        builder.finish();
        dbg!(builder.ast_tree);
    }

    #[test]
    fn at_rule_test() {
        let mut lexer = Lexer::new(
            r#"@font 123123ks sdafjk asdfjksadf sadfjk {

            }"#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_at_rule();
        builder.finish();
        let serialized = serde_json::to_string(&builder.ast_tree).unwrap();
        println!("serialized = {}", serialized);
        dbg!(builder.ast_tree);
    }

    #[test]
    fn at_select_test() {
        let mut lexer = Lexer::new(r#"div.class "#);
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_selector();
        builder.finish();
        let serialized = serde_json::to_string(&builder.ast_tree).unwrap();
        println!("serialized = {}", serialized);
        dbg!(builder.ast_tree);
    }

    #[test]
    fn select3_test() {
        let mut lexer = Lexer::new(
            r#"  button::-moz-focus-inner,
        input::-moz-focus-inner {
            padding: 0;
            border: 0
        }"#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_selector();
        builder.finish();
        let serialized = serde_json::to_string(&builder.ast_tree).unwrap();
        println!("serialized = {}", serialized);
        dbg!(builder.ast_tree);
    }

    #[test]
    fn simple_test() {
        let mut lexer = Lexer::new(
            r#"
            article,
            aside,
            details,
            figcaption,
            figure,
            footer,
            header,
            hgroup,
            main,
            menu,
            nav,
            section,
            summary"#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_selector();
        let serialized = serde_json::to_string(&builder.ast_tree).unwrap();
        println!("serialized = {}", serialized);
        dbg!(builder.ast_tree);
    }

    #[test]
    fn simple1_test() {
        let mut lexer = Lexer::new(
            r#"{
                -webkit-text-size-adjust: 100%;
                -ms-text-size-adjust: 100%
            }"#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_declaration_list();
        dbg!(builder.ast_tree);
    }

    #[test]
    fn simple2_test() {
        let mut lexer = Lexer::new(
            r#"
            audio:not([controls])"#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_selector();
        let serialized = serde_json::to_string(&builder.ast_tree).unwrap();
        println!("serialized = {}", serialized);
        dbg!(builder.ast_tree);
    }

    #[test]
    fn simple3_test() {
        let mut lexer = Lexer::new(
            r#"
            {
                margin: .67em 0;
                font-size: 2em
            }"#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_declaration_list();
        let serialized = serde_json::to_string(&builder.ast_tree).unwrap();
        println!("serialized = {}", serialized);
        dbg!(builder.ast_tree);
    }
    #[test]
    fn simple4_test() {
        let mut lexer = Lexer::new(
            r#"
    @media print {
        *,
        :after,
        :before {
            box-shadow: none !important
        }
    
        a,
        a:visited {
            text-decoration: underline
        }
    }"#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_at_rule();
        let serialized = serde_json::to_string(&builder.ast_tree).unwrap();
        println!("serialized = {}", serialized);
        dbg!(builder.ast_tree);
    }

    #[test]
    fn simple5_test() {
        let mut lexer = Lexer::new(
            r#"
            @font-face {
                src: url(../fonts/glyphicons-halflings-regular.eot);
                src: url(../fonts/glyphicons-halflings-regular.eot?#iefix) format('embedded-opentype'), url(../fonts/glyphicons-halflings-regular.woff2) format('woff2'), url(../fonts/glyphicons-halflings-regular.woff) format('woff'), url(../fonts/glyphicons-halflings-regular.ttf) format('truetype'), url(../fonts/glyphicons-halflings-regular.svg#glyphicons_halflingsregular) format('svg')
            }"#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_at_rule();
        let serialized = serde_json::to_string(&builder.ast_tree).unwrap();
        println!("serialized = {}", serialized);
        dbg!(builder.ast_tree);
    }

    #[test]
    fn simple6_test() {
        let mut lexer = Lexer::new(
            r#"
            html {
                font-size: 10px;
                -webkit-tap-highlight-color: rgba(0, 1, 2, 3)
            } 
          
            "#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_rule();
        let serialized = serde_json::to_string(&builder.ast_tree).unwrap();
        println!("serialized = {}", serialized);
        dbg!(builder.ast_tree);
    }

    //ANCHOR:exapmle
    #[test]
    fn simple7_test() {
        let mut lexer = Lexer::new(
            r#"
            table col[class*=col-] {
                position: static;
                display: table-column;
                float: none
            }
          
            "#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse();
        let serialized = serde_json::to_string(&builder.ast_tree).unwrap();
        println!("serialized = {}", serialized);
        dbg!(builder.ast_tree);
    }
    //ANCHOR_END:exapmle
    #[test]
    fn simple8_test() {
        let mut lexer = Lexer::new(
            r#"
         
                filter: progid:DXImageTransform.Microsoft.gradient(enabled=false)
            "#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_declaration();
        let serialized = serde_json::to_string(&builder.ast_tree).unwrap();
        println!("serialized = {}", serialized);
        dbg!(builder.ast_tree);
    }

    #[test]
    fn simple9_test() {
        let mut lexer = Lexer::new(
            r#"
            -webkit-linear-gradient(45deg, rgba(255, 255, 255, .15) 25%, transparent 25%, transparent 50%, rgba(255, 255, 255, .15) 50%, rgba(255, 255, 255, .15) 75%, transparent 75%, transparent);
            "#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_function();
        let serialized = serde_json::to_string(&builder.ast_tree).unwrap();
        println!("serialized = {}", serialized);
        dbg!(builder.ast_tree);
    }

    #[test]
    fn simple10_test() {
        let mut lexer = Lexer::new(
            r#"
            .glyphicon-chevron-up:before {
                content: "\e113"
            }
            "#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse();
        let serialized = serde_json::to_string(&builder.ast_tree).unwrap();
        println!("serialized = {}", serialized);
        dbg!(builder.ast_tree);
    }

    #[test]
    fn simple11_test() {
        let mut lexer = Lexer::new(
            r#"
            @supports (-webkit-overflow-scrolling:touch) {
                has-parallax {
                    background-attachment: scroll
                }
            }
            "#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse();
        let serialized = serde_json::to_string(&builder.ast_tree).unwrap();
        println!("serialized = {}", serialized);
        dbg!(builder.ast_tree);
    }

    #[test]
    fn simple12_test() {
        let mut lexer = Lexer::new(
            r#"
          
                 calc((100% - 16px)/ 2)
            "#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_function();
        let serialized = serde_json::to_string(&builder.ast_tree).unwrap();
        println!("serialized = {}", serialized);
        dbg!(builder.ast_tree);
    }

    #[test]
    fn simple13_test() {
        let mut lexer = Lexer::new(
            r#"
            .wp-block-gallery.columns-1 .blocks-gallery-image:nth-of-type(1n),.wp-block-gallery.columns-1 .blocks-gallery-item:nth-of-type(1n) {
                margin-right: 0
            }
            "#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse_rule();
        let serialized = serde_json::to_string(&builder.ast_tree).unwrap();
        println!("serialized = {}", serialized);
        dbg!(builder.ast_tree);
    }
    #[test]
    fn simple14_test() {
        let mut lexer = Lexer::new(
            r#"
         
    p:not(.irrelevant) {
        font-weight: bold;
    }
    
    p > strong,
    p > b.important {
        color: crimson;
    }
    
    p > :not(strong, b.important) {
        color: darkmagenta;
    }
            "#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse();
        let serialized = serde_json::to_string(&builder.ast_tree).unwrap();
        dbg!(builder.ast_tree);
    }

    #[test]
    fn simple15_test() {
        let mut lexer = Lexer::new(
            r#"
            p {
                font-weight: bold;
            }
            
            li:nth-child(-n+3) {
                border: 2px solid orange;
                margin-bottom: 1px;
            }
            
            li:nth-child(even) {
                background-color: lightyellow;
            }
            "#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse();
        dbg!(builder.ast_tree);
    }

    #[test]
    fn simple16_test() {
        let mut lexer = Lexer::new(
            r#"
            @media(prefers-reduced-motion:reduce) {
                html {
                    scroll-behavior: auto
                }
            
                *,:after,:before {
                    -webkit-animation-duration: .01ms!important;
                    animation-duration: .01ms!important;
                    -webkit-animation-iteration-count: 1!important;
                    animation-iteration-count: 1!important;
                    scroll-behavior: auto!important;
                    transition-duration: .01ms!important
                }
            }
            "#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse();
        dbg!(builder.ast_tree);
    }

    #[test]
    fn simple17_test() {
        let mut lexer = Lexer::new(
            r#"
            @-webkit-keyframes blink {
                0%,to {
                    text-decoration-line: none
                }
            
                50% {
                    text-decoration-line: underline
                }
            }
            "#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse();
        dbg!(builder.ast_tree);
    }

    #[test]
    fn simple18_test() {
        let mut lexer = Lexer::new(
            r#"
            /*! tailwindcss v3.2.4 | MIT License | https://tailwindcss.com*/
            *,
            :after,
            :before {
              border: 0 solid #e5e7eb;
              box-sizing: border-box;
            }
            "#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse();
        dbg!(builder.ast_tree);
    }

    #[test]
    fn simple19_test() {
        let mut lexer = Lexer::new(
            r#"
          .flex-item {
                flex-basis: calc(8% -(var(--su0) * 0.1)
            );
              }
            "#,
        );
        let mut builder = AstTreeBuilder::new();
        let mut parser = Parser::new(&mut lexer, &mut builder);
        parser.parse();
        dbg!(builder.ast_tree);
    }
}
