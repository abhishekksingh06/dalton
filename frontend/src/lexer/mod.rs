use miette::SourceSpan;

use crate::lexer::{
    cursor::Cursor,
    error::LexerError,
    keywords::lookup_keyword,
    token::{Token, TokenKind},
};

mod cursor;
pub mod error;
mod keywords;
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

    fn lex_string_literal(&mut self, start: usize) -> Result<TokenKind, LexerError> {
        let mut value = String::new();

        loop {
            let c = match self.cursor.consume() {
                Some(ch) => ch,
                None => {
                    return Err(LexerError::UnterminatedString {
                        span: (start..self.cursor.pos()).into(),
                    });
                }
            };

            match c {
                '"' => {
                    return Ok(TokenKind::StringLiteral(value));
                }

                '\n' => {
                    return Err(LexerError::NewlineInString {
                        span: (self.cursor.pos() - 1..self.cursor.pos()).into(),
                    });
                }

                '\\' => {
                    self.handle_escape(start, &mut value)?;
                }

                other => value.push(other),
            }
        }
    }

    fn handle_escape(&mut self, escape_start: usize, out: &mut String) -> Result<(), LexerError> {
        let next = match self.cursor.consume() {
            Some(ch) => ch,
            None => {
                return Err(LexerError::IncompleteEscapeSequence {
                    span: (escape_start..self.cursor.pos()).into(),
                });
            }
        };

        match next {
            'n' => out.push('\n'),
            't' => out.push('\t'),
            'r' => out.push('\r'),
            '"' => out.push('"'),
            '\\' => out.push('\\'),

            // Unicode escape (simple)
            'u' => {
                self.handle_unicode_escape(escape_start, out)?;
            }

            other => {
                return Err(LexerError::InvalidEscapeSequence {
                    escape: other,
                    span: (escape_start..self.cursor.pos()).into(),
                });
            }
        }

        Ok(())
    }

    fn handle_unicode_escape(
        &mut self,
        escape_start: usize,
        out: &mut String,
    ) -> Result<(), LexerError> {
        match self.cursor.consume() {
            Some('{') => {}
            _ => {
                return Err(LexerError::InvalidUnicodeEscape {
                    span: (escape_start..self.cursor.pos()).into(),
                });
            }
        }

        let mut hex = String::new();

        loop {
            let c = match self.cursor.consume() {
                Some(ch) => ch,
                None => {
                    return Err(LexerError::IncompleteEscapeSequence {
                        span: (escape_start..self.cursor.pos()).into(),
                    });
                }
            };

            match c {
                '}' => break,
                x if x.is_ascii_hexdigit() => hex.push(x),
                _ => {
                    return Err(LexerError::InvalidUnicodeEscape {
                        span: (escape_start..self.cursor.pos()).into(),
                    });
                }
            }
        }

        if hex.is_empty() {
            return Err(LexerError::InvalidUnicodeEscape {
                span: (escape_start..self.cursor.pos()).into(),
            });
        }

        let value = u32::from_str_radix(&hex, 16).unwrap();

        if value > 0x10FFFF {
            return Err(LexerError::UnicodeEscapeOutOfRange {
                value,
                span: (escape_start..self.cursor.pos()).into(),
            });
        }

        match char::from_u32(value) {
            Some(ch) => {
                out.push(ch);
                Ok(())
            }
            None => Err(LexerError::InvalidUnicodeEscape {
                span: (escape_start..self.cursor.pos()).into(),
            }),
        }
    }

    fn collect_digits(
        &mut self,
        out: &mut String,
        valid: impl Fn(char) -> bool,
    ) -> Result<(), LexerError> {
        let mut last_was_underscore = false;
        let mut seen_digit = false;

        while let Some(c) = self.cursor.peek() {
            if c == '_' {
                if !seen_digit || last_was_underscore {
                    return Err(LexerError::InvalidNumber {
                        span: self.get_source_span(self.cursor.pos()),
                    });
                }
                last_was_underscore = true;
                out.push(self.cursor.consume().unwrap());
            } else if valid(c) {
                seen_digit = true;
                last_was_underscore = false;
                out.push(self.cursor.consume().unwrap());
            } else {
                break;
            }
        }

        if last_was_underscore {
            return Err(LexerError::InvalidNumber {
                span: self.get_source_span(self.cursor.pos()),
            });
        }

        Ok(())
    }

    fn lex_number(&mut self, first: char) -> Result<TokenKind, LexerError> {
        let mut s = String::new();
        s.push(first);
        let base = if first == '0' {
            match self.cursor.peek() {
                Some('b') | Some('B') => {
                    s.push(self.cursor.consume().unwrap());
                    self.collect_digits(&mut s, |c| c == '0' || c == '1')?;
                    2
                }
                Some('o') | Some('O') => {
                    s.push(self.cursor.consume().unwrap());
                    self.collect_digits(&mut s, |c| ('0'..='7').contains(&c))?;
                    8
                }
                Some('x') | Some('X') => {
                    s.push(self.cursor.consume().unwrap());
                    self.collect_digits(&mut s, |c| c.is_ascii_hexdigit())?;
                    16
                }
                _ => 10,
            }
        } else {
            10
        };

        if base != 10 {
            return Ok(TokenKind::IntegerLiteral(s));
        }

        self.collect_digits(&mut s, |c| c.is_ascii_digit())?;

        let mut is_float = false;
        if self.cursor.peek() == Some('.') {
            self.cursor.consume();
            if let Some(c) = self.cursor.peek() {
                if c.is_ascii_digit() {
                    is_float = true;
                    s.push('.');
                    self.collect_digits(&mut s, |d| d.is_ascii_digit())?;
                } else {
                    return Ok(TokenKind::IntegerLiteral(s));
                }
            }
        }

        if let Some(_e @ ('e' | 'E')) = self.cursor.peek() {
            is_float = true;
            s.push(self.cursor.consume().unwrap());

            if let Some(_sign @ ('+' | '-')) = self.cursor.peek() {
                s.push(self.cursor.consume().unwrap());
            }

            self.collect_digits(&mut s, |c| c.is_ascii_digit())?;
        }

        if is_float {
            Ok(TokenKind::FloatLiteral(s))
        } else {
            Ok(TokenKind::IntegerLiteral(s))
        }
    }

    #[inline]
    fn make_token(&self, start: usize, kind: TokenKind) -> Token {
        Token {
            kind,
            span: self.get_source_span(start),
        }
    }

    #[inline]
    fn consume_if(&mut self, start: usize, expected: char) -> Result<bool, LexerError> {
        match self.cursor.peek() {
            Some(c) => {
                if c != expected {
                    return Ok(false);
                }
                self.cursor.consume();
                Ok(true)
            }
            None => Err(LexerError::UnexpectedEof {
                span: self.get_source_span(start),
            }),
        }
    }

    fn lex_keyword(&mut self, first: char) -> TokenKind {
        let mut ident = String::from(first);
        while let Some(c) = self.cursor.peek() {
            if !(c.is_ascii_alphanumeric() || c == '_') {
                break;
            }
            ident.push(c);
            self.cursor.consume();
        }
        lookup_keyword(ident)
    }

    #[inline]
    fn get_token_kind(&mut self, start: usize, ch: char) -> Result<TokenKind, LexerError> {
        let kind = match ch {
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            '%' => TokenKind::Percent,
            '^' => TokenKind::Caret,
            '.' => TokenKind::Dot,
            ';' => TokenKind::Semicolon,
            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,

            '&' if self.consume_if(start, '&')? => TokenKind::AndAnd,
            '|' if self.consume_if(start, '|')? => TokenKind::OrOr,
            '-' if self.consume_if(start, '>')? => TokenKind::Arrow,

            '+' if self.consume_if(start, '=')? => TokenKind::PlusEqual,
            '+' => TokenKind::Plus,

            '-' if self.consume_if(start, '=')? => TokenKind::MinusEqual,
            '-' => TokenKind::Minus,

            '*' if self.consume_if(start, '=')? => TokenKind::StarEqual,
            '*' => TokenKind::Star,

            '/' if self.consume_if(start, '=')? => TokenKind::SlashEqual,
            '/' => TokenKind::Slash,

            '!' if self.consume_if(start, '=')? => TokenKind::NotEqual,
            '!' => TokenKind::Bang,

            '=' if self.consume_if(start, '=')? => TokenKind::EqualEqual,
            '=' => TokenKind::Equal,

            '<' if self.consume_if(start, '=')? => TokenKind::LessEqual,
            '<' => TokenKind::Less,

            '>' if self.consume_if(start, '=')? => TokenKind::GreaterEqual,
            '>' => TokenKind::Greater,

            'a'..='z' | 'A'..='Z' | '_' => self.lex_keyword(ch),
            '"' => self.lex_string_literal(start)?,
            '0'..='9' => self.lex_number(ch)?,

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
