use std::io;
use std::result;
use std::{num::ParseIntError, ops::RangeInclusive, str::FromStr};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),
    #[error("Bad number: {0}")]
    BadNumber(#[from] ParseIntError),
}

pub type Result<T> = result::Result<T, CoreError>;

pub trait Solver {
    fn handle_line(&mut self, line: &str) -> Result<()>;
    fn extract_solution(&self) -> Result<String>;
}

#[derive(Debug, Error)]
pub enum ArgumentError {
    #[error("Bad numeric argument")]
    Number(#[from] ParseIntError),
    #[error("Value {0} is not within the range {1:?}")]
    OutOfRange(u16, RangeInclusive<u16>),
}

#[derive(Debug)]
pub struct Year(u16);

impl FromStr for Year {
    type Err = ArgumentError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let year = to_num_within_range(s, 2023..=2023)?;
        Ok(Self(year))
    }
}

impl ToString for Year {
    fn to_string(&self) -> String {
        format!("{:04}", self.0)
    }
}

impl Year {
    pub fn raw_value(&self) -> u16 {
        self.0
    }
}

#[derive(Debug)]
pub struct Day(u16);

impl FromStr for Day {
    type Err = ArgumentError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let day = to_num_within_range(s, 1..=25)?;
        Ok(Self(day))
    }
}

impl ToString for Day {
    fn to_string(&self) -> String {
        format!("{:02}", self.0)
    }
}

impl Day {
    pub fn raw_value(&self) -> u16 {
        self.0
    }
}

#[derive(Debug)]
pub struct Part(u16);

impl FromStr for Part {
    type Err = ArgumentError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let part = to_num_within_range(s, 1..=2)?;
        Ok(Self(part))
    }
}

impl ToString for Part {
    fn to_string(&self) -> String {
        format!("{:02}", self.0)
    }
}

impl Part {
    pub fn raw_value(&self) -> u16 {
        self.0
    }
}

fn to_num_within_range(s: &str, range: RangeInclusive<u16>) -> result::Result<u16, ArgumentError> {
    let raw_value = s.parse::<u16>()?;
    let value = assert_within_range_inclusive(raw_value, &range)?;
    Ok(value)
}

fn assert_within_range_inclusive(
    value: u16,
    range: &RangeInclusive<u16>,
) -> result::Result<u16, ArgumentError> {
    if range.contains(&value) {
        Ok(value)
    } else {
        Err(ArgumentError::OutOfRange(value, range.clone()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn err_if_value_not_in_range() {
        let range: RangeInclusive<u16> = 15..=20;
        let result = assert_within_range_inclusive(10, &range);

        if let Err(ArgumentError::OutOfRange(found_value, found_range)) = result {
            assert_eq!(found_value, 10);
            assert_eq!(found_range, range);
        } else {
            panic!("{}", &format!("Expected Err but got {:?}", result));
        }
    }

    #[test]
    fn ok_if_value_in_range() {
        let range: RangeInclusive<u16> = 5..=20;
        let in_range: result::Result<u16, ArgumentError> =
            assert_within_range_inclusive(10, &range);

        if let Ok(y) = in_range {
            assert_eq!(y, 10);
        } else {
            panic!("{}", &format!("Expected Ok(10) but got {:?}", in_range));
        }
    }
}
