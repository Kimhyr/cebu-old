use crate::syntax::token::{Token, LiteralToken, KeywordToken, BinaryToken};

pub struct Lexer<'a> {
    src: &'a str,
    cur_pos: usize,
    cur_char: char,
    is_done: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Result<Self, LexerError> {
        Ok(Self {
            src,
            cur_pos: 0,
            cur_char: match src.chars().nth(0) {
                Some(c) => c,
                None => return Err(LexerError::EmptySource),
            },
            is_done: false
        })
    }

    pub fn get_next_token(&mut self) -> Result<Token, LexerError> {
        if self.is_done {
            return Err(LexerError::Done);
        }

        while self.cur_char.is_whitespace() {
            self.next_char();
        }

        if self.cur_char.is_alphabetic() {
            let mut t_str = String::from(self.cur_char);
            while !self.is_done {
                self.next_char();
                if !self.cur_char.is_alphabetic() && !self.cur_char.is_numeric() {
                    break;
                }
                t_str.push(self.cur_char);
            }

            match t_str.as_str() {
                "dat" => { return Ok(Token::Keyword(KeywordToken::Dat)) }
                _ => { return Ok(Token::Identifier(t_str)) }
            }
        }


        if self.cur_char.is_numeric() {
            let mut t_str = String::from(self.cur_char);
            while !self.is_done {
                self.next_char();
                if !self.cur_char.is_numeric() {
                    break;
                }
                t_str.push(self.cur_char);
            }
            return Ok(Token::Literal(LiteralToken::Integer(t_str)));
        }

        let c = self.cur_char;
        self.next_char();
        match c {
            ':' => Ok(Token::Binary(BinaryToken::Colon)),
            '=' => Ok(Token::Binary(BinaryToken::Equal)),
            '+' => Ok(Token::Binary(BinaryToken::Plus)),
            ';' => Ok(Token::Binary(BinaryToken::Semicolon)),
            _ => Err(LexerError::UnkownToken),
        }
    }

    fn next_char(&mut self) {
        self.cur_pos += 1;
        match self.src.chars().nth(self.cur_pos) {
            Some(c) => {
                self.cur_char = c;
            },
            None => {
                self.is_done = true;
            }
        }
    }
}

#[derive(Debug)]
pub enum LexerError {
    Done,               // The lexer has no more characters to scan.
    EmptySource,        // The source has no characters to initialize the lexer.
    UnkownToken,        // The scanned characters could not parse into a known token.
}

// pub struct Position<'a> {
//     path: &'a Path,
//     row: usize,
//     column: usize,
// }