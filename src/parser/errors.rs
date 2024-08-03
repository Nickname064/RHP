use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ParserError {
    pub(crate) char: usize,
    pub(crate) error_type: ParserErrorType,
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
