// Data
// #[derive(Clone, Copy, Debug)]
// pub struct Span {
//     pub start: LnCol,
//     pub end: LnCol,
// }

#[derive(Clone, Copy, Debug)]
pub struct LnCol {
    pub ln: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: LnCol,
}

#[derive(Debug)]
pub enum LitKind {
    Char(char),
    Integer(String),
    Float(String),
    String(String),
}

#[derive(Debug)]
pub enum PermKind {
    R,
    RW,
}

#[derive(Debug)]
pub enum Delimiter {
    Paren,
    Brace,
    Bracket,
}

#[derive(Debug)]
pub enum TokenKind {
    Undef(char),
    Ident(String),
    Literal(LitKind),
    Permission(PermKind),
    ColonColon,
    GtEq,
    LtEq,
    EqEq,
    NotEq,
    MinusArrow,
    MinusEq,
    PlusEq,
    AndAnd,
    OrOr,
    At,
    Div,
    BSlash,
    Not,
    Hashtag,
    Percent,
    And,
    Or,
    Star,
    SingleQuote,
    DoubleQuote,
    Semi,
    Colon,
    Gt,
    Lt,
    Eq,
    Minus,
    Plus,
    Dot,
    Comma,
    OpenDelim(Delimiter),
    CloseDelim(Delimiter),
    Main,
    Return,
    If,
    Fn,
    I32,
    Isize,
    U32,
    Usize,
    F32,
    True,
    False,
    EOF,
}

// Implementations
impl TokenKind {
    pub(crate) fn is_permission(&self) -> bool {
        match self {
            TokenKind::Permission(_perm_kind) => true,
            _ => false,
        }
    }
}

impl LnCol {
    /// Creates a new [`Position`].
    pub(crate) fn new(ln: usize, col: usize) -> Self {
        LnCol { ln: ln, col: col }
    }

    pub(crate) fn update(&mut self, ln: usize, col: usize) -> Self {
        let span = self.clone();

        self.ln += ln;
        self.col += col;

        span
    }
}

// impl Span {
//     pub(crate) fn new(start: LnCol, end: LnCol) -> Self {
//         Span { start, end }
//     }
// }

impl Token {
    /// Creates a new [`Token`].
    pub(crate) fn new(kind: TokenKind, pos: LnCol) -> Token {
        Token { kind, pos }
    }
}
