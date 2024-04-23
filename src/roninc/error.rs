use super::lexer::token::Span;
use core::fmt;

// // // // // // // // // // // // // // // //  ERROR FORMATTING TRAIT
pub trait ErrorFormatting {
    fn error_print(&self);
    fn error_code(&self) -> String;
    fn error_verbose(&self) -> String;
}

// // // // // // // // // // // // // // // //  LEXICAL ERROR ENUM
#[derive(Debug, Clone, Copy)]
pub enum LexicalError {
    IllegalCharacter,
    ExceedingLengthId,
    StringMissingTrailingSign,
    CharacterMissingTrailingSign,
    // todo -> add more
}

impl ErrorFormatting for LexicalError {
    fn error_code(&self) -> String {
        format!("error[LE{:02}]", *self as usize + 1)
    }

    fn error_verbose(&self) -> String {
        match self {
            LexicalError::IllegalCharacter => "llegal character".to_string(),
            LexicalError::ExceedingLengthId => "exceeding length of identifier".to_string(),
            LexicalError::StringMissingTrailingSign => {
                "string literal is missing a trailing sign `\"`".to_string()
            }
            LexicalError::CharacterMissingTrailingSign => {
                "character literal is missing a trailing sign `\'`".to_string()
            }
        }
    }

    fn error_print(&self) {
        eprintln!("{} :: {}\n", self.error_code(), self.error_verbose())
    }
}

// // // // // // // // // // // // // // // //  SYNTAX ERROR ENUM
#[derive(Debug, Clone, Copy)]
pub enum SyntaxError {
    TypeMismatch,
    SemiMissing,
    SemiOmmited,
    // todo -> add more
}

impl ErrorFormatting for SyntaxError {
    fn error_code(&self) -> String {
        format!("error[SE{:02}]", *self as usize + 1)
    }

    fn error_verbose(&self) -> String {
        match self {
            SyntaxError::TypeMismatch => "type mismatch".to_string(),
            SyntaxError::SemiMissing => "character `;` missing".to_string(),
            SyntaxError::SemiOmmited => "character `;` ommited".to_string(),
        }
    }

    fn error_print(&self) {
        eprintln!("{} :: {}\n", self.error_code(), self.error_verbose())
    }
}

// // // // // // // // // // // // // // // //  CONTEXT STRUCT

#[derive(Debug)]
pub struct Context {
    filename: String,
    text: String,
    span: Span,
}

impl Context {
    pub fn new(filename: &str, text: String, span: Span) -> Self {
        Context {
            filename: filename.to_string(),
            text,
            span,
        }
    }
}

// // // // // // // // // // // // // // // // RONIN ERROR STRUCT
pub type RoninErrors<ErrKind> = Vec<RoninError<ErrKind>>;

#[derive(Debug)]
pub struct RoninError<ErrKind> {
    pub kind: ErrKind,
    context: Option<Context>,
}

impl<ErrKind: ErrorFormatting> fmt::Display for RoninError<ErrKind> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} :: {}",
            self.kind.error_code(),
            self.kind.error_verbose()
        )
    }
}

impl<ErrKind> RoninError<ErrKind> {
    pub fn generate(kind: ErrKind) -> RoninError<ErrKind> {
        RoninError {
            kind,
            context: None,
        }
    }

    pub fn attach(self, filename: String, text: String, span: Span) -> RoninError<ErrKind> {
        RoninError {
            kind: self.kind,
            context: Some(Context {
                filename,
                text,
                span,
            }),
        }
    }
}

// Some(Context::new(
//     &self.buffer.filename,
//     self.buffer.get_line(self.pos.ln),
//     Span::new(self.pos, self.pos.add(0, len_ct as usize)),
// )
