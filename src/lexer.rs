use token;
use token::{Token,TokenType};

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        let mut l = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        l.read_char();
        return l;
    }

    pub fn next_token(&mut self) -> Token {
        let mut tok = Token::default();

        self.skip_whitespace();

        match self.ch {
            Some(ch @ '=') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    tok = Token {
                        token_type: TokenType::Equal,
                        literal: String::from("=="),
                    };
                } else {
                    tok = new_token(TokenType::Assign, ch);
                }
            }
            Some(ch @ '!') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    tok = Token {
                        token_type: TokenType::NotEqual,
                        literal: String::from("!="),
                    };
                } else {
                    tok = new_token(TokenType::Bang, ch);
                }
            }
            Some(ch @ '+') => tok = new_token(TokenType::Plus, ch),
            Some(ch @ '-') => tok = new_token(TokenType::Minus, ch),
            Some(ch @ '/') => tok = new_token(TokenType::Slash, ch),
            Some(ch @ '*') => tok = new_token(TokenType::Asterisk, ch),
            Some(ch @ '<') => tok = new_token(TokenType::LowerThan, ch),
            Some(ch @ '>') => tok = new_token(TokenType::GreaterThan, ch),
            Some(ch @ ';') => tok = new_token(TokenType::Semicolon, ch),
            Some(ch @ ',') => tok = new_token(TokenType::Comma, ch),
            Some(ch @ '{') => tok = new_token(TokenType::LeftBrace, ch),
            Some(ch @ '}') => tok = new_token(TokenType::RightBrace, ch),
            Some(ch @ '(') => tok = new_token(TokenType::LeftParenthesis, ch),
            Some(ch @ ')') => tok = new_token(TokenType::RightParenthesis, ch),

            Some(ch @ _) => {
                if is_letter(ch) {
                    tok.literal = self.read_identifier();
                    tok.token_type = token::lookup_ident(&tok.literal);
                    return tok;
                } else if ch.is_numeric() {
                    tok.token_type = TokenType::Integer;
                    tok.literal = self.read_number();
                    return tok;
                } else {
                    tok = new_token(TokenType::Illegal, ch);
                }
            }

            None => {
                tok.literal = String::new();
                tok.token_type = TokenType::EndOfFile;
            }
        }

        self.read_char();
        return tok;
    }

    fn skip_whitespace(&mut self) {
        while match self.ch {
            Some(ch) => ch.is_whitespace(),
            _ => false,
        } {
            self.read_char();
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input[self.read_position..].chars().next();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char_eq(&mut self, ch: char) -> bool {
        if self.read_position >= self.input.len() {
            return false;
        }

        let peek_ch = self.input
            .chars()
            .nth(self.read_position)
            .unwrap();

        peek_ch == ch
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;

        while is_letter(self.ch.expect("Error reading character")) {
            self.read_char();
        }

        self.input[position..self.position].to_owned()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;

        while self.ch.expect("Error reading character").is_numeric() {
            self.read_char();
        }
        self.input[position..self.position].to_owned()
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

fn new_token(token_type: TokenType, ch: char) -> Token {
    Token {
        token_type: token_type,
        literal: ch.to_string(),
    }
}