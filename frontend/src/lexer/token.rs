#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // End & Invalid
    Eof,
    Invalid,

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

