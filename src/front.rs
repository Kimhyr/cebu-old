use crate::{syntax::{token::{Token, LiteralToken, KeywordToken, BinaryToken}, stmts::{Stmt, DatStmt}, exprs::{Expr, PathExpr, IdExpr, LiteralExpr}}, error::LexerError};

pub struct Parser<'a> {
    lexer: Lexer<'a>
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer
        }
    }

    pub fn parse(&mut self) {
        match self.lexer.get_next_token() {
            Ok(tk) => {
                match tk {
                    Token::Keyword(kw) => {
                        match kw {
                            KeywordToken::Dat => {
                                self.parse_dat_stmt();
                            },
                            KeywordToken::Proc => todo!(),
                        }
                    },
                    _ => {},
                }
            },
            Err(e) => todo!(),
        }
    }

    fn parse_dat_stmt(&mut self) {
        // let buf: (IdExpr, PathExpr, Box<dyn Expr>);
    }

    fn parse_id_expr(&mut self) -> Result<IdExpr, ()> {
        match self.lexer.get_next_token() {
            Ok(tk) => {
                match tk {
                    Token::Identifier(id) => {
                        return Ok(IdExpr::new(id));
                    },
                    _ => {}
                }
            },
            Err(e) => todo!(),
        }
        Err(())
    }

    fn parse_path_expr(&mut self) -> Result<PathExpr, ()> {
        let mut buf = Vec::<IdExpr>::new();
        loop {
            if let Ok(id) = self.parse_id_expr() {
                buf.push(id);
            }

            if let Ok(tk) = self.lexer.get_next_token() {
                if let Token::Binary(bi) = tk {
                    match bi {
                        BinaryToken::Period => { continue; },
                        _ => { return Ok(PathExpr::new(buf)); }
                    }
                }
            }
        }
    }

    fn parse_literal_expr(&mut self) -> Result<LiteralExpr, ()> {
        match self.lexer.get_next_token() {
            Ok(tk) => {
                if let Token::Literal(lit) = tk {
                    match lit {
                        _ => { return Ok(LiteralExpr::new(lit)); }
                    }
                }
            },
            Err(_) => return Err(()),
        }

        Err(())
    }

    fn parse_binary_expr(&mut self) {

    }
}

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

// pub struct Position<'a> {
//     path: &'a Path,
//     row: usize,
//     column: usize,
// }