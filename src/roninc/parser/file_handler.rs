use std::{fs, io};

pub struct Buffer {
    pub filename: String,
    pub symbols: Vec<char>,
    pub cursor: usize,
    line_start: Vec<usize>,
}

impl Buffer {
    fn new(filename: &str, symbols: String) -> Self {
        Buffer {
            filename: filename.to_string(),
            symbols: symbols.chars().collect(),
            cursor: 0,
            line_start: vec![0],
        }
    }

    pub fn get_line(&self, index: usize) -> String {
        match self.line_start.get(index) {
            Some(&idx) => self
                .symbols
                .iter()
                .skip(idx)
                .take_while(|&&ch| ch != '\n')
                .collect(),

            None => todo!(),
        }
    }

    pub fn notfify_new_line(&mut self) {
        self.line_start.push(self.cursor + 1);
    }

    pub fn next(&mut self) -> Option<&char> {
        self.cursor += 1;
        self.symbols.get(self.cursor - 1)
    }

    pub fn peek(&self) -> Option<&char> {
        self.symbols.get(self.cursor)
    }
}

pub fn load_file_to_buffer(path: &str) -> Result<Buffer, io::Error> {
    return match fs::read_to_string(path) {
        Ok(res) => Ok(Buffer::new(path, res)),
        Err(err) => Err(err),
    };
}
