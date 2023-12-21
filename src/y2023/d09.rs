use std::{collections::VecDeque, num::ParseIntError};

use crate::core::{CoreError, Result as CoreResult, Solver};

pub fn part_1() -> Box<dyn Solver> {
    Box::new(Part1(0, Direction::Right))
}

pub fn part_2() -> Box<dyn Solver> {
    Box::new(Part1(0, Direction::Left))
}

struct Part1(i32, Direction);

enum Direction {
    Left,
    Right,
}

impl Solver for Part1 {
    fn handle_line(&mut self, line: &str) -> CoreResult<()> {
        let numbers = line
            .split(' ')
            .map(|s| s.parse())
            .collect::<Result<Vec<i32>, ParseIntError>>()?;
        let mut sequence = Sequence(numbers);
        sequence.expand_once()?;
        let next_number = match self.1 {
            Direction::Left => first(&sequence.0)?,
            Direction::Right => last(&sequence.0)?,
        };
        self.0 += next_number;
        Ok(())
    }

    fn extract_solution(&self) -> CoreResult<String> {
        Ok(self.0.to_string())
    }
}

struct Sequence(Vec<i32>);

impl Sequence {
    fn expand_once(&mut self) -> CoreResult<()> {
        let nums = std::mem::take(&mut self.0);

        let mut sequences = VecDeque::from(vec![nums]);

        loop {
            let sequence = &sequences[0];
            if all_zero(sequence) {
                break;
            }
            sequences.push_front(deltas(sequence));
        }

        while sequences.len() > 1 {
            let top = pop_front(&mut sequences)?;
            let mut bottom = pop_front(&mut sequences)?;

            let new_first = first(&bottom)? - first(&top)?;
            let new_last = last(&bottom)? + last(&top)?;

            bottom.insert(0, new_first);
            bottom.push(new_last);

            sequences.push_front(bottom);
        }

        self.0 = pop_front(&mut sequences)?;
        Ok(())
    }
}

fn first(items: &[i32]) -> CoreResult<i32> {
    match items.first() {
        None => Err(CoreError::general(
            "Attempted to get first item of empty collection",
        )),
        Some(x) => Ok(*x),
    }
}

fn last(items: &[i32]) -> CoreResult<i32> {
    match items.last() {
        None => Err(CoreError::general(
            "Attempted to get first item of empty collection",
        )),
        Some(x) => Ok(*x),
    }
}

fn pop_front(items: &mut VecDeque<Vec<i32>>) -> CoreResult<Vec<i32>> {
    match items.pop_front() {
        None => Err(CoreError::general(
            "Attempted to pop from front of empty collection",
        )),
        Some(x) => Ok(x),
    }
}

fn deltas(nums: &[i32]) -> Vec<i32> {
    let mut nums = nums.iter();

    let mut current = match nums.next() {
        Some(x) => x,
        None => {
            return vec![];
        }
    };

    nums.map(|n| {
        let delta = n - current;
        current = n;
        delta
    })
    .collect()
}

fn all_zero(nums: &[i32]) -> bool {
    nums.iter().all(|n| *n == 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_calculate_deltas() {
        assert_eq!(deltas(&[10, 13, 16, 21, 30, 45]), vec![3, 3, 5, 9, 15]);
        assert_eq!(deltas(&[3, 3, 5, 9, 15]), vec![0, 2, 4, 6]);
        assert_eq!(deltas(&[0, 2, 4, 6]), vec![2, 2, 2]);
        assert_eq!(deltas(&[2, 2, 2]), vec![0, 0]);
    }

    #[test]
    fn can_expand_sequence() {
        let nums = [10, 13, 16, 21, 30, 45];

        let mut s = Sequence(nums.to_vec());
        s.expand_once().unwrap();
        assert_eq!(s.0, [5, 10, 13, 16, 21, 30, 45, 68]);
    }
}
