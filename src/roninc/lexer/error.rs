use super::token::LnCol;

pub enum LexicalError {
    IllegalCharacter,
    ExceedingLengthId,
    IncorrectSpelling,
    ExceedingLengthNum,
    StringMissingTrailingSign,
    CharacterMissingTrailingSign,
}

pub struct Span {
    start: LnCol,
    end: LnCol,
}

pub struct Context {
    filename: String,
    line: Vec<String>,
    span: Span,
}

pub type RoninErrors<ErrKind> = Vec<RoninError<ErrKind>>;
pub struct RoninError<ErrKind> {
    pub kind: ErrKind,
    pub context: Option<Context>,
}

impl Context {
    pub fn new(filename: String, line: Vec<String>, span: Span) -> Self {
        Context {
            filename,
            line,
            span,
        }
    }
}

impl<ErrKind> RoninError<ErrKind> {
    pub fn generate(kind: ErrKind, context: Option<Context>) -> Result<(), ErrKind> {
        Err(kind)
    }
}
