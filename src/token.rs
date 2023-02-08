pub enum Token {
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

enum TokenSuffix {
    d,
    L,
    LL,
    U,
    UL,
    ULL,
}

enum TokenMod {
    Hex,
    Bin,
    Oct,
    Char,
    Multiline,
}

enum TypeKind {
    Base,
    Pointer,
    Array,
}

pub struct Token {
    kind: TokenKind,
    name: String,
    modd: TokenMod,
    suffix: TokenSuffix,
    line: i32,
    // todo: add Value; https://github.com/seamless-os/Mova/blob/master/src/token.h#L66
}

impl Token {
    fn new(kind: TokenKind, name: String) -> Self {
        Token {
            kind: kind,
            name: name,
        }
    }
}
