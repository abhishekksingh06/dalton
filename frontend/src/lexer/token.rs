use crate::span::Spanned;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // End,
    Eof,

    // Identifiers & Literals (with values)
    Identifier(String),
    IntegerLiteral { value: String, base: u32 },
    FloatLiteral(String),
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

impl TokenKind {
    pub fn to_string_value(&self) -> String {
        match self {
            TokenKind::Eof => "EOF".into(),

            TokenKind::Identifier(s) => s.clone(),

            TokenKind::IntegerLiteral { value, base } => {
                format!("{} (base {})", value, base)
            }

            TokenKind::FloatLiteral(fl) => fl.to_string(),

            TokenKind::StringLiteral(s) => s.clone(),

            TokenKind::CharLiteral(c) => c.to_string(),

            TokenKind::True => "true".into(),
            TokenKind::False => "false".into(),

            // Keywords
            TokenKind::Fn => "fn".into(),
            TokenKind::Return => "return".into(),
            TokenKind::Let => "let".into(),
            TokenKind::Var => "var".into(),
            TokenKind::If => "if".into(),
            TokenKind::Else => "else".into(),
            TokenKind::For => "for".into(),
            TokenKind::While => "while".into(),
            TokenKind::Break => "break".into(),
            TokenKind::Continue => "continue".into(),

            // Operators
            TokenKind::Plus => "+".into(),
            TokenKind::Minus => "-".into(),
            TokenKind::Star => "*".into(),
            TokenKind::Slash => "/".into(),
            TokenKind::Percent => "%".into(),
            TokenKind::Caret => "^".into(),
            TokenKind::Bang => "!".into(),
            TokenKind::AndAnd => "&&".into(),
            TokenKind::OrOr => "||".into(),

            // Comparisons
            TokenKind::EqualEqual => "==".into(),
            TokenKind::NotEqual => "!=".into(),
            TokenKind::Less => "<".into(),
            TokenKind::Greater => ">".into(),
            TokenKind::LessEqual => "<=".into(),
            TokenKind::GreaterEqual => ">=".into(),

            // Assignment
            TokenKind::Equal => "=".into(),

            // Compound assignment
            TokenKind::PlusEqual => "+=".into(),
            TokenKind::MinusEqual => "-=".into(),
            TokenKind::StarEqual => "*=".into(),
            TokenKind::SlashEqual => "/=".into(),

            // Delimiters
            TokenKind::LParen => "(".into(),
            TokenKind::RParen => ")".into(),
            TokenKind::LBrace => "{".into(),
            TokenKind::RBrace => "}".into(),
            TokenKind::LBracket => "[".into(),
            TokenKind::RBracket => "]".into(),

            // Punctuation
            TokenKind::Comma => ",".into(),
            TokenKind::Dot => ".".into(),
            TokenKind::Semicolon => ";".into(),
            TokenKind::Colon => ":".into(),
            TokenKind::Arrow => "->".into(),
        }
    }
}

pub type Token = Spanned<TokenKind>;
