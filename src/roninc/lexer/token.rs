use core::fmt;

// // // // // // // // // // // // // // // // LN COL

#[derive(Clone, Copy, Debug)]
pub struct LnCol {
    pub ln: usize,
    pub col: usize,
}

impl LnCol {
    /// Creates a new [`LnCol`].
    pub(crate) fn new(ln: usize, col: usize) -> Self {
        LnCol { ln: ln, col: col }
    }

    pub(crate) fn update(&mut self, ln: usize, col: usize) -> Self {
        let lc = self.clone();

        self.ln += ln;
        self.col += col;

        lc
    }

    pub(crate) fn add(&self, ln: usize, col: usize) -> Self {
        Self {
            ln: self.ln + ln,
            col: self.col + col,
        }
    }
}

// // // // // // // // // // // // // // // // SPAN

#[derive(Debug)]
pub struct Span {
    pub start: LnCol,
    pub end: LnCol,
}

impl Span {
    pub(crate) fn new(start: LnCol, end: LnCol) -> Self {
        Span { start, end }
    }
}

// // // // // // // // // // // // // // // // TOKEN
pub type Tokens = Vec<Token>;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: LnCol,
}

impl Token {
    /// Creates a new [`Token`].
    pub(crate) fn new(kind: TokenKind, pos: LnCol) -> Token {
        Token { kind, pos }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // write!(f, "{}",self::)
            TokenKind::Ident(val) => write!(f, "{val}"),
            TokenKind::Literal(kind) => match kind {
                LitKind::Char(val) => write!(f, "\'{val}\'"),
                LitKind::Integer(val) => write!(f, "{val}"),
                LitKind::Float(val) => write!(f, "{val}"),
                LitKind::String(val) => write!(f, "\"{val}\""),
            },
            TokenKind::Permission(val) => match val {
                PermKind::R => write!(f, "R"),
                PermKind::RW => write!(f, "RW"),
            },
            TokenKind::ColonColon => write!(f, "ColonColon"),
            TokenKind::GtEq => write!(f, "GtEq"),
            TokenKind::LtEq => write!(f, "LtEq"),
            TokenKind::EqEq => write!(f, "EqEq"),
            TokenKind::NotEq => write!(f, "NotEq"),
            TokenKind::MinusEq => write!(f, "MinusEq"),
            TokenKind::PlusEq => write!(f, "PlusEq"),
            TokenKind::RArrow => write!(f, "RArrow"),
            TokenKind::AndAnd => write!(f, "AndAnd"),
            TokenKind::OrOr => write!(f, "OrOr"),
            TokenKind::At => write!(f, "At"),
            TokenKind::Div => write!(f, "Div"),
            TokenKind::BSlash => write!(f, "BSlash"),
            TokenKind::Not => write!(f, "Not"),
            TokenKind::Hashtag => write!(f, "Hashtag"),
            TokenKind::Percent => write!(f, "Percent"),
            TokenKind::And => write!(f, "And"),
            TokenKind::Or => write!(f, "Or"),
            TokenKind::Star => write!(f, "Star"),
            TokenKind::SingleQuote => write!(f, "SingleQuote"),
            TokenKind::DoubleQuote => write!(f, "DoubleQuote"),
            TokenKind::Semi => write!(f, "Semi"),
            TokenKind::Colon => write!(f, "Colon"),
            TokenKind::Gt => write!(f, "Gt"),
            TokenKind::Lt => write!(f, "Lt"),
            TokenKind::Eq => write!(f, "Eq"),
            TokenKind::Minus => write!(f, "Minus"),
            TokenKind::Plus => write!(f, "Plus"),
            TokenKind::Dot => write!(f, "Dot"),
            TokenKind::Comma => write!(f, "Comma"),
            TokenKind::OpenDelim(val) => match val {
                Delimiter::Paren => write!(f, "Paren"),
                Delimiter::Brace => write!(f, "Brace"),
                Delimiter::Bracket => write!(f, "Bracket"),
            },
            TokenKind::CloseDelim(val) => match val {
                Delimiter::Paren => write!(f, "Paren"),
                Delimiter::Brace => write!(f, "Brace"),
                Delimiter::Bracket => write!(f, "Bracket"),
            },
            TokenKind::Main => write!(f, "Main"),
            TokenKind::Return => write!(f, "Return"),
            TokenKind::If => write!(f, "If"),
            TokenKind::Fn => write!(f, "Fn"),
            TokenKind::I32 => write!(f, "I32"),
            TokenKind::Isize => write!(f, "Isize"),
            TokenKind::U32 => write!(f, "U32"),
            TokenKind::Usize => write!(f, "Usize"),
            TokenKind::F32 => write!(f, "F32"),
            TokenKind::AsciiChar => write!(f, "AsciiChar"),
            TokenKind::True => write!(f, "True"),
            TokenKind::False => write!(f, "False"),
            TokenKind::EOF => write!(f, "EOF"),
        }
    }
}

// // // // // // // // // // // // // // // //

#[derive(Debug)]
pub enum LitKind {
    Char(String),
    Integer(String),
    Float(String),
    String(String),
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

// // // // // // // // // // // // // // // //

#[derive(Debug)]
pub enum PermKind {
    R,
    RW,
}

impl PartialEq for PermKind {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

// // // // // // // // // // // // // // // //

#[derive(Debug)]
pub enum Delimiter {
    Paren,
    Brace,
    Bracket,
}

impl PartialEq for Delimiter {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

// // // // // // // // // // // // // // // //

#[derive(Debug)]
pub enum TokenKind {
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
    AsciiChar,
    True,
    False,
    EOF,
}

impl PartialEq for TokenKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
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

    pub(crate) fn match_keyword(lxm: &str) -> Option<TokenKind> {
        match lxm {
            "i32" => Some(TokenKind::I32),
            "u32" => Some(TokenKind::U32),
            "char" => Some(TokenKind::AsciiChar),
            "if" => Some(TokenKind::If),
            "fn" => Some(TokenKind::Fn),
            "return" => Some(TokenKind::Return),
            "isize" => Some(TokenKind::Isize),
            "usize" => Some(TokenKind::Usize),
            "f32" => Some(TokenKind::F32),
            "main" => Some(TokenKind::Main),
            "true" => Some(TokenKind::True),
            "false" => Some(TokenKind::False),
            "r" => Some(TokenKind::Permission(PermKind::R)),
            "rw" => Some(TokenKind::Permission(PermKind::RW)),
            _ => None,
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
