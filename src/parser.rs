use lexer::Lexer;

pub struct Parser<'a> {
    l: Lexer<'a>,
    error: &'a str,
}
