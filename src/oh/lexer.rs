use std::str;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    ASSIGN,

    PLUS,
    MINUS,
    ASTERISK,
    SLASH,

    LT,
    GT,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    SEMICOLON,
    COMMA,

    INT(&'a [u8]),
    IDENT(&'a [u8]),

    EOF,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a [u8],
    pos: usize,
    next_pos: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input: input.as_bytes(),
            pos: 0,
            next_pos: 0,
            ch: 0,
        };

        lexer.read_char();

        lexer
    }

    pub fn next_token(&mut self) -> Token<'a> {
        self.skip_whitespace();
        let token = match self.ch {
            b'=' => Token::ASSIGN,

            b'+' => Token::PLUS,
            b'-' => Token::MINUS,
            b'/' => Token::SLASH,
            b'*' => Token::ASTERISK,

            b'<' => Token::LT,
            b'>' => Token::GT,

            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,

            b';' => Token::SEMICOLON,
            b',' => Token::COMMA,

            _ => {
                if self.is_digit() {
                    return Token::INT(self.read_number());
                }
                if self.is_letter() {
                    return Token::IDENT(self.read_identifier());
                }
                return Token::EOF;
            }
        };

        self.read_char();

        token
    }

    fn read_number(&mut self) -> &'a [u8] {
        let start = self.pos;

        while self.is_digit() {
            self.read_char();
        }

        &self.input[start..self.pos]
    }

    fn read_identifier(&mut self) -> &'a [u8] {
        let start = self.pos;

        while self.is_letter() {
            self.read_char();
        }

        &self.input[start..self.pos]
    }

    fn is_letter(&self) -> bool {
        self.ch.is_ascii_alphabetic()
    }

    fn is_digit(&self) -> bool {
        self.ch.is_ascii_digit()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_char(&mut self) {
        if self.next_pos >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.next_pos];
        }
        self.pos = self.next_pos;
        self.next_pos += 1;
    }
}

impl<'a> std::iter::Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        let token = self.next_token();
        if token == Token::EOF {
            return None;
        }
        Some(token)
    }
}

#[test]
fn test_tokens() {
    let mut lexer = Lexer::new("+-*/=(){}<>;,");

    let expected = [
        Token::PLUS,
        Token::MINUS,
        Token::ASTERISK,
        Token::SLASH,
        Token::ASSIGN,
        Token::LPAREN,
        Token::RPAREN,
        Token::LBRACE,
        Token::RBRACE,
        Token::LT,
        Token::GT,
        Token::SEMICOLON,
        Token::COMMA,
        Token::EOF,
    ];

    for e in expected {
        let token = lexer.next_token();
        assert_eq!(token, e);
    }
}

#[test]
fn test_ident() {
    let mut lexer = Lexer::new("test");
    let token = lexer.next_token();

    assert_eq!(token, Token::IDENT("test".as_bytes()));
}

#[test]
fn test_number() {
    let mut lexer = Lexer::new("1234");
    let token = lexer.next_token();

    assert_eq!(token, Token::INT("1234".as_bytes()));
}

#[test]
fn test_assign() {
    let mut lexer = Lexer::new("n = 1234");

    let expected = [
        Token::IDENT("n".as_bytes()),
        Token::ASSIGN,
        Token::INT("1234".as_bytes()),
        Token::EOF,
    ];

    for e in expected {
        let token = lexer.next_token();
        assert_eq!(token, e);
    }
}
