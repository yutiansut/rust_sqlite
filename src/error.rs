//! Error Types

use std::{
    error::Error, 
    fmt::{Display, Formatter},
    result,
};

use sqlparser::parser::ParserError;

pub type Result<T> = result::Result<T, SQLRiteError>;

/// SQLRite error
#[derive(Debug, PartialEq)]
pub enum SQLRiteError {
    NotImplemented(String),
    General(String),
    Internal(String),
    SqlError(ParserError),
    // IoError(io::Error),
}

impl<T> Into<Result<T>> for SQLRiteError {
    fn into(self) -> Result<T> {
        Err(self)
    }
}

/// Return SQLRite errors from String
pub fn sqlrite_error(message: &str) -> SQLRiteError {
    SQLRiteError::General(message.to_owned())
}

impl From<String> for SQLRiteError {
    fn from(e: String) -> Self {
        SQLRiteError::General(e)
    }
}

impl From<ParserError> for SQLRiteError {
    fn from(e: ParserError) -> Self {
        SQLRiteError::SqlError(e)
    }
}

// impl From<io::Error> for SQLRiteError {
//     fn from(e: io::Error) -> Self {
//         SQLRiteError::IoError(e)
//     }
// }

impl Display for SQLRiteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SQLRiteError::NotImplemented(ref desc) => write!(f, "Not implemented: {}", desc),
            SQLRiteError::General(ref desc) => write!(f, "General error: {}", desc),
            SQLRiteError::SqlError(ref desc) => write!(f, "SQL error: {:?}", desc),
            // SQLRiteError::IoError(ref desc) => write!(f, "IO error: {}", desc),
            SQLRiteError::Internal(desc) => write!(f, "Internal SQLRite error: {}", desc),
        }
    }
}

impl Error for SQLRiteError {}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn sqlrite_error_test() {
        let input = String::from("test error"); 
        let expected = SQLRiteError::General("test error".to_string());

        let result = sqlrite_error(&input);
        assert_eq!(result, expected);
    }
}