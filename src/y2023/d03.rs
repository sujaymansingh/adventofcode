/*
 * Ok, so how does this work?
 * Imagine the number 467.
 * Are there any symbols in an adjacent square?
 * - what are the adjacent squares?
 * - are there any symbols in it?
 *
 * Imagine 592 on line 6:
 * Adjacent squares are
 * - line 5: columns 1, 2, 3, 4, 5
 * - line 6: columns 1, 5
 * - line 7: columns 1, 2, 3, 4, 5
 *
 * 592 is at position (2, 6)  # x, y
 * so min max column is (x-1) to (x + len(num))
 * min max line is (y - 1), (y + 2)
 */

use crate::{
    core::{Result, Solver},
    string_scanner::StringScanner,
};
use std::collections::HashMap;

pub fn part_1() -> Box<dyn Solver> {
    Box::<SumOfPartNumbers>::default()
}

pub fn part_2() -> Box<dyn Solver> {
    Box::<SumOfGearRatios>::default()
}

#[derive(Default)]
pub struct SumOfPartNumbers {
    lines: Vec<String>,
}

impl Solver for SumOfPartNumbers {
    fn handle_line(&mut self, line: &str) -> Result<()> {
        self.lines.push(line.to_string());
        Ok(())
    }

    fn extract_solution(&self) -> Result<String> {
        let schematic = build_schematic(&self.lines)?;
        let sum: u32 = schematic.get_part_numbers().iter().map(|n| *n as u32).sum();
        Ok(sum.to_string())
    }
}

#[derive(Default)]
pub struct SumOfGearRatios {
    lines: Vec<String>,
}

impl Solver for SumOfGearRatios {
    fn handle_line(&mut self, line: &str) -> Result<()> {
        self.lines.push(line.to_string());
        Ok(())
    }

    fn extract_solution(&self) -> Result<String> {
        let schematic = build_schematic(&self.lines)?;
        let sum: u32 = schematic.get_gears().iter().map(|g| g.ratio()).sum();
        Ok(sum.to_string())
    }
}

fn build_schematic(lines: &[String]) -> Result<Schematic> {
    Schematic::from_lines(
        lines
            .iter()
            .map(AsRef::as_ref)
            .collect::<Vec<&str>>()
            .as_slice(),
    )
}

type Point = (u8, u8);

#[derive(Debug)]
struct Number {
    value: u16,
    num_digits: u8,
    position: Point,
}

fn calculate_num_digits(value: u16) -> u8 {
    if value < 10 {
        1
    } else if value < 100 {
        2
    } else if value < 1000 {
        3
    } else if value < 10_000 {
        4
    } else {
        unreachable!()
    }
}

#[derive(Debug)]
struct Gear {
    value_1: u16,
    value_2: u16,
}

impl Gear {
    fn ratio(&self) -> u32 {
        self.value_1 as u32 * self.value_2 as u32
    }
}

#[derive(Debug)]
struct Schematic {
    squares: Vec<Vec<char>>,
    numbers: Vec<Number>,
}

impl Schematic {
    fn is_part_number(&self, number: &Number) -> bool {
        self.neighbours_for(number)
            .iter()
            .any(|p| self.is_symbol_at(*p))
    }

    fn width(&self) -> u8 {
        self.squares[0].len() as u8
    }
    fn height(&self) -> u8 {
        self.squares.len() as u8
    }

    fn from_lines(lines: &[&str]) -> Result<Self> {
        let mut squares = vec![];
        let mut numbers = vec![];
        for (y, line) in lines.iter().enumerate() {
            let chars = line.chars().collect::<Vec<char>>();
            squares.push(chars);

            let mut scanner = StringScanner::new(line);
            let mut x = 0;

            while !scanner.is_finished() {
                match scanner.peek() {
                    Some(c) if c.is_ascii_digit() => {
                        let value: u16 = scanner.expect_uint()?;
                        let num_digits = calculate_num_digits(value);
                        let number = Number {
                            value,
                            num_digits,
                            position: (x, y as u8),
                        };
                        x += num_digits;
                        numbers.push(number);
                    }
                    _ => {
                        x += 1;
                        scanner.advance();
                    }
                }
            }
        }

        Ok(Self { squares, numbers })
    }

    fn is_symbol_at(&self, point: Point) -> bool {
        let (x, y) = point;
        let c = self.squares[y as usize][x as usize];
        !(c == '.' || c.is_ascii_digit())
    }

    fn is_star_at(&self, point: Point) -> bool {
        let (x, y) = point;
        let c = self.squares[y as usize][x as usize];
        c == '*'
    }

    fn neighbours_for(&self, number: &Number) -> Vec<Point> {
        let (x, y) = number.position;
        let num_digits = number.num_digits;
        let height = self.height();
        let width = self.width();

        let min_x = if x > 0 { x - 1 } else { 0 };
        let max_x = if (x + num_digits + 1) < width {
            x + num_digits + 1
        } else {
            width
        };

        let left = if x > 0 { Some((x - 1, y)) } else { None };
        let right = if (x + num_digits) < (width) {
            Some((x + num_digits, y))
        } else {
            None
        };

        let top_row: Vec<Point> = if y > 0 {
            (min_x..max_x).map(|x| (x, y - 1)).collect()
        } else {
            vec![]
        };

        let bottom_row: Vec<Point> = if y < (height - 1) {
            (min_x..max_x).map(|x| (x, y + 1)).collect()
        } else {
            vec![]
        };

        let neighbours = top_row
            .iter()
            .chain(left.iter())
            .chain(right.iter())
            .chain(bottom_row.iter())
            .copied()
            .collect();
        neighbours
    }

    fn get_part_numbers(&self) -> Vec<u16> {
        self.numbers
            .iter()
            .filter(|n| self.is_part_number(n))
            .map(|n| n.value)
            .collect()
    }

    fn get_gears(&self) -> Vec<Gear> {
        let mut potential_gears = HashMap::<Point, Vec<u16>>::new();

        for number in &self.numbers {
            for position in self.neighbours_for(number) {
                if !self.is_star_at(position) {
                    continue;
                }

                potential_gears
                    .entry(position)
                    .or_insert_with(std::vec::Vec::new);

                potential_gears
                    .get_mut(&position)
                    .unwrap()
                    .push(number.value);
            }
        }

        potential_gears
            .values()
            .filter(|v| v.len() == 2)
            .map(|v| {
                let value_1 = v[0];
                let value_2 = v[1];
                Gear { value_1, value_2 }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_schematic() -> Schematic {
        let lines: Vec<&str> = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];

        Schematic::from_lines(&lines).unwrap()
    }

    #[test]
    fn symbols_recorded_properly() {
        let schematic = sample_schematic();
        assert!(!schematic.is_symbol_at((0, 0)));
        assert!(schematic.is_symbol_at((3, 1)));
    }

    #[test]
    fn numbers_found() {
        let schematic = sample_schematic();

        let n467 = &schematic.numbers[0];
        assert_eq!(
            schematic.neighbours_for(n467),
            [(3, 0), (0, 1), (1, 1), (2, 1), (3, 1)]
        );

        let n35 = &schematic.numbers[2];
        assert_eq!(
            schematic.neighbours_for(n35),
            [
                (1, 1),
                (2, 1),
                (3, 1),
                (4, 1),
                (1, 2),
                (4, 2),
                (1, 3),
                (2, 3),
                (3, 3),
                (4, 3)
            ]
        );
    }
}
