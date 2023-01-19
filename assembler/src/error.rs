use crate::token::TokenType;
use std::fmt::Display;


#[derive(Debug, Clone)]
pub enum ErrorKind {
    NumberError(String),
    InvalidArguments(String, String),
    UnexpectedLexeme(String),
    UnexpectedToken(Vec<TokenType>, TokenType),
    Eof,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ErrorKind: ")?;
        match self {
            ErrorKind::NumberError(number) => {
                f.write_fmt(format_args!("Number out of bounds: {}", number))
            }
            ErrorKind::InvalidArguments(expected, found) => f.write_fmt(format_args!(
                "Invalid Arguments: expected {}, found {}",
                expected, found
            )),
            ErrorKind::UnexpectedLexeme(found) => {
                f.write_fmt(format_args!("Unexpected token: {}", found))
            }
            ErrorKind::UnexpectedToken(expected, found) => f.write_fmt(format_args!(
                "Invalid Arguments: expected {:?}, found {:?}",
                expected, found
            )),
            ErrorKind::Eof => f.write_str("Reached end of file!"),
        }
    }
}

pub struct ParseError {
    pub position: (usize, usize),
    pub error: ErrorKind,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "\nParse error at {}:{}\nError kind: {}",
            self.position.0, self.position.1, self.error
        ))
    }
}
