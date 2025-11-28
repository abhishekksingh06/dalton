use crate::span::Spanned;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // End,
    Eof,

    // Identifiers & Literals (with values)
    Identifier(String),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    CharLiteral(char),
    True,
    False,

    // Keywords
    Fn,
    Return,
    Let,
    Var,
    If,
    Else,
    For,
    While,
    Break,
    Continue,

    // Operators
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Percent, // %
    Caret,   // ^
    Bang,    // !
    AndAnd,  // &&
    OrOr,    // ||

    // Comparisons
    EqualEqual,   // ==
    NotEqual,     // !=
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=

    // Assignment
    Equal, // =

    // Compound assignment
    PlusEqual,  // +=
    MinusEqual, // -=
    StarEqual,  // *=
    SlashEqual, // /=

    // Delimiters
    LParen,   // (
    RParen,   // )
    LBrace,   // {
    RBrace,   // }
    LBracket, // [
    RBracket, // ]

    // Punctuation
    Comma,     // ,
    Dot,       // .
    Semicolon, // ;
    Colon,     // :
    Arrow,     // ->
}

use std::fmt;

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // End & Invalid
            TokenType::Eof => write!(f, "EOF"),

            // Identifiers & Literals
            TokenType::Identifier(s) => write!(f, "Identifier({})", s),
            TokenType::IntegerLiteral(i) => write!(f, "IntegerLiteral({})", i),
            TokenType::FloatLiteral(fl) => write!(f, "FloatLiteral({})", fl),
            TokenType::StringLiteral(s) => write!(f, "StringLiteral(\"{}\")", s),
            TokenType::CharLiteral(c) => write!(f, "CharLiteral('{}')", c),
            TokenType::True => write!(f, "true"),
            TokenType::False => write!(f, "false"),

            // Keywords
            TokenType::Fn => write!(f, "fn"),
            TokenType::Return => write!(f, "return"),
            TokenType::Let => write!(f, "let"),
            TokenType::Var => write!(f, "var"),
            TokenType::If => write!(f, "if"),
            TokenType::Else => write!(f, "else"),
            TokenType::For => write!(f, "for"),
            TokenType::While => write!(f, "while"),
            TokenType::Break => write!(f, "break"),
            TokenType::Continue => write!(f, "continue"),

            // Operators
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Star => write!(f, "*"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Percent => write!(f, "%"),
            TokenType::Caret => write!(f, "^"),
            TokenType::Bang => write!(f, "!"),
            TokenType::AndAnd => write!(f, "&&"),
            TokenType::OrOr => write!(f, "||"),

            // Comparisons
            TokenType::EqualEqual => write!(f, "=="),
            TokenType::NotEqual => write!(f, "!="),
            TokenType::Less => write!(f, "<"),
            TokenType::Greater => write!(f, ">"),
            TokenType::LessEqual => write!(f, "<="),
            TokenType::GreaterEqual => write!(f, ">="),

            // Assignment
            TokenType::Equal => write!(f, "="),

            // Compound assignment
            TokenType::PlusEqual => write!(f, "+="),
            TokenType::MinusEqual => write!(f, "-="),
            TokenType::StarEqual => write!(f, "*="),
            TokenType::SlashEqual => write!(f, "/="),

            // Delimiters
            TokenType::LParen => write!(f, "("),
            TokenType::RParen => write!(f, ")"),
            TokenType::LBrace => write!(f, "{{"),
            TokenType::RBrace => write!(f, "}}"),
            TokenType::LBracket => write!(f, "["),
            TokenType::RBracket => write!(f, "]"),

            // Punctuation
            TokenType::Comma => write!(f, ","),
            TokenType::Dot => write!(f, "."),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Colon => write!(f, ":"),
            TokenType::Arrow => write!(f, "->"),
        }
    }
}

pub type Token = Spanned<TokenType>;

