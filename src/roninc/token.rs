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
    Char(String),
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
    MinusEq,
    PlusEq,
    RArrow,
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
impl PartialEq for Delimiter {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl PartialEq for LitKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Char(l0), Self::Char(r0)) => l0 == r0,
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl PartialEq for PermKind {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl PartialEq for TokenKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Undef(l0), Self::Undef(r0)) => l0 == r0,
            (Self::Ident(l0), Self::Ident(r0)) => l0 == r0,
            (Self::Literal(l0), Self::Literal(r0)) => l0 == r0,
            (Self::Permission(l0), Self::Permission(r0)) => l0 == r0,
            (Self::OpenDelim(l0), Self::OpenDelim(r0)) => l0 == r0,
            (Self::CloseDelim(l0), Self::CloseDelim(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl TokenKind {
    pub(crate) fn is_permission(&self) -> bool {
        match self {
            TokenKind::Permission(_perm_kind) => true,
            _ => false,
        }
    }

    pub(crate) fn match_punctuation(ch: &char) -> Option<Self> {
        match ch {
            ';' => Some(Self::Semi),
            ':' => Some(Self::Colon),
            '{' => Some(Self::OpenDelim(Delimiter::Brace)),
            '(' => Some(Self::OpenDelim(Delimiter::Paren)),
            ')' => Some(Self::CloseDelim(Delimiter::Paren)),
            '}' => Some(Self::CloseDelim(Delimiter::Brace)),
            '[' => Some(Self::OpenDelim(Delimiter::Bracket)),
            ']' => Some(Self::CloseDelim(Delimiter::Bracket)),
            '=' => Some(Self::Eq),
            '-' => Some(Self::Minus),
            '+' => Some(Self::Plus),
            '/' => Some(Self::Div),
            '*' => Some(Self::Star),
            '.' => Some(Self::Dot),
            ',' => Some(Self::Comma),
            '\'' => Some(Self::SingleQuote),
            '\"' => Some(Self::DoubleQuote),
            '!' => Some(Self::Not),
            '&' => Some(Self::And),
            '|' => Some(Self::Or),
            '>' => Some(Self::Gt),
            '<' => Some(Self::Lt),
            '#' => Some(Self::Hashtag),
            '%' => Some(Self::Percent),
            '\\' => Some(Self::BSlash),
            '@' => Some(Self::At),
            _ => None,
        }
    }

    pub(crate) fn get_combo(&self, ch: &char) -> Option<TokenKind> {
        let kind = match Self::match_punctuation(ch) {
            Some(kind) => kind,
            None => {
                return None;
            }
        };

        match (self, &kind) {
            (Self::Colon, Self::Colon) => Some(Self::ColonColon),
            (Self::Gt, Self::Eq) => Some(Self::GtEq),
            (Self::Lt, Self::Eq) => Some(Self::LtEq),
            (Self::Eq, Self::Eq) => Some(Self::EqEq),
            (Self::Not, Self::Eq) => Some(Self::NotEq),
            (Self::Minus, Self::Eq) => Some(Self::MinusEq),
            (Self::Plus, Self::Eq) => Some(Self::PlusEq),
            (Self::Minus, Self::Gt) => Some(Self::RArrow),
            (Self::And, Self::And) => Some(Self::AndAnd),
            (Self::Or, Self::Or) => Some(Self::OrOr),
            (_, _) => None,
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
