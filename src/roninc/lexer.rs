use crate::roninc::token::{LitKind, LnCol, PermKind, Token, TokenKind};
use std::{fmt::Error, fs, iter::Peekable, str::Chars};

pub(crate) struct Lexer<'a> {
    pub tokens: &'a mut Vec<Token>,
    pub iter: Peekable<Chars<'a>>,
    pub pos: LnCol,
}

pub fn emit_tokens(path: &str) -> Result<Vec<Token>, Error> {
    let input: String = match fs::read_to_string(path) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("lexer::emit_tokens >> {err}");
            return Err(Error);
        }
    };

    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(&input, &mut tokens);

    while let Some(ch) = lexer.iter.peek() {
        match ch {
            ch if ch.is_whitespace() => lexer.skip_whitespace(),
            '#' => lexer.skip_comments(),
            '\"' => lexer.get_string(),
            '\'' => lexer.get_char(),
            _ => lexer.get_tokens(),
        }
    }

    lexer.t_push(TokenKind::EOF, 0, 1);

    Ok(tokens)
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str, tokens: &'a mut Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            iter: input.chars().peekable(),
            pos: LnCol::new(1, 1),
        }
    }

    fn get_tokens(&mut self) {
        if let Some(&ch) = self.iter.peek() {
            match ch {
                '_' | 'a'..='z' | 'A'..='Z' => self.get_id(),
                '0'..='9' => self.get_nums(),
                _ => self.get_punctuation(),
            }
        }
    }

    fn get_id(&mut self) {
        let mut lxm = String::new();

        while let Some(&ch) = self.iter.peek() {
            if !ch.is_alphanumeric() && ch != '_' {
                break;
            }

            lxm.push(ch);
            self.iter.next();
        }

        match TokenKind::match_keyword(&lxm) {
            Some(tk) if tk.is_permission() => {
                if let Some(lt) = self.tokens.last_mut() {
                    if lt.kind.eq(&TokenKind::Div) && lt.pos.col == self.pos.col - 1 {
                        lt.kind = tk;
                    } else {
                        self.t_push(TokenKind::Ident(lxm.clone()), 0, lxm.len())
                    }
                }
            }
            Some(tk) => self.t_push(tk, 0, lxm.len()),
            None => self.t_push(TokenKind::Ident(lxm.clone()), 0, lxm.len()),
        }
    }

    fn get_punctuation(&mut self) {
        let kind: TokenKind = match self.iter.next() {
            Some(ch) => match TokenKind::match_punctuation(&ch) {
                Some(res) => res,
                None => todo!(),
            },
            None => {
                self.t_push(TokenKind::EOF, 0, 1);
                return;
            }
        };

        let ch = match self.iter.peek() {
            Some(ch) => ch,
            None => {
                self.t_push(kind, 0, 1);
                return;
            }
        };

        match kind.get_combo(ch) {
            Some(res) => {
                self.t_push(res, 0, 2);
                self.iter.next();
            }
            None => self.t_push(kind, 0, 1),
        }
    }

    fn is_keyword(lxm: &str) -> Option<TokenKind> {
        match lxm {
            "i32" => Some(TokenKind::I32),
            "u32" => Some(TokenKind::U32),
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

    fn get_nums(&mut self) {
        let mut dot: bool = false;
        let mut lxm = String::new();

        while let Some(&ch) = self.iter.peek() {
            match ch {
                '0'..='9' => {
                    lxm.push(ch);
                    self.iter.next();
                }
                '.' if !dot => {
                    dot = true;
                    lxm.push(ch);
                    self.iter.next();
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

    fn get_string(&mut self) /* -> Result<char, Error> */
    {
        self.iter.next();
        let mut esc_flag: bool = false;
        let mut lxm: String = String::new();
        let (mut ln, mut col) = (0, 1);

        loop {
            col += 1;

            match self.iter.next() {
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
                    eprintln!("Syntax Error >> string literal is missing a `\"` trailing symbol");
                    panic!();
                }
            }
        }

        self.t_push(TokenKind::Literal(LitKind::String(lxm)), ln, col)
    }

    fn get_char(&mut self) /* -> Result<char, Error> */
    {
        self.iter.next();
        let mut esc_flag: bool = false;
        let mut lxm: String = String::new();

        loop {
            match self.iter.next() {
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
                None => {
                    eprintln!("Syntax Error >> character literal is missing a `'` trailing symbol");
                    panic!();
                }
            }
        }

        self.t_push(
            TokenKind::Literal(LitKind::Char(lxm.clone())),
            0,
            lxm.len() + 2,
        )
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.iter.peek() {
            if !ch.is_whitespace() {
                break;
            }

            if ch == '\n' {
                self.pos.ln += 1;
                self.pos.col = 1;
            } else {
                self.pos.col += 1;
            }

            self.iter.next();
        }
    }

    fn skip_comments(&mut self) {
        while let Some(ch) = self.iter.next() {
            if ch == '\n' {
                break;
            }
        }

        self.pos.ln += 1;
    }

    fn t_push(&mut self, tk: TokenKind, ln: usize, col: usize) {
        self.tokens.push(Token::new(tk, self.pos.update(ln, col)));
    }
}
