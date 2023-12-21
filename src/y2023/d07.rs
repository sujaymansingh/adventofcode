use std::{cmp::Ordering, collections::HashMap};

use crate::{
    core::{CoreError, Result, Solver},
    string_scanner::StringScanner,
};

pub fn part_1() -> Box<dyn Solver> {
    Box::new(HandsWithBids::new(CompareType::Basic))
}

pub fn part_2() -> Box<dyn Solver> {
    Box::new(HandsWithBids::new(CompareType::Joker))
}

#[derive(Debug)]
struct HandsWithBids(Vec<HandWithBid>, CompareType);

impl Solver for HandsWithBids {
    fn handle_line(&mut self, line: &str) -> Result<()> {
        self.0.push(HandWithBid::from_string(line)?);
        Ok(())
    }

    fn extract_solution(&self) -> Result<String> {
        Ok(self.total_score().to_string())
    }
}

impl HandsWithBids {
    fn new(compare_type: CompareType) -> Self {
        Self(vec![], compare_type)
    }

    fn total_score(&self) -> u64 {
        let mut other = self.0.clone();
        other.sort_by(|a, b| self.1.compare_hands(&a.hand, &b.hand));

        other
            .iter()
            .enumerate()
            .map(|(i, hand_with_bid)| {
                let rank = i + 1;
                (rank as u64) * hand_with_bid.bid
            })
            .sum::<u64>()
    }
}

#[derive(Debug)]
enum CompareType {
    Basic,
    Joker,
}

impl CompareType {
    fn compare_hands(&self, hand_1: &Hand, hand_2: &Hand) -> Ordering {
        let hand_type_compare = match self {
            Self::Basic => hand_1.hand_type().cmp(&hand_2.hand_type()),
            Self::Joker => hand_1.best_hand_type().cmp(&hand_2.best_hand_type()),
        };

        match hand_type_compare {
            Ordering::Equal => {
                for (c_1, c_2) in hand_1.0.iter().zip(hand_2.0.iter()) {
                    let label_compare = match self {
                        Self::Basic => c_1.cmp(c_2),
                        Self::Joker => c_1.joker_cmp(c_2),
                    };
                    match label_compare {
                        Ordering::Equal => {}
                        x => {
                            return x;
                        }
                    }
                }
                Ordering::Equal
            }
            y => y,
        }
    }
}

#[derive(Debug, Clone)]
struct HandWithBid {
    hand: Hand,
    bid: u64,
}

impl HandWithBid {
    fn from_string(line: &str) -> Result<Self> {
        let mut scanner = StringScanner::new(line);
        let hand = Hand::from_string_scanner(&mut scanner)?;
        scanner.read_whitespace();
        let bid = scanner.expect_uint()?;
        Ok(Self { hand, bid })
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Hand([Label; 5]);

impl Hand {
    fn from_string_scanner(scanner: &mut StringScanner) -> Result<Self> {
        let mut labels = vec![];
        for _ in 0..5 {
            if let Some(c) = scanner.peek() {
                labels.push(Label::from_char(c)?);
                scanner.advance();
            } else {
                return Err(CoreError::general("Couldn't read 5 labels"));
            }
        }

        Ok(Self([
            labels[0], labels[1], labels[2], labels[3], labels[4],
        ]))
    }

    fn hand_type(&self) -> HandType {
        let mut label_counts = HashMap::new();
        for label in self.0.iter() {
            label_counts
                .entry(label)
                .and_modify(|count| *count += 1)
                .or_insert(1_u8);
        }

        let mut counts = label_counts.values().copied().collect::<Vec<u8>>();
        counts.sort();

        match counts.last() {
            Some(5) => HandType::FiveOfAKind,
            Some(4) => HandType::FourOfAKind,
            Some(3) => {
                if counts[0] == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            Some(2) => {
                if counts.len() == 3 {
                    // Must be 1, 2, 2
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            _ => HandType::HighCard,
        }
    }

    fn possible_hands(&self) -> Vec<Self> {
        let mut concrete = vec![];
        let mut might_be_expanded = vec![self.clone()];

        while let Some(hand) = might_be_expanded.pop() {
            match hand.0.iter().position(Label::is_joker) {
                Some(i) => {
                    for new_label in Label::non_jokers() {
                        might_be_expanded.push(hand.replaced(i, new_label));
                    }
                }
                None => {
                    concrete.push(hand);
                }
            }
        }

        concrete
    }

    fn replaced(&self, i: usize, new_label: Label) -> Self {
        let mut hand = self.clone();
        hand.0[i] = new_label;
        hand
    }

    fn best_hand_type(&self) -> HandType {
        self.possible_hands()
            .iter()
            .map(Self::hand_type)
            .max_by(HandType::cmp)
            .unwrap_or_default()
    }
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    #[default]
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum Label {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Label {
    fn from_char(c: char) -> Result<Self> {
        match c {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => Err(CoreError::general(&format!("Invalid char: {}", c))),
        }
    }

    fn is_joker(&self) -> bool {
        self == &Self::Jack
    }

    fn non_jokers() -> Vec<Self> {
        vec![
            Self::Two,
            Self::Three,
            Self::Four,
            Self::Five,
            Self::Six,
            Self::Seven,
            Self::Eight,
            Self::Nine,
            Self::Ten,
            Self::Queen,
            Self::King,
            Self::Ace,
        ]
    }

    fn joker_cmp(&self, other: &Label) -> Ordering {
        match (self, other) {
            (a, b) if a == b => Ordering::Equal,
            (Self::Jack, _) => Ordering::Less,
            (_, Self::Jack) => Ordering::Greater,
            (_, _) => self.cmp(other),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn make_hand(line: &str) -> Hand {
        let mut scanner = StringScanner::new(line);
        Hand::from_string_scanner(&mut scanner).unwrap()
    }

    #[test]
    fn can_get_type_of_hands() {
        let hand = make_hand("32T3K");
        assert_eq!(hand.hand_type(), HandType::OnePair);

        let hand = make_hand("T55J5");
        assert_eq!(hand.hand_type(), HandType::ThreeOfAKind);

        let hand = make_hand("KK677");
        assert_eq!(hand.hand_type(), HandType::TwoPair);

        let hand = make_hand("KTJJT");
        assert_eq!(hand.hand_type(), HandType::TwoPair);

        let hand = make_hand("QQQJA");
        assert_eq!(hand.hand_type(), HandType::ThreeOfAKind);
    }

    #[test]
    fn can_basic_sort_hands() {
        let mut hands = vec![
            make_hand("32T3K"),
            make_hand("T55J5"),
            make_hand("KK677"),
            make_hand("KTJJT"),
            make_hand("QQQJA"),
        ];

        hands.sort_by(|a, b| CompareType::Basic.compare_hands(a, b));

        assert_eq!(
            hands,
            vec![
                make_hand("32T3K"),
                make_hand("KTJJT"),
                make_hand("KK677"),
                make_hand("T55J5"),
                make_hand("QQQJA"),
            ]
        );
    }

    fn make_hands_with_bids(compare_type: CompareType) -> HandsWithBids {
        let raw: Vec<HandWithBid> = [
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .iter()
        .map(|line| HandWithBid::from_string(line).unwrap())
        .collect();
        HandsWithBids(raw, compare_type)
    }

    #[test]
    fn total_score() {
        let hands_with_bids = make_hands_with_bids(CompareType::Basic);
        assert_eq!(hands_with_bids.total_score(), 6440);
    }

    #[test]
    fn expand_a_hand_with_jokers() {
        let hand = make_hand("A23JK");
        use Label::*;
        assert_eq!(
            hand.possible_hands(),
            [
                Hand([Ace, Two, Three, Ace, King]),
                Hand([Ace, Two, Three, King, King]),
                Hand([Ace, Two, Three, Queen, King]),
                Hand([Ace, Two, Three, Ten, King]),
                Hand([Ace, Two, Three, Nine, King]),
                Hand([Ace, Two, Three, Eight, King]),
                Hand([Ace, Two, Three, Seven, King]),
                Hand([Ace, Two, Three, Six, King]),
                Hand([Ace, Two, Three, Five, King]),
                Hand([Ace, Two, Three, Four, King]),
                Hand([Ace, Two, Three, Three, King]),
                Hand([Ace, Two, Three, Two, King])
            ]
        );
    }

    #[test]
    fn best_hand_type() {
        for hand_string in ["T55J5", "KTJJT", "QQQJA"] {
            let hand = make_hand(hand_string);
            assert_eq!(hand.best_hand_type(), HandType::FourOfAKind);
        }
    }

    #[test]
    fn total_score_with_jokers() {
        let hands_with_bids = make_hands_with_bids(CompareType::Joker);
        assert_eq!(hands_with_bids.total_score(), 5905);
    }
}
