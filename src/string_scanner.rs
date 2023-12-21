use std::{num::ParseIntError, str::FromStr};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum StringScannerError {
    #[error("Didn't find '{expected}' at position: {position}")]
    UnexpectedString { expected: String, position: usize },
    #[error("Didn't find uint at position: {position}. Source Err = {source_error:?}")]
    NotAUint {
        source_error: ParseIntError,
        position: usize,
    },
    #[error("Didn't find '{expected}' at position: {position}")]
    UnexpectedChar { expected: char, position: usize },
}

#[derive(Debug)]
pub struct StringScanner {
    current_position: usize,
    chars: Vec<char>,
}

impl StringScanner {
    pub fn new(source: &str) -> Self {
        let chars = source.chars().collect();
        Self {
            current_position: 0,
            chars,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.current_position >= self.chars.len()
    }

    pub fn peek(&self) -> Option<char> {
        if self.is_finished() {
            None
        } else {
            Some(self.chars[self.current_position])
        }
    }

    pub fn peek_string(&self, other: &str) -> bool {
        for (i, other_char) in other.chars().enumerate() {
            match self.peek_forward(i) {
                Some(this_char) if this_char == other_char => {}
                _ => {
                    return false;
                }
            }
        }
        true
    }

    pub fn peek_forward(&self, n: usize) -> Option<char> {
        if (self.current_position + n) >= self.chars.len() {
            None
        } else {
            Some(self.chars[self.current_position + n])
        }
    }

    pub fn advance(&mut self) {
        if !self.is_finished() {
            self.current_position += 1;
        }
    }

    pub fn advance_by(&mut self, n: usize) {
        self.current_position += n;
        if self.current_position > self.chars.len() {
            self.current_position = self.chars.len();
        }
    }

    pub fn match_char(&mut self, c: char) -> bool {
        match self.peek() {
            Some(d) if c == d => {
                self.advance();
                true
            }
            _ => false,
        }
    }

    pub fn match_string(&mut self, other: &str) -> bool {
        if self.peek_string(other) {
            self.advance_by(other.len());
            true
        } else {
            false
        }
    }

    fn read_while<F>(&mut self, char_func: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.is_finished() {
            match self.peek() {
                Some(c) if char_func(c) => {
                    result.push(c);
                    self.advance();
                }
                _ => {
                    break;
                }
            }
        }
        result
    }

    pub fn read_whitespace(&mut self) -> String {
        self.read_while(char::is_whitespace)
    }

    pub fn expect_uint<T>(&mut self) -> Result<T, StringScannerError>
    where
        T: FromStr<Err = ParseIntError>,
    {
        let number_string = self.read_while(|c| c.is_ascii_digit());
        match T::from_str(&number_string) {
            Ok(x) => Ok(x),
            Err(e) => Err(StringScannerError::NotAUint {
                source_error: e,
                position: self.current_position,
            }),
        }
    }

    pub fn expect_char(&mut self, c: char) -> Result<(), StringScannerError> {
        if self.match_char(c) {
            Ok(())
        } else {
            Err(StringScannerError::UnexpectedChar {
                expected: c,
                position: self.current_position,
            })
        }
    }

    pub fn expect_string(&mut self, other: &str) -> Result<(), StringScannerError> {
        if self.match_string(other) {
            Ok(())
        } else {
            Err(StringScannerError::UnexpectedString {
                expected: other.to_string(),
                position: self.current_position,
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_peek_forward() {
        let scanner = StringScanner::new("bar");

        assert_eq!(scanner.peek_forward(0), Some('b'));
        assert_eq!(scanner.peek_forward(1), Some('a'));
        assert_eq!(scanner.peek_forward(2), Some('r'));
        assert_eq!(scanner.peek_forward(3), None);
    }

    #[test]
    fn test_peek_string() {
        let mut scanner = StringScanner::new("Something in the way");

        assert!(!scanner.peek_string("Nothing"));
        assert!(scanner.peek_string("Something"));
        for _ in 0.."Something".len() {
            scanner.advance();
        }

        assert!(!scanner.peek_string("in the way"));
        assert!(scanner.peek_string(" in the way"));
    }

    #[test]
    fn test_read_while() {
        let mut scanner = StringScanner::new("aabacdcd");
        let part_1 = scanner.read_while(|c| c == 'a' || c == 'b');
        assert_eq!(part_1, "aaba".to_string());
        let part_2 = scanner.read_while(|c| c == 'c' || c == 'd');
        assert_eq!(part_2, "cdcd".to_string());
    }

    #[test]
    fn test_expect_uint() {
        let mut scanner = StringScanner::new("20 January");
        assert_eq!(scanner.expect_uint::<u32>().unwrap(), 20);
    }
}
