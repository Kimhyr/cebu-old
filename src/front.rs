use crate::front::NextCharErr::EOF;

pub struct Parser<'a, S: Stmt, E: Expr> {
    src: &'a str,
    src_len: usize,
    cur_pos: usize,
    is_done: bool,
}

impl<'a, S: Stmt, E: Expr> Parser<'a, S, E> {
    pub fn new(src: &'a str) -> Self <'a> {
        Self {
            src,
            src_len: src.len(),
            cur_pos: 0,
            is_done: false,
        }
    }

    pub fn parse_stmt(&mut self) -> Result<S, ParseStmtErr> {
        self.skip_whitespace();

        match self.scan_word() {
            Ok(word) => {
                match word {
                    "dat" =>  {
                        self.parse_dat_stmt();
                    }
                    &_ => {
                        Err(ParseStmtErr::UnknownSym)
                    }
                }
            }
            Err(_) => { todo!() }
        }

        Err(ParseStmtErr::None)
    }

    fn parse_dat_stmt(&mut self) -> Result<DatDefStmt, ()> {
        let id: &'a str;
        match self.scan_word() {
            Ok(word) => {
                id = word;
            }
            Err(_) => { todo!() }
        }

        Err(())
    }

    fn parse_path_expr() -> Result<(), ()> {}

    fn scan_word(&mut self) -> Result<&'a str, ()> {
        let mut word = String::new();
        loop {
            match self.next_char() {
                Ok(val) => {
                    if val.is_whitespace() {
                        break;
                    }

                    word.push(val);
                }
                Err(_) => Err(())
            }
        }

        Ok(word.as_str())
    }


    fn skip_whitespace(&'a mut self) {
        loop {
            match self.next_char() {
                Ok(val) => {
                    if !val.is_whitespace() {
                        break;
                    }
                }
                Err(_) => break
            }
        }
    }

    fn next_char(&mut self) -> Result<char, NextCharErr> {
        if self.cur_pos == self.src_len - 1 {
            self.is_done = true;
            Err(EOF)
        }

        self.cur_pos += 1;
        Ok(self.src.chars()[self.cur_pos])
    }
}

pub enum ParseStmtErr {
    None,
    UnknownSym,
}

pub enum NextCharErr {
    EOF,
}