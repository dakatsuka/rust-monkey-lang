use crate::token::Token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            },
            b';' => Token::Semicolon,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b',' => Token::Comma,
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            b'*' => Token::Asterisk,
            b'/' => Token::Slash,
            b'<' => Token::Lt,
            b'>' => Token::Gt,
            b'{' => Token::Lbrace,
            b'}' => Token::Rbrace,
            b'a'...b'z' | b'A'...b'Z' | b'_' => return self.read_identifier(),
            b'0'...b'9'                      => return self.read_number(),
            0    => Token::Eof,
            _    => Token::Illegal,
        };

        self.read_char();
        token
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.ch {
                b' ' | b'\t' | b'\n' | b'\r' => self.read_char(),
                _                            => break,
            }
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self.input.as_bytes()[self.read_position];
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start_position = self.position;

        loop {
            match self.ch {
                b'a'...b'z' | b'A'...b'Z' | b'_' => self.read_char(),
                _                                => break,
            }
        }

        let literal = &self.input[start_position..self.position];

        match literal {
            "fn"     => Token::Function,
            "let"    => Token::Let,
            "if"     => Token::If,
            "else"   => Token::Else,
            "true"   => Token::True,
            "false"  => Token::False,
            "return" => Token::Return,
            _        => Token::Ident(String::from(literal)),
        }
    }

    fn read_number(&mut self) -> Token {
        let start_position = self.position;

        loop {
            match self.ch {
                b'0'...b'9' => self.read_char(),
                _           => break,
            }
        }

        let literal = &self.input[start_position..self.position];
        Token::Number(literal.parse::<i64>().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Token;
    use crate::lexer::Lexer;

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
"#;

        let tests = vec![
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Number(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Number(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::Lparen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::Rparen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Number(5),
            Token::Semicolon,
            Token::Number(5),
            Token::Lt,
            Token::Number(10),
            Token::Gt,
            Token::Number(5),
            Token::Semicolon,
            Token::If,
            Token::Lparen,
            Token::Number(5),
            Token::Lt,
            Token::Number(10),
            Token::Rparen,
            Token::Lbrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::Rbrace,
            Token::Else,
            Token::Lbrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::Rbrace,
            Token::Number(10),
            Token::Equal,
            Token::Number(10),
            Token::Semicolon,
            Token::Number(10),
            Token::NotEqual,
            Token::Number(9),
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input.to_string());

        for expect in tests {
            let token = lexer.next_token();
            assert_eq!(expect, token);
        }
    }
}