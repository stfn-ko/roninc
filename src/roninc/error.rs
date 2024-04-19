use super::lexer::token::Span;
use core::fmt;
use std::error::Error;

// // // // // // // // // // // // // // // //  ERROR FORMATTING TRAIT
pub trait ErrorFormatting {
    fn error_print(&self);
    fn error_code(&self) -> String;
    fn error_verbose(&self) -> String;
}

// // // // // // // // // // // // // // // //  FILE HANDLER ERROR ENUM
#[derive(Debug)]
pub enum FileHandlerError {
    IO(std::io::Error), // todo -> add more
}

impl ErrorFormatting for FileHandlerError {
    fn error_code(&self) -> String {
        match self {
            FileHandlerError::IO(err) => format!("error[FHE{:02}]", err.kind() as usize + 1),
        }
    }

    fn error_verbose(&self) -> String {
        match self {
            FileHandlerError::IO(err) => format!("{}", err.to_string()),
        }
    }

    fn error_print(&self) {
        eprintln!("{} :: {}\n", self.error_code(), self.error_verbose())
    }
}

// // // // // // // // // // // // // // // //  LEXICAL ERROR ENUM
#[derive(Debug, Clone, Copy)]
pub enum LexicalError {
    IllegalCharacter,
    ExceedingLengthId,
    IncorrectSpelling,
    ExceedingLengthNum,
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
            LexicalError::ExceedingLengthId => "exceeding length of idenrifier".to_string(),
            LexicalError::IncorrectSpelling => "incorrect spelling".to_string(),
            LexicalError::ExceedingLengthNum => "exceeding length number literal".to_string(),
            LexicalError::StringMissingTrailingSign => {
                "string is missing a trailing sign `\"`".to_string()
            }
            LexicalError::CharacterMissingTrailingSign => {
                "character is missing a trailing sign `\'`".to_string()
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
    line: Vec<String>,
    span: Span,
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

// // // // // // // // // // // // // // // // RONIN ERROR STRUCT
pub type RoninErrors<ErrKind> = Vec<RoninError<ErrKind>>;

#[derive(Debug)]
pub struct RoninError<ErrKind> {
    pub kind: ErrKind,
    pub context: Option<Context>,
}

impl<ErrKind: ErrorFormatting> fmt::Display for RoninError<ErrKind> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.context {
            Some(ctx) => write!(
                f,
                " {} | {} :: {}",
                ctx.span.start.ln,
                self.kind.error_code(),
                self.kind.error_verbose()
            ),
            None => todo!(),
        }
    }
}

impl<ErrKind> RoninError<ErrKind> {
    pub fn generate(kind: ErrKind, context: Option<Context>) -> Result<(), RoninError<ErrKind>> {
        Err(RoninError { kind, context })
    }
}
