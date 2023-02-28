use std::iter::Peekable;
use std::str::Chars;

pub struct CharsNavigator<'a> {
    chars: Peekable<Chars<'a>>,
    current: Option<char>,
    line: u32,
    column: u32,
}

impl<'a> CharsNavigator<'a> {
    pub fn new(mut chars: Chars<'a>) -> Self {
        let current_char = chars.next();

        CharsNavigator {
            chars: chars.peekable(),
            current: current_char,
            line: 1,
            column: 1,
        }
    }

    pub fn next(&mut self) -> Option<char> {
        self.current = self.chars.next();

        self.check_line_column();
        self.current
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    pub fn next_if_match(&mut self, c: char) -> bool {
        let next = self.chars.next_if(|next| *next == c);

        if next.is_some() {
            self.current = next;
        };

        self.check_line_column();

        next.is_some()
    }

    pub fn current(&self) -> Option<char> {
        self.current
    }

    pub const fn is_at_end(&self) -> bool {
        self.current.is_none()
    }

    pub const fn line(&self) -> u32 {
        self.line
    }

    pub const fn column(&self) -> u32 {
        self.column
    }

    fn check_line_column(&mut self) {
        if let Some(c) = self.current {
            if c == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        };
    }
}
