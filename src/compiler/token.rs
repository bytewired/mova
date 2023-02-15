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

pub enum TokenSuffix {
    None,
    D,
    L,
    LL,
    U,
    UL,
    ULL,
}

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

#[derive(PartialEq)]
pub enum Value {
    None,
    Int(i32),
    Float(f32),
    Str(String),
}

pub struct Token {
    pub kind: TokenKind,
    pub modd: TokenMod,
    pub suffix: TokenSuffix,
    pub value: Value,
    pub line: u32,
}

impl Token {
    pub fn new(kind: TokenKind, value: Value) -> Self {
        Token {
            kind: kind,
            modd: TokenMod::None,
            suffix: TokenSuffix::None,
            value: value,
            line: 0,
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
