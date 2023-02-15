use crate::compiler::token::{Token, TokenKind, Value};
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    iter: CharsIterator<'a>,
    line: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        if source.is_empty() {
            panic!("File is empty")
        }

        Lexer {
            iter: CharsIterator::new(source.chars()),
            line: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut vec = Vec::new();

        while let token = self.eat_token() {
            if matches!(token.kind, TokenKind::Eof) {
                break;
            } else {
                vec.push(token);
            }
        }

        vec
    }

    fn eat_token(&mut self) -> Token {
        while !self.iter.is_at_end() {
            if let Some(number) = self.eat_number() {
                return number;
            } else {
                return Token::eof();
            }
        }

        Token::eof()
    }

    fn eat_number(&mut self) -> Option<Token> {
        let mut mantissa = String::new();
        let mut exponent = String::new();
        let mut has_exponent = false;

        if !is_digit(self.iter.current().unwrap()) {
            return Option::None;
        }

        // parsing mantissa
        while !self.iter.is_at_end() {
            let current = self.iter.current().unwrap();

            if is_digit(current) {
                mantissa.push(*current)
            } else {
                break;
            }

            let is_next_dot = matches!(self.iter.next(), Some(c) if *c == '.');
            let is_after_next_digit = matches!(self.iter.peek(), Some(c) if is_digit(c));

            if is_next_dot && is_after_next_digit {
                has_exponent = true;
                self.iter.next();
                break;
            }
        }

        if has_exponent {
            while !self.iter.is_at_end() {
                let current = self.iter.current().unwrap();

                if is_digit(current) {
                    exponent.push(*current)
                } else {
                    break;
                }
            }

            let float = format!("{mantissa}.{exponent}").parse::<f32>().unwrap();
            return Option::Some(Token::new(TokenKind::Float, Value::Float(float)));
        }

        Option::Some(Token::new(
            TokenKind::Int,
            Value::Int(mantissa.parse::<i32>().unwrap()),
        ))
    }
}

struct CharsIterator<'a> {
    chars: Peekable<Chars<'a>>,
    current_char: Option<char>,
}

impl<'a> CharsIterator<'a> {
    fn new(mut chars: Chars<'a>) -> Self {
        let current_char = chars.next();

        CharsIterator {
            chars: chars.peekable(),
            current_char: current_char,
        }
    }

    const fn current(&self) -> Option<&char> {
        self.current_char.as_ref()
    }

    fn next(&mut self) -> Option<&char> {
        self.current_char = self.chars.next();

        self.current_char.as_ref()
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    const fn is_at_end(&self) -> bool {
        self.current().is_none()
    }

    fn is_new_line(&self) -> bool {
        matches!(self.current(), Some(c) if *c == '\n')
    }
}

// utils
fn is_digit(c: &char) -> bool {
    c.is_digit(10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eat_number_int() {
        let string = "123456";
        let mut lexer = Lexer::new(string);

        let token = lexer.eat_number();

        assert!(token.is_some());
        assert!(token.unwrap().value == Value::Int(123456));
    }

    #[test]
    fn eat_number_float() {
        let string = "3.14";
        let mut lexer = Lexer::new(string);

        let token = lexer.eat_number();

        assert!(token.is_some());
        assert!(token.unwrap().value == Value::Float(3.14));
    }
}
