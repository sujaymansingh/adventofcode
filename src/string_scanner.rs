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
}
