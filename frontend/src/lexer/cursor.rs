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

#[cfg(test)]
mod tests {
    use super::Cursor;

    #[test]
    fn new_cursor_starts_at_pos_0() {
        let c = Cursor::new("abc");
        assert_eq!(c.pos(), 0);
    }

    #[test]
    fn peek_shows_first_char_without_consuming() {
        let mut c = Cursor::new("abc");
        assert_eq!(c.peek(), Some('a'));
        assert_eq!(c.pos(), 0); // pos must not change
    }

    #[test]
    fn consume_returns_chars_in_order() {
        let mut c = Cursor::new("abc");
        assert_eq!(c.consume(), Some('a'));
        assert_eq!(c.consume(), Some('b'));
        assert_eq!(c.consume(), Some('c'));
    }

    #[test]
    fn consume_increments_index_only_when_char_exists() {
        let mut c = Cursor::new("a");
        assert_eq!(c.consume(), Some('a'));
        assert_eq!(c.pos(), 1);

        // Now EOF
        assert_eq!(c.consume(), None);
        assert_eq!(c.pos(), 1); // index must NOT increase at EOF
    }

    #[test]
    fn peek_on_eof_returns_none() {
        let mut c = Cursor::new("");
        assert_eq!(c.peek(), None);
    }

    #[test]
    fn peek_does_not_move_cursor() {
        let mut c = Cursor::new("xyz");
        assert_eq!(c.peek(), Some('x'));
        assert_eq!(c.peek(), Some('x'));
        assert_eq!(c.pos(), 0);

        c.consume();
        assert_eq!(c.peek(), Some('y'));
        assert_eq!(c.pos(), 1);
    }
}
