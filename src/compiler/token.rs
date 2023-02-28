use crate::utils::exit_with_err_msg;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Identifier,
    String,
    Int,
    Float,
    LeftBrace,
    RightBrace,
    Star,
    Slash,
    Dot,
    DotDot,
    Sharp,
    Comma,
    Semicolon,
    Colon,
    ColonColon,
    Minus,
    Plus,
    BangEqual,
    Bang,
    Pipe,
    EqualEqual,
    Equal,
    LessEqual,
    Less,
    GreaterEqual,
    Greater,
    SlashEqual,
    MinusEqual,
    StarEqual,
    PlusEqual,
    Question,
    False,
    True,
    Fn,
    Loop,
    If,
    Else,
    Print,
    For,
    While,
    Struct,
    Internal,
    Enum,
    And,
    Or,
    Let,
    Var,
    Nil,
    Return,
    Break,
    Continue,
    Inc,
    Dec,
    Hat,
    At,
    Impl,
    Init,
    Switch,
    MinusGreater,
    Fall,
    Import,
    Defer,
    Eof,
}

#[derive(Clone, PartialEq, Debug)]
pub enum TokenSuffix {
    None,
    D,
    L,
    LL,
    U,
    UL,
    ULL,
}

impl TokenSuffix {
    pub fn from(str: &String) -> Option<TokenSuffix> {
        match str.as_str() {
            "D" => Some(TokenSuffix::D),
            "L" => Some(TokenSuffix::L),
            "LL" => Some(TokenSuffix::LL),
            "U" => Some(TokenSuffix::U),
            "UL" => Some(TokenSuffix::UL),
            "ULL" => Some(TokenSuffix::ULL),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum TokenMod {
    None,
    Hex,
    Bin,
    Oct,
    Char,
    Multiline,
}

pub enum TypeKind {
    Base,
    Pointer,
    Array,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    None,
    Int(i32),
    Float(f32),
    Str(String),
}

impl Value {
    pub fn from(slice: &'static str) -> Value {
        Value::Str(slice.to_string())
    }

    pub fn get_str(&self) -> String {
        match self {
            Self::Str(s) => s.clone(),
            _ => exit_with_err_msg("expected Value::Str"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub modd: TokenMod,
    pub suffix: TokenSuffix,
    pub value: Value,
    pub line: u32,
}

impl Token {
    pub fn new(kind: TokenKind, line: u32, value: Value) -> Self {
        Token {
            kind: kind,
            modd: TokenMod::None,
            suffix: TokenSuffix::None,
            value: value,
            line: line,
        }
    }

    pub fn new_number(kind: TokenKind, line: u32, value: Value, suffix: TokenSuffix) -> Self {
        Token {
            kind: kind,
            modd: TokenMod::None,
            suffix: suffix,
            value: value,
            line: line,
        }
    }

    pub fn eof() -> Self {
        Token {
            kind: TokenKind::Eof,
            value: Value::None,
            modd: TokenMod::None,
            suffix: TokenSuffix::None,
            line: 0,
        }
    }
}
