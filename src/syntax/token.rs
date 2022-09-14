use std::path::Path;


#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Keyword(KeywordToken),
    Binary(BinaryToken),
    Unary(UnaryToken),
    Delimeter(DelimiterToken),
    Literal(LiteralToken),
}

#[derive(Debug)]
pub enum LiteralToken {
    Bool(bool),
    Char(char),
    String(String),
    Integer(String),
}

#[derive(Debug)]
pub enum DelimiterToken {
    LCurl,
    RCurl,
    LParen,
    RParen,
    LBrack,
    RBrack,
}

#[derive(Debug)]
pub enum UnaryToken {
    Astrix,
    Amper,
}

#[derive(Debug)]
pub enum BinaryToken {
    Colon,
    Semicolon,
    Equal,
    Plus,
    Minus,
    Divide,
    Multiply,
}

#[derive(Debug)]
pub enum KeywordToken {
    Dat,
    Proc,
}