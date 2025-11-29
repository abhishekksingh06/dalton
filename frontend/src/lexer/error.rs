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
    #[error("Unterminated string literal")]
    #[diagnostic(
        code(lexer::string_unterminated),
        help("A string literal must end with a matching double quote.")
    )]
    UnterminatedString {
        #[label("string literal starts here but never ends")]
        span: SourceSpan,
    },

    #[error("Invalid escape sequence '\\{escape}'")]
    #[diagnostic(
        code(lexer::string_invalid_escape),
        help("Use only valid escape sequences such as \\n, \\t, \\\", \\\\, etc.")
    )]
    InvalidEscapeSequence {
        escape: char,

        #[label("invalid escape sequence here")]
        span: SourceSpan,
    },

    #[error("Escape sequence not completed")]
    #[diagnostic(
        code(lexer::string_incomplete_escape),
        help("The escape sequence ends abruptly or is incomplete.")
    )]
    IncompleteEscapeSequence {
        #[label("escape starts here")]
        span: SourceSpan,
    },

    #[error("String literal cannot contain an unescaped newline")]
    #[diagnostic(
        code(lexer::string_newline),
        help("Use \\n for newline, or close the string before the line break.")
    )]
    NewlineInString {
        #[label("newline inside string literal")]
        span: SourceSpan,
    },

    #[error("Invalid Unicode escape sequence")]
    #[diagnostic(
        code(lexer::string_invalid_unicode),
        help("Unicode escapes must be in the form \\u{{...}} using hex digits.")
    )]
    InvalidUnicodeEscape {
        #[label("invalid unicode escape here")]
        span: SourceSpan,
    },

    #[error("Unicode escape value out of range")]
    #[diagnostic(
        code(lexer::string_unicode_out_of_range),
        help("Unicode value must be within valid scalar range (0x0–0x10FFFF).")
    )]
    UnicodeEscapeOutOfRange {
        value: u32,

        #[label("unicode scalar out of range")]
        span: SourceSpan,
    },
}
