use lexer::Lexer;
use token;
use token::{Token, TokenType};

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
    current_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser {
        let mut parser = Parser {
            l: lexer,
            error: "",
            current_token: Token::default(),
            peek_token: Token::default(),
        };
        return parser;
    }

    // pub fn next_token(&mut self) {
    //     self.
    // }
}
