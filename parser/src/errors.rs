#[derive(Debug)]
pub struct ParserError {
    pub char: usize,
    pub error_type: ParserErrorType,
}

#[derive(Debug)]
pub enum ParserErrorType {
    InvalidAttribute,
    InvalidValue,
    InvalidName,
    UnexpectedEOF,
    UnmatchedClosingTag,
    UnexpectedCharacter { expected: Vec<char> },
}
