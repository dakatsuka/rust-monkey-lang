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
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Equal,
    NotEqual,

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
    True,
    False,
    If,
    Else,
    Return,
}