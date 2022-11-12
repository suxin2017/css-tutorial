use crate::lexer::Lexer;


#[derive(Debug)]
struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,

}

impl<'a> Parser<'a> {
    fn new(lexer:&'a mut Lexer<'a>) ->Self{
        Self{
            lexer
        }
    }

    fn stylessheet(&mut self){
        todo!()
    }

    fn function_block(&mut self){todo!()}
    fn square_block(&mut self){todo!()}
    fn curly_block(&mut self){todo!()}
    fn parenthesis_block(&mut self){todo!()}
    fn component_value(&mut self){
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn new()  {
            let mut lexer = Lexer::new(
                r#""abc
        ""#,
            );
        let mut parser = Parser::new(&mut lexer);
        dbg!(parser);


    }
}
