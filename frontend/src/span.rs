use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span(pub Range<usize>);

impl From<Range<usize>> for Span {
    fn from(value: Range<usize>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T> {
    pub kind: T,
    pub span: Span,
}
