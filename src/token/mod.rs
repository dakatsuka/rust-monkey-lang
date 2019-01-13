#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    // Identifiers + Literal
    Ident(String),
    Number(i64),

    // Statements
    Assign,

    // Operators
    Plus,

    // Delimiters
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // Keywords
    Function,
    Let,
}