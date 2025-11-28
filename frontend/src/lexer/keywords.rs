use crate::lexer::token::TokenKind;

pub fn lookup_keyword(ident: String) -> TokenKind {
    match ident.as_str() {
        "fn" => TokenKind::Fn,
        "return" => TokenKind::Return,
        "let" => TokenKind::Let,
        "var" => TokenKind::Var,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "while" => TokenKind::While,
        "break" => TokenKind::Break,
        "coutinue" => TokenKind::Continue,
        _ => TokenKind::Identifier(ident),
    }
}
