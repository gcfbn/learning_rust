use derive_more::{Add, Display, Sub};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::cmp::{Eq, PartialEq, PartialOrd};
use std::convert::TryFrom;
use std::ops::{Add, Sub};
use std::str::FromStr;
use thiserror::Error;

#[derive(
    Add, Sub, Clone, Copy, Debug, Display, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
#[serde(try_from = "u32")]
pub struct PositiveInteger(u32);

impl PositiveInteger {
    pub fn value(&self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for PositiveInteger {
    type Error = PositiveIntegerError;
    fn try_from(val: u32) -> Result<Self, Self::Error> {
        if val == 0 {
            return Err(PositiveIntegerError::InputNumberIsEqZeroError);
        }
        Ok(PositiveInteger(val))
    }
}

impl Add<u32> for PositiveInteger {
    type Output = Self;

    fn add(self, other: u32) -> Self {
        Self(self.value() + other)
    }
}

impl Sub<u32> for PositiveInteger {
    type Output = Self;

    fn sub(self, other: u32) -> Self {
        let new_value = self.value() - other;
        if new_value == 0 {
            panic!("0 is not allowed - it must be a positive integer");
        }

        Self(new_value)
    }
}

impl PartialOrd<u32> for PositiveInteger {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialEq<u32> for PositiveInteger {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

#[derive(Debug, Error)]
pub enum PositiveIntegerError {
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
    type Err = PositiveIntegerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ivalue: isize =
            s.parse().map_err(
                |err| PositiveIntegerError::InputNumberIsNotIntegerError {
                    input: s.to_string(),
                    parse_error_message: format!("{}", err),
                },
            )?;

        if ivalue < 0 {
            return Err(PositiveIntegerError::InputNumberIsNegativeError(
                s.to_string(),
            ));
        }

        Self::try_from(ivalue as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse {
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

    mod try_from {
        use super::*;

        #[test]
        fn ok() {
            assert_eq!(PositiveInteger::try_from(1).unwrap().value(), 1);
        }

        #[test]
        fn fails_becuase_zero_as_input_value() {
            assert!(PositiveInteger::try_from(0).is_err());
        }
    }

    mod json {
        use super::*;

        #[test]
        fn serialize() {
            let actual = serde_json::to_string(&PositiveInteger(10)).unwrap();
            let expected = "10";
            assert_eq!(actual, expected);
        }

        #[test]
        fn deserialize() {
            let input = "10";
            let actual: PositiveInteger = serde_json::from_str(input).unwrap();
            let expected = PositiveInteger(10);

            assert_eq!(actual, expected);
        }

        #[test]
        fn failing_deserialization_aka_validation() {
            let input = "0";
            let result: Result<PositiveInteger, serde_json::Error> = serde_json::from_str(input);

            assert!(result.is_err());
        }
    }

    mod arithmetic {
        use super::*;
        use test_case::test_case;

        #[test]
        fn add() {
            assert_eq!(PositiveInteger(2) + PositiveInteger(2), 4);
            assert_eq!(PositiveInteger(2) + PositiveInteger(2), PositiveInteger(4));
        }

        #[test]
        fn add_primitive() {
            assert_eq!(PositiveInteger(1) + 1, 2);
        }

        #[test]
        fn sub() {
            assert_eq!(PositiveInteger(5) - PositiveInteger(1), 4);
            assert_eq!(PositiveInteger(5) - PositiveInteger(1), PositiveInteger(4));
        }

        #[test]
        fn sub_primitive() {
            assert_eq!(PositiveInteger(2) - 1, 1);
            assert_eq!(PositiveInteger(2) - 1, PositiveInteger(1));
        }

        #[test_case(2, 2; "zero is not allowed")]
        #[test_case(2, 3; "negative value is not allowed")]
        #[should_panic]
        fn failing_sub(u1: u32, u2: u32) {
            let _ = PositiveInteger(u1) - u2;
        }
    }

    mod compare {
        use super::*;

        #[test]
        fn eq() {
            assert!(PositiveInteger(1) == 1);
            assert!(PositiveInteger(1) == PositiveInteger(1));
        }

        #[test]
        fn lt() {
            assert!(PositiveInteger(1) < 2);
            assert!(PositiveInteger(1) < PositiveInteger(2));
        }

        #[test]
        fn le() {
            assert!(PositiveInteger(1) <= 2);
            assert!(PositiveInteger(1) <= PositiveInteger(2));
        }

        #[test]
        fn gt() {
            assert!(PositiveInteger(2) > 1);
            assert!(PositiveInteger(2) > PositiveInteger(1));
        }

        #[test]
        fn ge() {
            assert!(PositiveInteger(2) >= 1);
            assert!(PositiveInteger(2) >= PositiveInteger(1));
        }
    }
}
