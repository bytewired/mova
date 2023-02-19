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

#[derive(Debug)]
pub enum TokenSuffix {
    None,
    D,
    L,
    LL,
    U,
    UL,
    ULL,
}

#[derive(Debug)]
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

#[derive(PartialEq, Debug)]
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
}

#[derive(Debug)]
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