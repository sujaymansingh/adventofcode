use std::collections::VecDeque;

use crate::{
    core::{Result, Solver},
    string_scanner::StringScanner,
};

pub fn part_1() -> Box<dyn Solver> {
    Box::<Part1>::default()
}

pub fn part_2() -> Box<dyn Solver> {
    Box::<Part2>::default()
}

#[derive(Default)]
pub struct Part1(CardCollection);

impl Solver for Part1 {
    fn handle_line(&mut self, line: &str) -> Result<()> {
        self.0.add_card_from_string(line)
    }
    fn extract_solution(&self) -> Result<String> {
        Ok(self.0.total_points().to_string())
    }
}

#[derive(Default)]
pub struct Part2(CardCollection);

impl Solver for Part2 {
    fn handle_line(&mut self, line: &str) -> Result<()> {
        self.0.add_card_from_string(line)
    }
    fn extract_solution(&self) -> Result<String> {
        Ok(self.0.expanded_number().to_string())
    }
}

#[derive(Default)]
struct CardCollection {
    cards: Vec<Card>,
}

impl CardCollection {
    fn add_card_from_string(&mut self, line: &str) -> Result<()> {
        let card = Card::from_string(line)?;
        self.cards.push(card);
        Ok(())
    }

    fn total_points(&self) -> u32 {
        self.cards.iter().map(|c| c.num_points()).sum()
    }

    fn expanded_number(&self) -> u32 {
        let mut queue = VecDeque::new();
        let num_cards = self.cards.len() as u32;
        for i in 1..=num_cards {
            queue.push_back(i);
        }

        let mut count = 0;

        while let Some(id) = queue.pop_front() {
            let card = &self.cards[id as usize - 1];
            for i in 0..card.num_matching() {
                queue.push_back(id + i + 1);
            }
            count += 1;
        }

        count
    }
}

#[derive(Debug, Eq, Default, PartialEq)]
struct Card {
    id: usize,
    winning_numbers: Vec<u8>,
    actual_numbers: Vec<u8>,
}

impl Card {
    fn num_matching(&self) -> u32 {
        self.actual_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count() as u32
    }

    fn num_points(&self) -> u32 {
        let num = self.num_matching();
        if num == 0 {
            0
        } else {
            2_u32.pow(num - 1)
        }
    }

    fn from_string(line: &str) -> Result<Self> {
        let mut scanner = StringScanner::new(line);
        scanner.expect_string("Card")?;
        scanner.read_whitespace();
        let id: usize = scanner.expect_uint()?;
        scanner.expect_char(':')?;

        let mut winning_numbers = vec![];
        let mut actual_numbers = vec![];

        let mut finished_with_winning = false;

        while !scanner.is_finished() {
            scanner.read_whitespace();
            if let Some('|') = scanner.peek() {
                finished_with_winning = true;
                scanner.advance();
                continue;
            }

            let num: u8 = scanner.expect_uint()?;
            if finished_with_winning {
                actual_numbers.push(num);
            } else {
                winning_numbers.push(num);
            }
        }

        Ok(Self {
            id,
            winning_numbers,
            actual_numbers,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculate_number_of_points_correctly() {
        let card = Card {
            id: 1,
            winning_numbers: vec![41, 48, 83, 86, 17],
            actual_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };
        assert_eq!(card.num_points(), 8);

        let card = Card {
            id: 1,
            winning_numbers: vec![41, 92, 73, 84, 69],
            actual_numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
        };
        assert_eq!(card.num_points(), 1);

        let card = Card {
            id: 1,
            winning_numbers: vec![41, 92, 73, 84, 69],
            actual_numbers: vec![59, 85, 76, 51, 58, 5, 54, 83],
        };
        assert_eq!(card.num_points(), 0);
    }

    #[test]
    fn create_card_from_string() {
        let card = Card::from_string("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").unwrap();
        assert_eq!(
            card,
            Card {
                id: 1,
                winning_numbers: vec![41, 48, 83, 86, 17],
                actual_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            }
        );
    }

    fn sample_card_collection() -> CardCollection {
        let mut card_collection = CardCollection::default();
        for line in [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ] {
            card_collection.add_card_from_string(line).unwrap();
        }
        card_collection
    }

    #[test]
    fn expanding_cards() {
        let cc = sample_card_collection();
        assert_eq!(cc.expanded_number(), 30);
    }
}
