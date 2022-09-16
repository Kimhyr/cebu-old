#[derive(Debug)]
pub enum LexerError {
    Done,               // The lexer has no more characters to scan.
    EmptySource,        // The source has no characters to initialize the lexer.
    UnkownToken,        // The scanned characters could not parse into a known token.
}