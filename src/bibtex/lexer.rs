use std::iter::Peekable;
use std::str::Chars;

use super::error::ParseError;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    At,            // @
    OpenBrace,     // { 
    CloseBrace,    // }
    OpenParen,     // (
    CloseParen,    // )
    Comma,         // ,
    Equals,        // =
    Hash,          // #
    Quote,         // "
    Ident(String), // an identifier
    Comment,       // a comment
    Eof,           // EOF
}

impl TokenKind {
    pub fn is_eof(&self) -> bool {
        matches!(&self, Self::Eof)
    }
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind, 
    pub line: usize,
    pub col:  usize, 
}

#[derive(Debug)]
pub struct Tokens {
    stream: Vec<Token>, 
}

impl Tokens {
    pub fn new() -> Self {
        Tokens { stream: Vec::new() }
    }

    pub fn push(&mut self, token: Token) {
        self.stream.push(token);
    }
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>, 
    line: usize, 
    col: usize, 
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input: input.chars().peekable(), line: 1, col: 1 }
    }

    pub fn tokenize(&mut self) -> Result<Tokens, ParseError> {
        let mut tokens = Tokens::new();
        while let Some(token) = self.next_token()? {
            if token.is_eof() {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        Ok(tokens)
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(c) = self.input.next() {
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            Some(c)
        } else {
            None
        }
    }

    fn next_token() -> Result<Option<Token>, ParseError> {

    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
}