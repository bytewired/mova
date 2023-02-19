use crate::compiler::token::{Token, TokenKind, Value};
use crate::utils::{exit_with_err_msg, print_debug};
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    nav: CharsNavigator<'a>,
    keywords: HashMap<&'static str, TokenKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        if source.is_empty() {
            panic!("File is empty")
        }

        Lexer {
            nav: CharsNavigator::new(source.chars()),
            keywords: Self::init_keywords(),
        }
    }

    fn init_keywords() -> HashMap<&'static str, TokenKind> {
        let mut keywords = HashMap::new();

        keywords.insert("var", TokenKind::Var);
        keywords.insert("fn", TokenKind::Fn);
        keywords.insert("return", TokenKind::Return);
        keywords.insert("let", TokenKind::Let);
        keywords.insert("else", TokenKind::Else);
        keywords.insert("loop", TokenKind::Loop);
        keywords.insert("if", TokenKind::If);
        keywords.insert("import", TokenKind::Import);
        keywords.insert("false", TokenKind::False);
        keywords.insert("true", TokenKind::True);
        keywords.insert("for", TokenKind::For);
        keywords.insert("print", TokenKind::Print);
        keywords.insert("while", TokenKind::While);
        keywords.insert("struct", TokenKind::Struct);
        keywords.insert("internal", TokenKind::Internal);
        keywords.insert("enum", TokenKind::Enum);
        keywords.insert("and", TokenKind::And);
        keywords.insert("or", TokenKind::Or);
        keywords.insert("nil", TokenKind::Nil);
        keywords.insert("break", TokenKind::Break);
        keywords.insert("continue", TokenKind::Continue);
        keywords.insert("impl", TokenKind::Impl);
        keywords.insert("init", TokenKind::Init);
        keywords.insert("switch", TokenKind::Switch);
        keywords.insert("fall", TokenKind::Fall);
        keywords.insert("defer", TokenKind::Defer);

        return keywords;
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut vec = Vec::new();

        loop {
            let token = self.eat_token();

            if matches!(token.kind, TokenKind::Eof) {
                break;
            } else {
                print_debug(format!("{:?}", token).as_str());
                vec.push(token);
            }
        }

        vec
    }

    fn eat_token(&mut self) -> Token {
        while !self.nav.is_at_end() {
            if let Some(number) = self.eat_number() {
                return number;
            } else if let Some(string) = self.eat_string() {
                return string;
            } else if let Some(identifier) = self.eat_identifier() {
                return identifier;
            } else {
                return match self.nav.current {
                    Some(c) => {
                        let token = match c {
                            '(' => {
                                Token::new(TokenKind::LeftParen, self.nav.line, Value::from("("))
                            }
                            ')' => {
                                Token::new(TokenKind::RightParen, self.nav.line, Value::from(")"))
                            }
                            '[' => {
                                Token::new(TokenKind::LeftBracket, self.nav.line, Value::from("["))
                            }
                            ']' => {
                                Token::new(TokenKind::RightBracket, self.nav.line, Value::from("]"))
                            }
                            '{' => {
                                Token::new(TokenKind::LeftBrace, self.nav.line, Value::from("{"))
                            }
                            '}' => {
                                Token::new(TokenKind::RightBrace, self.nav.line, Value::from("}"))
                            }
                            ';' => {
                                Token::new(TokenKind::Semicolon, self.nav.line, Value::from(":"))
                            }
                            ':' => {
                                if self.nav.match_next(c) {
                                    Token::new(
                                        TokenKind::ColonColon,
                                        self.nav.line,
                                        Value::from("::"),
                                    )
                                } else {
                                    Token::new(TokenKind::Colon, self.nav.line, Value::from(":"))
                                }
                            }
                            '.' => {
                                if self.nav.match_next(c) {
                                    Token::new(TokenKind::DotDot, self.nav.line, Value::from(".."))
                                } else {
                                    Token::new(TokenKind::Dot, self.nav.line, Value::from("."))
                                }
                            }
                            '-' => {
                                if self.nav.match_next(c) {
                                    Token::new(TokenKind::Dec, self.nav.line, Value::from("--"))
                                } else if self.nav.match_next('=') {
                                    Token::new(
                                        TokenKind::MinusEqual,
                                        self.nav.line,
                                        Value::from("-+"),
                                    )
                                } else if self.nav.match_next('>') {
                                    Token::new(
                                        TokenKind::MinusGreater,
                                        self.nav.line,
                                        Value::from("->"),
                                    )
                                } else {
                                    Token::new(TokenKind::Minus, self.nav.line, Value::from("-"))
                                }
                            }
                            '+' => {
                                if self.nav.match_next(c) {
                                    Token::new(TokenKind::Inc, self.nav.line, Value::from("++"))
                                } else if self.nav.match_next('=') {
                                    Token::new(
                                        TokenKind::PlusEqual,
                                        self.nav.line,
                                        Value::from("+="),
                                    )
                                } else {
                                    Token::new(TokenKind::Plus, self.nav.line, Value::from("+"))
                                }
                            }
                            '/' => {
                                // todo: add parsing comment
                                if self.nav.match_next('=') {
                                    Token::new(
                                        TokenKind::SlashEqual,
                                        self.nav.line,
                                        Value::from("/="),
                                    )
                                } else {
                                    Token::new(TokenKind::Slash, self.nav.line, Value::from("/"))
                                }
                            }
                            '*' => {
                                if self.nav.match_next('=') {
                                    Token::new(
                                        TokenKind::StarEqual,
                                        self.nav.line,
                                        Value::from("*="),
                                    )
                                } else {
                                    Token::new(TokenKind::Star, self.nav.line, Value::from("*"))
                                }
                            }
                            ',' => Token::new(TokenKind::Comma, self.nav.line, Value::from(",")),
                            '#' => Token::new(TokenKind::Sharp, self.nav.line, Value::from("#")),
                            '|' => Token::new(TokenKind::Pipe, self.nav.line, Value::from("|")),
                            '@' => Token::new(TokenKind::At, self.nav.line, Value::from("@")),
                            '?' => Token::new(TokenKind::Question, self.nav.line, Value::from("?")),
                            '!' => {
                                if self.nav.match_next('=') {
                                    Token::new(
                                        TokenKind::BangEqual,
                                        self.nav.line,
                                        Value::from("!="),
                                    )
                                } else {
                                    Token::new(TokenKind::Bang, self.nav.line, Value::from("!"))
                                }
                            }
                            '=' => {
                                if self.nav.match_next('=') {
                                    Token::new(
                                        TokenKind::EqualEqual,
                                        self.nav.line,
                                        Value::from("=="),
                                    )
                                } else {
                                    Token::new(TokenKind::Equal, self.nav.line, Value::from("="))
                                }
                            }
                            '<' => {
                                if self.nav.match_next('=') {
                                    Token::new(
                                        TokenKind::LessEqual,
                                        self.nav.line,
                                        Value::from("<="),
                                    )
                                } else {
                                    Token::new(TokenKind::Less, self.nav.line, Value::from("<"))
                                }
                            }
                            '>' => {
                                if self.nav.match_next('=') {
                                    Token::new(
                                        TokenKind::GreaterEqual,
                                        self.nav.line,
                                        Value::from(">="),
                                    )
                                } else {
                                    Token::new(TokenKind::Greater, self.nav.line, Value::from(">"))
                                }
                            }
                            '^' => Token::new(TokenKind::Hat, self.nav.line, Value::from("^")),
                            '\n' => {
                                self.nav.next();
                                continue;
                            }
                            ' ' => {
                                self.nav.next();
                                continue;
                            }
                            _ => {
                                exit_with_err_msg(
                                    format!(
                                        "unknown token at {}:{}",
                                        self.nav.line, self.nav.column
                                    )
                                    .as_str(),
                                );
                                continue;
                            }
                        };

                        self.nav.next();
                        token
                    }
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

        if !is_digit(self.nav.current.unwrap()) {
            return Option::None;
        }

        // parsing mantissa
        while !self.nav.is_at_end() {
            let current = self.nav.current.unwrap();

            if is_digit(current) {
                mantissa.push(current)
            } else {
                break;
            }

            let is_next_dot = matches!(self.nav.next(), Some(c) if c == '.');
            let is_after_next_digit = matches!(self.nav.peek(), Some(c) if is_digit(*c));

            if is_next_dot && is_after_next_digit {
                has_exponent = true;
                self.nav.next();
                break;
            }
        }

        if has_exponent {
            while !self.nav.is_at_end() {
                let current = self.nav.current.unwrap();

                if is_digit(current) {
                    exponent.push(current);
                    self.nav.next();
                } else {
                    break;
                }
            }

            let float = format!("{mantissa}.{exponent}").parse::<f32>().unwrap();
            return Option::Some(Token::new(
                TokenKind::Float,
                self.nav.line,
                Value::Float(float),
            ));
        }

        Option::Some(Token::new(
            TokenKind::Int,
            self.nav.line,
            Value::Int(mantissa.parse::<i32>().unwrap()),
        ))
    }

    fn eat_string(&mut self) -> Option<Token> {
        if self.nav.current.unwrap() != '\"' {
            return Option::None;
        }

        let mut string = String::new();

        while let Some(c) = self.nav.next() {
            if c == '\"' {
                break;
            } else {
                string.push(c);
            }
        }

        if self.nav.is_at_end() {
            exit_with_err_msg("Unterminated string");
        }

        Option::Some(Token::new(
            TokenKind::String,
            self.nav.line,
            Value::Str(string),
        ))
    }

    fn eat_identifier(&mut self) -> Option<Token> {
        if matches!(self.nav.current, Some(c) if !c.is_alphabetic() && c != '_') {
            return None;
        }

        let mut identifier = String::from(self.nav.current.unwrap());

        loop {
            match self.nav.peek() {
                Some(c) => {
                    if c.is_alphabetic() || *c == '_' || is_digit(*c) {
                        identifier.push(self.nav.next().unwrap());
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }

        self.nav.next();

        match self.keywords.get_key_value(identifier.as_str()) {
            Some(token) => {
                Option::Some(Token::new(*token.1, self.nav.line, Value::Str(identifier)))
            }
            None => Option::Some(Token::new(
                TokenKind::Identifier,
                self.nav.line,
                Value::Str(identifier),
            )),
        }
    }
}

struct CharsNavigator<'a> {
    chars: Peekable<Chars<'a>>,
    current: Option<char>,
    line: u32,
    column: u32,
}

impl<'a> CharsNavigator<'a> {
    fn new(mut chars: Chars<'a>) -> Self {
        let current_char = chars.next();

        CharsNavigator {
            chars: chars.peekable(),
            current: current_char,
            line: 1,
            column: 1,
        }
    }

    fn next(&mut self) -> Option<char> {
        self.current = self.chars.next();

        if let Some(c) = self.current {
            if c == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        }
        self.current
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn match_next(&mut self, c: char) -> bool {
        let next = self.chars.next_if(|next| *next == c);

        if next.is_some() {
            self.current = next;
        };

        if let Some(c) = self.current {
            if c == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        };

        next.is_some()
    }

    const fn is_at_end(&self) -> bool {
        self.current.is_none()
    }
}

// utils
fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eat_number_int() {
        let source = "123456";
        let mut lexer = Lexer::new(source);

        let token = lexer.eat_number().unwrap();

        assert_eq!(token.kind, TokenKind::Int);
        assert_eq!(token.value, Value::Int(123456));
    }

    #[test]
    fn eat_number_float() {
        let source = "3.14";
        let mut lexer = Lexer::new(source);

        let token = lexer.eat_number().unwrap();

        assert_eq!(token.kind, TokenKind::Float);
        assert_eq!(token.value, Value::Float(3.14));
    }

    #[test]
    fn eat_string_is_valid_string() {
        let source = "\"abcdef\"";
        let mut lexer = Lexer::new(source);

        let token = lexer.eat_string().unwrap();

        assert_eq!(token.kind, TokenKind::String);
        assert_eq!(token.value, Value::Str("abcdef".to_string()));
    }

    #[test]
    #[ignore = "need to handle process::exit()"]
    fn eat_string_is_unterminated_string() {
        let source = "\"abcdef";
        let mut lexer = Lexer::new(source);

        lexer.eat_string();
    }

    #[test]
    fn eat_identifier_starts_with_undescore() {
        let source = "_asd";
        let mut lexer = Lexer::new(source);

        let token = lexer.eat_identifier().unwrap();

        assert_eq!(token.kind, TokenKind::Identifier);
        assert_eq!(token.value, Value::Str("_asd".to_string()))
    }

    #[test]
    fn eat_identifier_starts_with_number() {
        let source = "10_asd";
        let mut lexer = Lexer::new(source);

        let token = lexer.eat_identifier();

        assert!(token.is_none());
    }

    #[test]
    fn eat_identifier_import() {
        let source = "import";
        let mut lexer = Lexer::new(source);

        let token = lexer.eat_identifier().unwrap();

        assert_eq!(token.kind, TokenKind::Import);
        assert_eq!(token.value, Value::Str("import".to_string()))
    }

    #[test]
    fn parse_empty_fn_without_parameters() {
        let source = "fn foo() { }";
        let mut lexer = Lexer::new(source);

        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].kind, TokenKind::Fn);
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[1].value, Value::from("foo"));
        assert_eq!(tokens[2].kind, TokenKind::LeftParen);
        assert_eq!(tokens[3].kind, TokenKind::RightParen);
        assert_eq!(tokens[4].kind, TokenKind::LeftBrace);
        assert_eq!(tokens[5].kind, TokenKind::RightBrace);
    }
}
