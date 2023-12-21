use crate::core::{Result, Solver};
use crate::string_scanner::StringScanner;

pub fn part_1() -> Box<dyn Solver> {
    let analyser = Part1 {
        original: CubeSet::new(12, 13, 14),
        sum: 0,
    };
    Box::new(analyser)
}

pub fn part_2() -> Box<dyn Solver> {
    Box::<Part2>::default()
}

#[derive(Debug)]
pub struct Part1 {
    original: CubeSet,
    sum: u16,
}

impl Solver for Part1 {
    fn handle_line(&mut self, line: &str) -> Result<()> {
        let mut scanner = StringScanner::new(line);
        let game = Game::from_scanner(&mut scanner)?;

        if game.is_possible(&self.original) {
            self.sum += game.id;
        }

        Ok(())
    }

    fn extract_solution(&self) -> Result<String> {
        Ok(self.sum.to_string())
    }
}

#[derive(Default)]
pub struct Part2 {
    sum: u32,
}

impl Solver for Part2 {
    fn handle_line(&mut self, line: &str) -> Result<()> {
        let mut scanner = StringScanner::new(line);
        let game = Game::from_scanner(&mut scanner)?;

        self.sum += game.minimal_cube_set().power() as u32;

        Ok(())
    }

    fn extract_solution(&self) -> Result<String> {
        Ok(self.sum.to_string())
    }
}

#[derive(Debug)]
struct Game {
    id: u16,
    cube_sets: Vec<CubeSet>,
}

impl Game {
    fn from_scanner(scanner: &mut StringScanner) -> Result<Self> {
        scanner.expect_string("Game ")?;
        let id = scanner.expect_uint()?;
        scanner.expect_string(": ")?;
        let mut cube_sets = vec![];

        while !scanner.is_finished() {
            let cube_set = CubeSet::from_scanner(scanner)?;
            cube_sets.push(cube_set);

            if scanner.match_char(';') {
                scanner.expect_char(' ')?;
            } else {
                break;
            }
        }

        Ok(Self { id, cube_sets })
    }

    fn is_possible(&self, actual: &CubeSet) -> bool {
        self.cube_sets
            .iter()
            .all(|cube_set| actual.allows(cube_set))
    }

    fn minimal_cube_set(&self) -> CubeSet {
        let mut final_cube_set = CubeSet::new(0, 0, 0);

        for cube_set in self.cube_sets.iter() {
            if cube_set.num_red > final_cube_set.num_red {
                final_cube_set.num_red = cube_set.num_red;
            }
            if cube_set.num_green > final_cube_set.num_green {
                final_cube_set.num_green = cube_set.num_green;
            }
            if cube_set.num_blue > final_cube_set.num_blue {
                final_cube_set.num_blue = cube_set.num_blue;
            }
        }

        final_cube_set
    }
}

#[derive(Debug, Eq, PartialEq)]
struct CubeSet {
    num_red: u16,
    num_green: u16,
    num_blue: u16,
}

impl CubeSet {
    fn new(num_red: u16, num_green: u16, num_blue: u16) -> Self {
        Self {
            num_red,
            num_green,
            num_blue,
        }
    }

    fn from_scanner(scanner: &mut StringScanner) -> Result<Self> {
        let mut cube_set = Self::new(0, 0, 0);
        loop {
            if scanner.is_finished() {
                break;
            }
            if let Some(c) = scanner.peek() {
                if c == ';' {
                    break;
                }
            }
            let num: u16 = scanner.expect_uint()?;
            scanner.expect_char(' ')?;

            if scanner.match_string("red") {
                cube_set.num_red = num;
            } else if scanner.match_string("green") {
                cube_set.num_green = num;
            } else if scanner.match_string("blue") {
                cube_set.num_blue = num;
            } else {
                // TODO??
            }

            if scanner.match_char(',') {
                scanner.expect_char(' ')?;
            } else {
                break;
            }
        }
        Ok(cube_set)
    }

    fn allows(&self, other: &Self) -> bool {
        other.num_red <= self.num_red
            && other.num_green <= self.num_green
            && other.num_blue <= self.num_blue
    }

    fn power(&self) -> u16 {
        self.num_red * self.num_green * self.num_blue
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cube_set_from_scanner() {
        let mut scanner = StringScanner::new("3 blue, 4 red;");

        let cube_set = CubeSet::from_scanner(&mut scanner).unwrap();
        assert_eq!(cube_set, CubeSet::new(4, 0, 3));
    }

    #[test]
    fn game_from_scanner() {
        let mut scanner = StringScanner::new(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        );

        let game = Game::from_scanner(&mut scanner).unwrap();

        assert_eq!(game.id, 3);

        assert_eq!(
            game.cube_sets,
            vec![
                CubeSet::new(20, 8, 6),
                CubeSet::new(4, 13, 5),
                CubeSet::new(1, 5, 0),
            ]
        );
    }

    #[test]
    fn game_ids_are_summed_up() {
        let mut g = Part1 {
            original: CubeSet::new(12, 13, 14),
            sum: 0,
        };
        g.handle_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
            .unwrap();
        g.handle_line("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")
            .unwrap();
        g.handle_line("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")
            .unwrap();
        g.handle_line("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red")
            .unwrap();
        g.handle_line("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
            .unwrap();
        assert_eq!(g.sum, 8);
    }

    #[test]
    fn calculate_minimum_cubeset() {
        let game = Game {
            id: 3,
            cube_sets: vec![
                CubeSet::new(20, 8, 6),
                CubeSet::new(4, 13, 5),
                CubeSet::new(1, 5, 0),
            ],
        };

        assert_eq!(game.minimal_cube_set(), CubeSet::new(20, 13, 6));
    }
}
