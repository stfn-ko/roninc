use crate::roninc::token::{LitKind, LnCol, Token, TokenKind};
use std::{fmt::Error, fs, iter::Peekable, ops::Index, str::Chars};

pub(crate) struct Lexer<'a> {
    pub tokens: &'a mut Vec<Token>,
    pub iter: Peekable<Chars<'a>>,
    pub pos: LnCol,
    path: &'a str,
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
    let mut lexer = Lexer::new(path, &input, &mut tokens);

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
    fn new(path: &'a str, input: &'a str, tokens: &'a mut Vec<Token>) -> Self {
        Self {
            tokens,
            iter: input.chars().peekable(),
            pos: LnCol::new(1, 1),
            path,
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
        let mut esc_flag = false;
        let mut lxm: String = String::new();
        let mut span: LnCol = LnCol::new(0, 0);

        loop {
            match self.iter.next() {
                Some(ch) => {
                    if esc_flag == true {
                        match ch {
                            'n' | 't' | '0' | 'r' | '\'' | '\"' | '\\' => {}
                            _ => eprintln!("ronin::lexer >> invalid character escape"),
                        }
                    }
                    if ch == '"' && esc_flag == false {
                        break;
                    } else if ch == '\r' {
                        continue;
                    } else if ch == '\n' {
                        println!("\\n enocuntered");
                        lxm.push('\\');
                        lxm.push('n');

                        span.ln += 1;
                        span.col = 0;
                        continue;
                    } else if ch == '\t' {
                        println!("\\t enocuntered");
                        lxm.push(' ');
                        lxm.push(' ');
                        lxm.push(' ');
                        lxm.push(' ');

                        span.col += 4;
                        continue;
                    }

                    esc_flag = false;
                    if ch == '\\' {
                        esc_flag = true;
                    }

                    span.col += 1;
                    lxm.push(ch);
                }
                None => {
                    eprintln!("ronin::lexer >> unexpected EOF");
                    // --> src\roninc\lexer.rs:194:101 #aquamarine
                    eprintln!(
                        "ronin::lexer >> string is missing a trailing character '\"' {}, {}",
                        self.pos.ln, self.pos.col
                    );
                    return;
                }
            }
        }

        println!("{lxm}");

        self.t_push(
            TokenKind::Literal(LitKind::Integer(lxm.clone())),
            span.ln,
            span.col,
        )
    }

    fn get_char(&mut self) /* -> Result<char, Error> */
    {
        self.iter.next();
        let mut esc_flag = false;
        let mut lxm: [char; 2] = ['\0', '\0'];

        match self.iter.next() {
            Some(ch) if ch == '\'' => {
                eprintln!("ronin::lexer >> empty char literal");
            }
            Some(ch) => {
                if ch == '\\' {
                    esc_flag = true;
                }

                lxm[0] = ch;
            }
            None => {
                eprintln!("unexpected EOF");
                return;
            }
        }

        if esc_flag == true {
            match self.iter.next() {
                Some(ch) => {
                    lxm[1] = ch;
                }
                None => {
                    eprintln!("ronin::lexer >> unexpected EOF");
                    return;
                }
            }
        }

        match self.iter.peek() {
            Some(&ch) => {
                if ch != '\'' {
                    eprintln!("ronin::lexer >> char literal is missing a closing quote");
                }
            }
            None => {
                eprintln!("ronin::lexer >> unexpected EOF");
                return;
            }
        }

        self.t_push(
            TokenKind::Literal(LitKind::Char(if esc_flag == true {
                lxm.iter().collect()
            } else {
                lxm.index(0).to_string()
            })),
            0,
            lxm.len(),
        );

        self.iter.next();
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
