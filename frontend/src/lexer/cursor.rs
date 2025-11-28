use std::{iter::Peekable, str::Chars};

#[derive(Clone)]
pub struct Cursor<'a> {
    chars: Peekable<Chars<'a>>,
    index: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            chars: src.chars().peekable(),
            index: 0,
        }
    }

    #[inline]
    pub fn consume(&mut self) -> Option<char> {
        let ch = self.chars.next();
        if ch.is_some() {
            self.index += 1;
        }
        ch
    }

    #[inline]
    pub fn peek(&mut self) -> Option<char> {
        self.chars.peek().cloned()
    }

    #[inline]
    pub fn pos(&self) -> usize {
        self.index
    }
}
