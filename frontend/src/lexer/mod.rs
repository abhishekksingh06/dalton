use miette::SourceSpan;

use crate::lexer::{
    cursor::Cursor,
    error::LexerError,
    token::{Token, TokenKind},
};

mod cursor;
pub mod error;
pub mod token;

pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            cursor: Cursor::new(src),
        }
    }

    #[inline]
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.cursor.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.cursor.consume();
        }
    }

    #[inline]
    fn get_source_span(&self, start: usize) -> SourceSpan {
        let end = self.cursor.pos();
        let len = end - start;
        SourceSpan::new(start.into(), len)
    }

    #[inline]
    fn make_token(&self, start: usize, kind: TokenKind) -> Token {
        Token {
            kind,
            span: self.get_source_span(start),
        }
    }

    #[inline]
    fn get_token_kind(&self, start: usize, ch: char) -> Result<TokenKind, LexerError> {
        let kind = match ch {
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            _ => {
                return Err(LexerError::UnexpectedChar {
                    found: ch,
                    span: self.get_source_span(start),
                });
            }
        };
        Ok(kind)
    }

    #[inline]
    fn next_token(&mut self) -> Result<Token, LexerError> {
        let start = self.cursor.pos();
        self.skip_whitespace();
        let ch = match self.cursor.consume() {
            Some(c) => c,
            None => return Ok(self.make_token(start, TokenKind::Eof)),
        };
        let kind = self.get_token_kind(start, ch)?;
        Ok(self.make_token(start, kind))
    }

    pub fn tokens(&mut self) -> (Vec<Token>, Vec<LexerError>) {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();
        loop {
            match self.next_token() {
                Ok(token) => {
                    if token.kind == TokenKind::Eof {
                        tokens.push(token);
                        break;
                    }
                    tokens.push(token);
                }
                Err(err) => errors.push(err),
            }
        }
        (tokens, errors)
    }
}
