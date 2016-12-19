use lexer::Lexer;

#[derive(Debug, PartialEq)]
pub enum ParseType {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}


pub struct Parser<'a> {
    l: Lexer<'a>,
    error: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser {
        let mut parser = Parser {
            l: lexer,
            error: "",
        };
        return parser;
    }
}
