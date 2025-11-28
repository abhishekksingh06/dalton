use crate::lexer::token::TokenKind;

pub fn lookup_keyword(ident: String) -> TokenKind {
    match ident.as_str() {
        "fn" => TokenKind::Fn,
        "return" => TokenKind::Return,
        "let" => TokenKind::Let,
        "var" => TokenKind::Var,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "for" => TokenKind::For,
        "while" => TokenKind::While,
        "break" => TokenKind::Break,
        "coutinue" => TokenKind::Continue,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        _ => TokenKind::Identifier(ident),
    }
}
