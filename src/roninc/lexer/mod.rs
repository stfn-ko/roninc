mod error;
mod file_handler;
mod token;

use error::{LexicalError, RoninError, RoninErrors};
use file_handler::*;
use token::{LitKind, LnCol, Token, TokenKind, Tokens};

// // // // // // // // // // // // // // // //

pub fn emit_tokens(path: &str) -> Result<Tokens, RoninErrors<LexicalError>> {
    let mut lx_err = Vec::new();

    let buffer: Buffer = match load_file_to_buffer(path) {
        Ok(res) => res,
        Err(e) => panic!("{:?}", e),
    };

    let mut lexer = Lexer::new(buffer);

    while let Some(ch) = lexer.buffer.peek() {
        match ch {
            ch if ch.is_whitespace() => match lexer.skip_whitespace() {
                Ok(_) => continue,
                Err(e) => lx_err.push(e),
            },
            '#' => match lexer.skip_comments() {
                Ok(_) => continue,
                Err(e) => lx_err.push(e),
            },
            '\"' => match lexer.get_string() {
                Ok(_) => continue,
                Err(e) => lx_err.push(e),
            },
            '\'' => match lexer.get_char() {
                Ok(_) => continue,
                Err(e) => lx_err.push(e),
            },
            _ => match lexer.get_tokens() {
                Ok(_) => continue,
                Err(e) => lx_err.push(e),
            },
        }
    }

    lexer.end();

    Ok(lexer.tokens)
}

// // // // // // // // // // // // // // // //

pub(crate) struct Lexer {
    tokens: Tokens,
    buffer: Buffer,
    pos: LnCol,
}

impl Lexer {
    fn new(buffer: Buffer) -> Self {
        Self {
            buffer,
            tokens: Tokens::new(),
            pos: LnCol::new(1, 1),
        }
    }

    fn get_tokens(&mut self) -> Result<(), LexicalError> {
        if let Some(ch) = self.buffer.peek() {
            match ch {
                '_' | 'a'..='z' | 'A'..='Z' => return self.get_id(),
                '0'..='9' => return self.get_nums(),
                _ => return self.get_punctuation(),
            }
        } else {
            RoninError::generate(LexicalError::IllegalCharacter, None)
        }
    }

    fn get_id(&mut self) -> Result<(), LexicalError> {
        let mut lxm = String::new();

        while let Some(ch) = self.buffer.peek() {
            match !ch.is_alphanumeric() && ch != '_' {
                true => break,
                false => {
                    lxm.push(ch);
                    self.buffer.next();
                }
            }
        }

        match TokenKind::match_keyword(&lxm) {
            Some(tk) => return self.handle_permission(tk, &lxm),
            None => Ok(self.t_push(TokenKind::Ident(lxm.clone()), 0, lxm.len())),
        }
    }

    fn handle_permission(&self, token_kind: TokenKind, lxm: &str) -> Result<(), LexicalError> {
        match token_kind.is_permission() {
            true => {
                let last_token = match self.tokens.last_mut() {
                    Some(t) => t,
                    None => panic!("could not access previous token"),
                };
                    
                match last_token.kind.eq(&TokenKind::Div) && last_token.pos.col == self.pos.col - 1 {
                    true => return Ok(last_token.kind = token_kind),
                    false => return Ok(self.t_push(TokenKind::Ident(lxm.to_owned()), 0, lxm.len()))
                }
            }
            false => Ok(self.t_push(token_kind, 0, lxm.len())),
        }
    }

    fn get_punctuation(&mut self) -> Result<(), LexicalError> {
        let kind: TokenKind = match self.buffer.next() {
            Some(ch) => match TokenKind::match_punctuation(&ch) {
                Some(res) => res,
                None => return,
            },
            None => {
                self.t_push(TokenKind::EOF, 0, 1);
                return;
            }
        };

        let ch = match self.buffer.peek() {
            Some(ch) => ch,
            None => {
                self.t_push(kind, 0, 1);
                return;
            }
        };

        match kind.get_combo(&ch) {
            Some(res) => {
                self.t_push(res, 0, 2);
                self.buffer.next();
            }
            None => self.t_push(kind, 0, 1),
        }
    }

    fn get_nums(&mut self) -> Result<(), LexicalError> {
        let mut dot: bool = false;
        let mut lxm = String::new();

        while let Some(ch) = self.buffer.peek() {
            match ch {
                '0'..='9' => {
                    lxm.push(ch);
                    self.buffer.next();
                }
                '.' if !dot => {
                    dot = true;
                    lxm.push(ch);
                    self.buffer.next();
                }
                _ => break,
            }
        }

        match dot {
            true => self.t_push(
                TokenKind::Literal(LitKind::Float(lxm.clone())),
                0,
                lxm.len(),
            ),
            false => self.t_push(
                TokenKind::Literal(LitKind::Integer(lxm.clone())),
                0,
                lxm.len(),
            ),
        }
    }

    fn get_string(&mut self) -> Result<(), LexicalError> {
        self.buffer.next();
        let mut esc_flag: bool = false;
        let mut lxm: String = String::new();
        let (mut ln, mut col) = (0, 1);

        loop {
            col += 1;

            match self.buffer.next() {
                Some(ch) => {
                    if ch == '\"' && esc_flag == false {
                        break;
                    }

                    if ch == '\\' && esc_flag == false {
                        esc_flag = true
                    } else if esc_flag == true {
                        esc_flag = false
                    }

                    if ch == '\n' {
                        ln += 1;
                        col = 1;
                    } else if ch == '\t' {
                        col += 4
                    }

                    lxm.push(ch);
                }
                None => {
                    panic!("Syntax Error >> string literal is missing a `\"` trailing symbol")
                }
            }
        }

        self.t_push(TokenKind::Literal(LitKind::String(lxm)), ln, col)
    }

    fn get_char(&mut self) -> Result<(), LexicalError> {
        self.buffer.next();
        let mut esc_flag: bool = false;
        let mut lxm: String = String::new();

        loop {
            match self.buffer.next() {
                Some(ch) => {
                    if ch == '\'' && esc_flag == false {
                        break;
                    }

                    if ch == '\\' && esc_flag == false {
                        esc_flag = true
                    } else if esc_flag == true {
                        esc_flag = false
                    }

                    lxm.push(ch);
                }
                None => return Err(CompilerError::SYNTX(SyntaxError::UnclosedCharLiteral)),
            }
        }

        Ok(self.t_push(
            TokenKind::Literal(LitKind::Char(lxm.clone())),
            0,
            lxm.len() + 2,
        ))
    }

    fn skip_whitespace(&mut self) -> Result<(), LexicalError> {
        while let Some(ch) = self.buffer.peek() {
            if !ch.is_whitespace() {
                break;
            }

            if ch == '\n' {
                self.pos.ln += 1;
                self.pos.col = 1;
            } else {
                self.pos.col += 1;
            }

            self.buffer.next();
        }
    }

    fn skip_comments(&mut self) -> Result<(), LexicalError> {
        while let Some(ch) = self.buffer.next() {
            if ch == '\n' {
                break;
            }
        }

        self.pos.ln += 1;
    }

    fn t_push(&mut self, tk: TokenKind, ln: usize, col: usize) {
        self.tokens.push(Token::new(tk, self.pos.update(ln, col)));
    }

    fn end(&mut self) {
        self.t_push(TokenKind::EOF, 0, 1)
    }
}
