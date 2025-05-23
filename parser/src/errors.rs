#[derive(Debug, Clone)]
pub struct ParserError {
    pub char: usize,
    pub error_type: ParserErrorType,
}

#[derive(Debug, Clone)]
pub enum ParserErrorType {
    InvalidAttribute,
    InvalidValue,
    InvalidName,
    UnexpectedEOF,
    UnmatchedClosingTag,
    UnexpectedCharacter { expected: Vec<char> },
}
