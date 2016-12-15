pub mod lexer;
pub mod token;

use std::io::{self, BufRead, Write};
use token::TokenType;
use lexer::Lexer;

fn main() {
    let stdin = io::stdin();

    loop {
        print!(">> ");
        io::stdout().flush().expect("Error flushing stdout");

        let mut line = String::new();
        stdin.lock().read_line(&mut line).expect("Error reading from stdin");
        let mut lexer = Lexer::new(&mut line);

        loop {
            let tok = lexer.next_token();
            print!("{:?}", tok);
            if tok.token_type == TokenType::EndOfFile {
                break;
            }
        }
    }
}