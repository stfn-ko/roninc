pub struct Errors<'a> {
    path: &'a str,
    buffer: Vec<Error>
}

pub struct Error {
    err_t: ErrorT,
    rec: bool, //  recoverable
}
pub enum ErrorT {
    Lexical(String),
}

impl Error {
    pub(crate) fn print(&self) {
        // --> src\roninc\lexer.rs:194:101 #aquamarine
    }
}

impl<'a> Errors<'a> {
    /// Creates a new [`Errors`].
    pub(crate) fn new(path: &'a str) -> Self {
        let mut buffer = Vec::new();
        Errors { path, buffer }
    }

    pub(crate) fn new_err(err_t: ErrorT, rec: bool) -> Error {
        Error { err_t, rec}
    }
    
    pub(crate) fn push_err(&mut self, err_t: ErrorT, rec: bool) {
        self.buffer.push(Error { err_t, rec});
    }
}