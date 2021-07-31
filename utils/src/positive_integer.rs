use derive_more::{Add, Display};
use std::convert::TryInto;
use std::ops::Add;
use std::str::FromStr;
use thiserror::Error;

#[derive(Add, Clone, Copy, Debug, Display, PartialEq, PartialOrd)]
pub struct PositiveInteger<T = u32>(T);

impl<T: Clone> PositiveInteger<T> {
    pub fn value(&self) -> T {
        self.0.clone()
    }
}

impl<T> From<T> for PositiveInteger<T> {
    fn from(val: T) -> Self {
        PositiveInteger(val)
    }
}

impl<T: Clone + Add<Output = T>> Add<T> for PositiveInteger<T> {
    type Output = Self;

    fn add(self, other: T) -> Self {
        Self(self.value() + other)
    }
}

#[derive(Debug, Error)]
pub enum ParsePositiveIntegerError {
    #[error("cannot be negative - {0}")]
    InputNumberIsNegativeError(String),

    #[error("0 is not allowed - it must be a positive integer")]
    InputNumberIsEqZeroError,

    #[error("{input} - input string is not an integer -> {parse_error_message}")]
    InputNumberIsNotIntegerError {
        input: String,
        parse_error_message: String,
    },
}

impl FromStr for PositiveInteger {
    type Err = ParsePositiveIntegerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let count_integer: isize =
            s.parse().map_err(
                |err| ParsePositiveIntegerError::InputNumberIsNotIntegerError {
                    input: s.to_string(),
                    parse_error_message: format!("{}", err),
                },
            )?;

        if count_integer < 0 {
            return Err(ParsePositiveIntegerError::InputNumberIsNegativeError(
                s.to_string(),
            ));
        }

        if count_integer == 0 {
            return Err(ParsePositiveIntegerError::InputNumberIsEqZeroError);
        }

        Ok(PositiveInteger(count_integer.try_into().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("10", 10)]
    #[test_case("1", 1)]
    fn ok(input: &str, expected: u32) {
        let actual = input.parse::<PositiveInteger>().unwrap();
        assert_eq!(actual.value(), expected);
    }

    #[test_case("200a"; "not an integer v1")]
    #[test_case("3,5"; "not an integer v2")]
    #[test_case("0"; "zero")]
    #[test_case("-123"; "negative")]
    fn failing(input: &str) {
        let result = input.parse::<PositiveInteger>();
        assert!(result.is_err());
    }
}
