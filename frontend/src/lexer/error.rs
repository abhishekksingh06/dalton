use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum LexerError {
    #[error("Unexpected character '{found}'")]
    #[diagnostic(
        code(lexer::unexpected_char),
        help("This character is not valid in this context.")
    )]
    UnexpectedChar {
        found: char,

        #[label("unexpected character found here")]
        span: SourceSpan,
    },

    #[error("Unexpected end of file")]
    #[diagnostic(
        code(lexer::unexpected_eof),
        help("The file ended before the token was completed.")
    )]
    UnexpectedEof {
        #[label("input ends here")]
        span: SourceSpan,
    },
}
