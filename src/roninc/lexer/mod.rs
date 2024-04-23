pub mod file_handler;
pub mod token;

use std::process::exit;

use self::token::Span;

use super::error::{LexicalError, RoninError, RoninErrors};
use file_handler::*;
use token::{LitKind, LnCol, Token, TokenKind, Tokens};

// // // // // // // // // // // // // // // //

pub fn emit_tokens(path: &str) -> Result<Tokens, RoninErrors<LexicalError>> {
    let mut lxr_err = RoninErrors::<LexicalError>::new();

    let buffer: Buffer = match load_file_to_buffer(path) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("{err}");
            exit(0);
        }
    };

    let mut lexer = Lexer::new(buffer);

    while let Some(ch) = lexer.buffer.peek() {
        if ch.is_whitespace() {
            lexer.skip_whitespace();
            continue;
        }

        match ch {
            '#' => lexer.skip_comments(),
            '\"' => match lexer.get_string() {
                Ok(_) => {}
                Err(e) => lxr_err.push(e),
            },
            '\'' => match lexer.get_char() {
                Ok(_) => {}
                Err(e) => lxr_err.push(e),
            },
            _ => match lexer.get_tokens() {
                Ok(_) => {}
                Err(e) => lxr_err.push(e),
            },
        }
    }

    lexer.push_eof();

    match lxr_err.is_empty() {
        true => Ok(lexer.tokens),
        false => Err(lxr_err),
    }
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

    fn get_tokens(&mut self) -> Result<(), RoninError<LexicalError>> {
        if let Some(ch) = self.buffer.peek() {
            match ch {
                '_' | 'a'..='z' | 'A'..='Z' => return self.get_id(),
                '0'..='9' => return self.get_nums(),
                _ => return self.get_punctuation(),
            }
        } else {
            Ok(())
        }
    }

    fn get_id(&mut self) -> Result<(), RoninError<LexicalError>> {
        let mut len_ct: u8 = 0;
        let mut lxm = String::new();

        while let Some(ch) = self.buffer.peek() {
            match !ch.is_alphanumeric() && *ch != '_' {
                true => break,
                false => {
                    len_ct += 1;

                    lxm.push(*ch);
                    self.buffer.next();
                }
            }
        }

        if len_ct > 30 {
            return Err(
                RoninError::generate(LexicalError::ExceedingLengthId).attach(
                    self.buffer.filename.to_owned(),
                    self.buffer.get_line(self.pos.ln),
                    Span::new(self.pos, self.pos.add(0, len_ct as usize)),
                ),
            );
        } else {
            match TokenKind::match_keyword(&lxm) {
                Some(tk) => return self.handle_permission(tk, &lxm),
                None => self.t_push(TokenKind::Ident(lxm.clone()), 0, lxm.len()),
            }

            Ok(())
        }
    }

    fn handle_permission(
        &mut self,
        token_kind: TokenKind,
        lxm: &str,
    ) -> Result<(), RoninError<LexicalError>> {
        match token_kind.is_permission() {
            true => {
                let last_token = match self.tokens.last_mut() {
                    Some(t) => t,
                    None => panic!("could not access previous token"),
                };

                match last_token.kind.eq(&TokenKind::Div) && last_token.pos.col == self.pos.col - 1
                {
                    true => return Ok(last_token.kind = token_kind),
                    false => {
                        return Ok(self.t_push(TokenKind::Ident(lxm.to_owned()), 0, lxm.len()))
                    }
                }
            }
            false => Ok(self.t_push(token_kind, 0, lxm.len())),
        }
    }

    fn get_punctuation(&mut self) -> Result<(), RoninError<LexicalError>> {
        let kind: TokenKind = match self.buffer.next() {
            Some(ch) => match TokenKind::match_punctuation(&ch) {
                Some(res) => res,
                None => return Err(RoninError::generate(LexicalError::IllegalCharacter)),
            },
            None => return Ok(self.t_push(TokenKind::EOF, 0, 1)),
        };

        let ch = match self.buffer.peek() {
            Some(ch) => ch,
            None => return Ok(self.t_push(kind, 0, 1)),
        };

        Ok(match kind.get_combo(&ch) {
            Some(res) => {
                self.t_push(res, 0, 2);
                self.buffer.next();
            }
            None => self.t_push(kind, 0, 1),
        })
    }

    fn get_nums(&mut self) -> Result<(), RoninError<LexicalError>> {
        let mut dot: bool = false;
        let mut lxm = String::new();

        while let Some(&ch) = self.buffer.peek() {
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
                ' ' | ';' => break,
                _ => return Err(RoninError::generate(LexicalError::IllegalCharacter)),
            }
        }

        Ok(match dot {
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
        })
    }

    fn get_string(&mut self) -> Result<(), RoninError<LexicalError>> {
        self.buffer.next();
        let mut esc_flag: bool = false;
        let mut lxm: String = String::new();
        let (mut ln, mut col) = (0, 1);

        loop {
            col += 1;

            match self.buffer.next() {
                Some(&ch) => {
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
                    return Err(RoninError::generate(
                        LexicalError::StringMissingTrailingSign,
                    ))
                }
            }
        }

        Ok(self.t_push(TokenKind::Literal(LitKind::String(lxm)), ln, col))
    }

    fn get_char(&mut self) -> Result<(), RoninError<LexicalError>> {
        self.buffer.next();
        let mut esc_flag: bool = false;
        let mut lxm: String = String::new();

        loop {
            match self.buffer.next() {
                Some(&ch) => {
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
                None => {
                    return Err(RoninError::generate(
                        LexicalError::CharacterMissingTrailingSign,
                    ))
                }
            }
        }

        Ok(self.t_push(
            TokenKind::Literal(LitKind::Char(lxm.clone())),
            0,
            lxm.len() + 2,
        ))
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.buffer.peek() {
            if !ch.is_whitespace() {
                break;
            }

            if ch == '\n' {
                self.buffer.notfify_new_line();
                self.pos.ln += 1;
                self.pos.col = 1;
            } else {
                self.pos.col += 1;
            }

            self.buffer.next();
        }
    }

    fn skip_comments(&mut self) {
        while let Some(&ch) = self.buffer.next() {
            if ch == '\n' {
                self.buffer.notfify_new_line();
                break;
            }
        }

        self.pos.ln += 1;
    }

    fn t_push(&mut self, tk: TokenKind, ln: usize, col: usize) {
        self.tokens.push(Token::new(tk, self.pos.update(ln, col)));
    }

    fn push_eof(&mut self) {
        self.t_push(TokenKind::EOF, 0, 1)
    }
}
