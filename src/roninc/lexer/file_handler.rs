use std::{fs, io};

pub struct Buffer {
    pub address: String,
    pub input: Vec<char>,
    pub cursor: usize,
}

impl Buffer {
    fn new(address: &str, input: String) -> Self {
        Buffer {
            address: address.to_string(),
            input: input.chars().collect(),
            cursor: 0,
        }
    }

    pub fn next(&mut self) -> Option<char> {
        self.cursor += 1;
        self.input.get(self.cursor - 1).copied()
    }

    pub fn peek(&self) -> Option<char> {
        self.input.get(self.cursor).copied()
    }
}

pub fn load_file_to_buffer(path: &str) -> Result<Buffer, io::Error> {
    return match fs::read_to_string(path) {
        Ok(res) => Ok(Buffer::new(path, res)),
        Err(err) => Err(err),
    };
}
