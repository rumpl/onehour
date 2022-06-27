use crate::{
    command::EngineError,
    oh::lexer::{Lexer, Token},
    parser::Program,
};
use std::str;

#[derive(Debug, Default)]
pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&mut self, input: &str) -> Result<Program, EngineError> {
        let mut lexer = Lexer::new(input);

        loop {
            let token = lexer.next_token();
            match token {
                Token::ASSIGN => println!("{:?}", token),

                Token::PLUS => println!("{:?}", token),
                Token::MINUS => println!("{:?}", token),
                Token::ASTERISK => println!("{:?}", token),
                Token::SLASH => println!("{:?}", token),

                Token::LT => println!("{:?}", token),
                Token::GT => println!("{:?}", token),

                Token::EOF => {
                    break;
                }
                Token::LPAREN => println!("{:?}", token),
                Token::RPAREN => println!("{:?}", token),
                Token::LBRACE => println!("{:?}", token),
                Token::RBRACE => println!("{:?}", token),

                Token::SEMICOLON => println!("{:?}", token),
                Token::COMMA => println!("{:?}", token),
                Token::INT(n) => println!("{:?} {:?}", token, n),
                Token::IDENT(ident) => println!("ident: {}", str::from_utf8(ident).unwrap()),
            }
        }

        Ok(Program {
            ..Default::default()
        })
    }
}
