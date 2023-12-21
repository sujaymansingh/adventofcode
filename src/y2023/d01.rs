use crate::core::{CoreError, Solver};
use crate::string_scanner::StringScanner;

const TOKENS_AND_VALUES: [(&str, u32); 20] = [
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

pub fn part_1() -> Box<dyn Solver> {
    Box::<ExtractAndSum>::default()
}

pub fn part_2() -> Box<dyn Solver> {
    Box::<ExtractAndSumWithWords>::default()
}

#[derive(Default)]
pub struct ExtractAndSum {
    total: u32,
}

impl Solver for ExtractAndSum {
    fn handle_line(&mut self, line: &str) -> Result<(), CoreError> {
        self.total += extract_number(line, false)?;
        Ok(())
    }

    fn extract_solution(&self) -> Result<String, CoreError> {
        Ok(self.total.to_string())
    }
}

#[derive(Default)]
pub struct ExtractAndSumWithWords {
    total: u32,
}

impl Solver for ExtractAndSumWithWords {
    fn handle_line(&mut self, line: &str) -> Result<(), CoreError> {
        self.total += extract_number(line, true)?;
        Ok(())
    }

    fn extract_solution(&self) -> Result<String, CoreError> {
        Ok(self.total.to_string())
    }
}

fn extract_digits_no_words(line: &str) -> Box<dyn Iterator<Item = u32> + '_> {
    let digits = line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| String::from(c).parse::<u32>().unwrap());
    Box::new(digits)
}

fn extract_digits_with_words(line: &str) -> Box<dyn Iterator<Item = u32> + '_> {
    Box::new(DigitExtractor {
        scanner: StringScanner::new(line),
    })
}

fn extract_number(line: &str, include_words: bool) -> Result<u32, CoreError> {
    let mut first_digit: Option<u32> = None;
    let mut last_digit = 0;

    let all_digits = if include_words {
        extract_digits_with_words(line)
    } else {
        extract_digits_no_words(line)
    };

    for digit in all_digits {
        if first_digit.is_none() {
            first_digit = Some(digit);
        }
        last_digit = digit;
    }

    let number = first_digit.map_or(0, |x| x * 10 + last_digit);

    Ok(number)
}

struct DigitExtractor {
    scanner: StringScanner,
}

impl Iterator for DigitExtractor {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.scanner.is_finished() {
            for (token, digit) in TOKENS_AND_VALUES {
                if self.scanner.peek_string(token) {
                    self.scanner.advance();
                    return Some(digit);
                }
            }
            self.scanner.advance();
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extracts_a_number_from_a_line() {
        assert_eq!(extract_number("1abc2", false).unwrap(), 12);
        assert_eq!(extract_number("pqr3stu8vwx", false).unwrap(), 38);
        assert_eq!(extract_number("a1b2c3d4e5f", false).unwrap(), 15);
        assert_eq!(extract_number("treb7uchet", false).unwrap(), 77);
    }

    #[test]
    fn digit_extractor() {
        let digits = DigitExtractor {
            scanner: StringScanner::new("xtwone3four"),
        };
        let all_digits: Vec<u32> = digits.collect();
        assert_eq!(all_digits, vec![2, 1, 3, 4]);
    }
}
