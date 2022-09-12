use std::{str::EncodeUtf16, fmt::Error};

use crate::syntax::token::{Token, KeywordToken, LiteralToken, BinaryToken};

pub struct Lexer<'a> {
    src: &'a str,
    cur_pos: usize,
    cur_char: char,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Result<Self, LexerError> {
        Ok(Self {
            src,
            cur_pos: src.len(),
            cur_char: match src.chars().nth(0) {
                Some(c) => c,
                None => return Err(LexerError::Nothing),
            }
        })
    }

    fn next(&mut self) -> Result<Token, LexerError> {
        while self.cur_char.is_whitespace() {
            if !self.advance() {
                return Err(LexerError::End);
            }
        }

        if self.cur_char.is_alphabetic() {
            let mut t_str = String::new();
            while self.advance() &&
                self.cur_char.is_alphabetic() ||
                self.cur_char.is_numeric() {
                t_str.push(self.cur_char);
            }

            match t_str.as_str() {
                "dat" => {
                    return Ok(Token::Keyword(
                        KeywordToken::Dat
                    ))
                },
                "proc" => {
                    return Ok(Token::Keyword(
                        KeywordToken::Proc
                    ))
                },
                _ => {
                    return Ok(Token::Identifier(t_str))
                }
            }
        }

        if self.cur_char.is_numeric() {
            let mut t_str = String::new();
            while self.advance() &&
                self.cur_char.is_numeric() {
                t_str.push(self.cur_char);
            }

            return Ok(Token::Literal(
                LiteralToken::Integer(t_str)
            ));
        }

        match self.cur_char {
            ':' => {
                return Ok(Token::Binary(
                    BinaryToken::Colon
                ))
            }
            ';' => {
                return Ok(Token::Binary(
                    BinaryToken::Semicolon
                ))
            }
            '=' => {
                return Ok(Token::Binary(
                    BinaryToken::Equal
                ))
            }
            '+' => {
                return Ok(Token::Binary(
                    BinaryToken::Plus
                ))
            }
            _ => { return Err(LexerError::Unknown); }
        }
    }

    fn advance(&mut self) -> bool {
        self.cur_pos += 1;
        match self.src.chars().nth(self.cur_pos) {
            Some(c) => self.cur_char = c,
            None => return false,
        }
        true
    }
}

pub enum LexerError {
    Nothing,
    End,
    Unknown,
}