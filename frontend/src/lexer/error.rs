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

    #[error("Invalid number")]
    #[diagnostic(
        code(lexer::invalid_number),
        help(
            "This number is not valid in Dalton. Check digits, underscores, base prefixes, or float format."
        )
    )]
    InvalidNumber {
        #[label("invalid number starts here")]
        span: SourceSpan,
    },

    #[error("Invalid digit '{digit}' for base {base}")]
    #[diagnostic(
        code(lexer::number_invalid_digit),
        help("The digit is not allowed in this numeric base.")
    )]
    InvalidDigitForBase {
        digit: char,
        base: u8,

        #[label("invalid digit here")]
        span: SourceSpan,
    },

    #[error("Misplaced underscore in number")]
    #[diagnostic(
        code(lexer::number_bad_underscore),
        help("Underscores must be between digits, not at the start or end.")
    )]
    InvalidUnderscore {
        #[label("underscore not allowed here")]
        span: SourceSpan,
    },

    #[error("Unterminated character literal")]
    #[diagnostic(
        code(lexer::unterminated_char),
        help("Character literals must end with a closing `'`.")
    )]
    UnterminatedChar {
        #[label("char literal starts here")]
        span: SourceSpan,
    },
    #[error("Newline in character literal")]
    #[diagnostic(
        code(lexer::newline_in_char),
        help("Character literals cannot contain newlines.")
    )]
    NewlineInChar {
        #[label("newline inside char literal")]
        span: SourceSpan,
    },

    #[error("Invalid character literal")]
    #[diagnostic(
        code(lexer::invalid_char_literal),
        help("Character literal must contain exactly one character or one escape.")
    )]
    InvalidCharLiteral {
        #[label("invalid character literal here")]
        span: SourceSpan,
    },
}
