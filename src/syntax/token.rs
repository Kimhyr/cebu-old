pub enum Token {
    Identifier(String),
    Keyword(KeywordToken),
    Binary(BinaryToken),
    Unary(UnaryToken),
    Delimeter(DelimiterToken),
    Literal(LiteralToken),
}

pub enum LiteralToken {
    Bool(bool),
    String(String),
    Integer(String),
}

pub enum DelimiterToken {
    LCurl,
    RCurl,
    LParen,
    RParen,
    LBrack,
    RBrack,
}

pub enum UnaryToken {
    Astrix,
    Amper,
}

pub enum BinaryToken {
    Colon,
    Semicolon,
    Equal,
    Plus,
    Minus,
    Divide,
    Multiply,
}

pub enum KeywordToken {
    Dat,
    Proc,
}