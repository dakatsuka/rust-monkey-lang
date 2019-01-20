extern crate rustyline;
extern crate monkey;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use monkey::lexer::Lexer;
use monkey::token::Token::*;

fn main() {
    let mut rl = Editor::<()>::new();

    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());

                let mut lexer = Lexer::new(line);

                loop {
                    let token = lexer.next_token();
                    match token {
                        Eof => break,
                        _   => println!("{:?}", token)
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                break
            },
            Err(ReadlineError::Eof) => {
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }

    rl.save_history("history.txt").unwrap();
}