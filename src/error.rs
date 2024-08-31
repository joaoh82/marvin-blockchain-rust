use thiserror::Error;

use std::result;

/// This is a type that encapsulated the `std::result` with the enum `MarvinError`
/// and makes function signatures easier to read.
pub type Result<T> = result::Result<T, MarvinError>;


/// MarvinError is an enum with all the standardized errors available for returning
///
#[derive(Error, Debug, PartialEq)]
pub enum MarvinError {
    #[error("Not Implemented error: {0}")]
    NotImplemented(String),
    #[error("General error: {0}")]
    General(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Returns SQLRiteError::General error from String
pub fn marvin_error(message: &str) -> MarvinError {
    MarvinError::General(message.to_owned())
}

#[cfg(test)]
mod tests {
    use std::fmt::format;

    use super::*;

    #[test]
    fn test_marvin_error() {
        let input = String::from("test error");
        let expected = MarvinError::General("test error".to_string());

        let result = marvin_error(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_marvin_error_not_implemented() {
        let error_string = String::from("not implemented");
        let input = MarvinError::NotImplemented(error_string.clone());

        let expected = format!("Not Implemented error: {}", error_string);

        let result = format!("{}", input);
        assert_eq!(result, expected);
    }

    #[test]
    fn marvin_display_general_test() {
        let error_string = String::from("General error.");
        let input = MarvinError::General(error_string.clone());

        let expected = format!("General error: {}", error_string);
        let result = format!("{}", input);
        assert_eq!(result, expected);
    }
}