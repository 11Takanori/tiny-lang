#[derive(Debug, PartialEq)]
pub enum TokenType {
    Illegal,
    EndOfFile,

    // Identifiers + literals
    Ident,
    Integer,

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LowerThan,
    GreaterThan,
    Equal,
    NotEqual,

    // Delimiters
    Comma,
    Semicolon,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Default for Token {
    fn default() -> Self {
        Token {
            token_type: TokenType::Illegal,
            literal: String::new(),
        }
    }
}

pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        _ => TokenType::Ident,
    }
}
