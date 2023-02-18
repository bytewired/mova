use crate::compiler::token::{Token, TokenKind, Value};
use crate::utils::exit_with_err_msg;
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
            } else if let Some(string) = self.eat_string() {
                return string;
            } else {
                return match self.iter.next() {
                    Some(c) => match c {
                        '(' => Token::new(TokenKind::LeftParen, Value::None),
                        ')' => Token::new(TokenKind::RightParen, Value::None),
                        '[' => Token::new(TokenKind::LeftBracket, Value::None),
                        ']' => Token::new(TokenKind::RightBracket, Value::None),
                        '{' => Token::new(TokenKind::LeftBrace, Value::None),
                        '}' => Token::new(TokenKind::RightBrace, Value::None),
                        ';' => Token::new(TokenKind::Semicolon, Value::None),
                        ':' => {
                            if self.iter.match_next(c) {
                                Token::new(TokenKind::ColonColon, Value::None)
                            } else {
                                Token::new(TokenKind::Colon, Value::None)
                            }
                        }
                        '.' => {
                            if self.iter.match_next(c) {
                                Token::new(TokenKind::DotDot, Value::None)
                            } else {
                                Token::new(TokenKind::Dot, Value::None)
                            }
                        }
                        '-' => {
                            if self.iter.match_next(c) {
                                Token::new(TokenKind::Dec, Value::None)
                            } else if self.iter.match_next('=') {
                                Token::new(TokenKind::MinusEqual, Value::None)
                            } else if self.iter.match_next('>') {
                                Token::new(TokenKind::MinusGreater, Value::None)
                            } else {
                                Token::new(TokenKind::Minus, Value::None)
                            }
                        }
                        '+' => {
                            if self.iter.match_next(c) {
                                Token::new(TokenKind::Inc, Value::None)
                            } else if self.iter.match_next('=') {
                                Token::new(TokenKind::PlusEqual, Value::None)
                            } else {
                                Token::new(TokenKind::Plus, Value::None)
                            }
                        }
                        '/' => {
                            // todo: add parsing comment
                            if self.iter.match_next('=') {
                                Token::new(TokenKind::SlashEqual, Value::None)
                            } else {
                                Token::new(TokenKind::Slash, Value::None)
                            }
                        }
                        '*' => {
                            if self.iter.match_next(c) {
                                Token::new(TokenKind::StarEqual, Value::None)
                            } else {
                                Token::new(TokenKind::Star, Value::None)
                            }
                        }
                        ',' => Token::new(TokenKind::Comma, Value::None),
                        '#' => Token::new(TokenKind::Sharp, Value::None),
                        '|' => Token::new(TokenKind::Pipe, Value::None),
                        '@' => Token::new(TokenKind::At, Value::None),
                        '?' => Token::new(TokenKind::Question, Value::None),
                        '!' => {
                            if self.iter.match_next('=') {
                                Token::new(TokenKind::BangEqual, Value::None)
                            } else {
                                Token::new(TokenKind::Bang, Value::None)
                            }
                        }
                        '=' => {
                            if self.iter.match_next('=') {
                                Token::new(TokenKind::EqualEqual, Value::None)
                            } else {
                                Token::new(TokenKind::Equal, Value::None)
                            }
                        }
                        '<' => {
                            if self.iter.match_next('=') {
                                Token::new(TokenKind::LessEqual, Value::None)
                            } else {
                                Token::new(TokenKind::Less, Value::None)
                            }
                        }
                        '>' => {
                            if self.iter.match_next('=') {
                                Token::new(TokenKind::GreaterEqual, Value::None)
                            } else {
                                Token::new(TokenKind::Greater, Value::None)
                            }
                        }
                        '^' => Token::new(TokenKind::Hat, Value::None),
                        _ => continue,
                    },
                    None => Token::eof(),
                };
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

            let is_next_dot = matches!(self.iter.next(), Some(c) if c == '.');
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

    fn eat_string(&mut self) -> Option<Token> {
        if *self.iter.current().unwrap() != '\"' {
            return Option::None;
        }

        let mut string = String::new();

        while let Some(c) = self.iter.next() {
            if c == '\"' {
                break;
            } else {
                string.push(c);
            }
        }

        if self.iter.is_at_end() {
            exit_with_err_msg("Unterminated string");
        }

        Option::Some(Token::new(TokenKind::String, Value::Str(string)))
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

    fn next(&mut self) -> Option<char> {
        self.current_char = self.chars.next();

        self.current_char
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn match_next(&mut self, c: char) -> bool {
        return match self.peek() {
            Some(peeked) => {
                let matches = c == *peeked;

                if c == *peeked {
                    self.next();
                }

                matches
            }
            None => {
                exit_with_err_msg("Unexpected end of file.");
                false
            }
        };
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

        let token = lexer.eat_number().unwrap();

        assert!(token.kind == TokenKind::Int);
        assert!(token.value == Value::Int(123456));
    }

    #[test]
    fn eat_number_float() {
        let string = "3.14";
        let mut lexer = Lexer::new(string);

        let token = lexer.eat_number().unwrap();

        assert!(token.kind == TokenKind::Float);
        assert!(token.value == Value::Float(3.14));
    }

    #[test]
    fn eat_string_is_valid_string() {
        let string = "\"abcdef\"";
        let mut lexer = Lexer::new(string);

        let token = lexer.eat_string().unwrap();

        assert!(token.kind == TokenKind::String);
        assert!(token.value == Value::Str(String::from("abcdef")));
    }

    #[test]
    #[ignore = "need to handle process::exit()"]
    fn eat_string_is_unterminated_string() {
        let string = "\"abcdef";
        let mut lexer = Lexer::new(string);

        lexer.eat_string();
    }
}
