use crate::roninc::token::{Delimiter, LitKind, LnCol, Token, TokenKind};
use std::{fmt::Error, fs, iter::Peekable, str::Chars};

use super::token::PermKind;

pub(crate) struct Lexer<'a> {
    pub tokens: &'a mut Vec<Token>,
    pub iter: Peekable<Chars<'a>>,
    pub pos: LnCol,
}

pub fn emit_tokens(path: &str) -> Result<Vec<Token>, Error> {
    let input: String = match fs::read_to_string(path) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Error: {err}");
            return Err(Error);
        }
    };

    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(&input, &mut tokens);

    while let Some(ch) = lexer.iter.peek() {
        match ch {
            ch if ch.is_whitespace() => lexer.skip_whitespace(),
            '#' => lexer.skip_comments(),
            _ => lexer.get_tokens(),
        }
    }

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

        match Self::is_keyword(&lxm) {
            Some(tk) => self
                .tokens
                .push(Token::new(tk, self.pos.update(0, lxm.len()))),
            None => self.tokens.push(Token::new(
                TokenKind::Ident(lxm.clone()),
                self.pos.update(0, lxm.len()),
            )),
        }
    }

    fn get_punctuation(&mut self) {
        let pt: TokenKind = match self.iter.next() {
            Some(ch) => match ch {
                ';' => TokenKind::Semi,
                ':' => TokenKind::Colon,
                '{' => TokenKind::OpenDelim(Delimiter::Brace),
                '(' => TokenKind::OpenDelim(Delimiter::Paren),
                ')' => TokenKind::CloseDelim(Delimiter::Paren),
                '}' => TokenKind::CloseDelim(Delimiter::Brace),
                '[' => TokenKind::OpenDelim(Delimiter::Bracket),
                ']' => TokenKind::CloseDelim(Delimiter::Bracket),
                '=' => TokenKind::Eq,
                '-' => TokenKind::Minus,
                '+' => TokenKind::Plus,
                '/' => TokenKind::FwSlash,
                '*' => TokenKind::Star,
                '.' => TokenKind::Dot,
                ',' => TokenKind::Comma,
                '\'' => TokenKind::SingleQuote,
                '\"' => TokenKind::DoubleQuote,
                '!' => TokenKind::Not,
                '&' => TokenKind::And,
                '|' => TokenKind::Or,
                '>' => TokenKind::Gt,
                '<' => TokenKind::Lt,
                '#' => TokenKind::Hashtag,
                '%' => TokenKind::Percent,
                '\\' => TokenKind::BSlash,
                '@' => TokenKind::At,
                _ => {
                    eprintln!(
                        "Unknown character encountered at ln: {}, col: {}",
                        self.pos.ln, self.pos.col
                    );

                    TokenKind::Undef(ch)
                }
            },
            None => TokenKind::EOF,
        };

        self.tokens.push(Token::new(pt, self.pos.update(0, 1)));
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
            true => self.tokens.push(Token::new(
                TokenKind::Literal(LitKind::Float(lxm.clone())),
                self.pos.update(0, lxm.len()),
            )),
            false => self.tokens.push(Token::new(
                TokenKind::Literal(LitKind::Integer(lxm.clone())),
                self.pos.update(0, lxm.len()),
            )),
        }
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
}
