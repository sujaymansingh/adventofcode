use crate::{
    core::{Result, Solver},
    string_scanner::StringScanner,
};

pub fn part_1() -> Box<dyn Solver> {
    Box::new(AlmanacSolver::new(SeedBehaviour::Simple))
}

pub fn part_2() -> Box<dyn Solver> {
    Box::new(AlmanacSolver::new(SeedBehaviour::Range))
}

pub struct AlmanacSolver(Almanac);

impl AlmanacSolver {
    fn new(seed_behaviour: SeedBehaviour) -> Self {
        Self(Almanac::new(seed_behaviour))
    }
}

impl Solver for AlmanacSolver {
    fn handle_line(&mut self, line: &str) -> Result<()> {
        self.0.handle_line(line)
    }

    fn extract_solution(&self) -> Result<String> {
        let solution = self
            .0
            .location_numbers()
            .min()
            .map_or("No value".to_string(), |n| n.to_string());
        Ok(solution)
    }
}

enum SeedBehaviour {
    Simple,
    Range,
}

impl SeedBehaviour {
    fn expand(&self, seeds: Vec<u64>) -> Vec<u64> {
        match self {
            Self::Simple => seeds,
            Self::Range => {
                let mut expanded = vec![];

                let mut i = 0;
                while i < seeds.len() {
                    let start = seeds[i];
                    let end = start + seeds[i + 1];
                    i += 2;

                    for n in start..end {
                        expanded.push(n);
                    }
                }

                expanded
            }
        }
    }
}

struct Almanac {
    seed_behaviour: SeedBehaviour,
    seeds: Vec<u64>,
    value_maps: Vec<ValueMap>,
}

impl Almanac {
    fn new(seed_behaviour: SeedBehaviour) -> Self {
        Self {
            seed_behaviour,
            seeds: vec![],
            value_maps: vec![],
        }
    }

    fn handle_line(&mut self, line: &str) -> Result<()> {
        if line.trim().is_empty() {
            return Ok(());
        } else if line.starts_with("seeds: ") {
            let mut scanner = StringScanner::new(line);
            scanner.expect_string("seeds:")?;
            let mut seeds = vec![];
            while !scanner.is_finished() {
                scanner.read_whitespace();
                seeds.push(scanner.expect_uint::<u64>()?);
            }
            self.seeds = self.seed_behaviour.expand(seeds);
        } else if line.ends_with("map:") {
            self.value_maps.push(ValueMap::default());
        } else {
            let mut scanner = StringScanner::new(line);
            let destination_start: u64 = scanner.expect_uint()?;
            scanner.read_whitespace();
            let source_start: u64 = scanner.expect_uint()?;
            scanner.read_whitespace();
            let source_length: u64 = scanner.expect_uint()?;
            let range = ValueMapRange {
                destination_start,
                source_start,
                source_length,
            };
            self.value_maps.last_mut().unwrap().0.push(range);
        }
        Ok(())
    }

    fn calculate_location(&self, value: u64) -> u64 {
        self.value_maps
            .iter()
            .fold(value, |acc, map| map.map_value(acc))
    }

    fn location_numbers(&self) -> Box<dyn Iterator<Item = u64> + '_> {
        Box::new(self.seeds.iter().map(|n| self.calculate_location(*n)))
    }
}

#[derive(Default)]
struct ValueMap(Vec<ValueMapRange>);

impl ValueMap {
    fn map_value(&self, value: u64) -> u64 {
        self.0
            .iter()
            .find_map(|range| range.map_value(value))
            .unwrap_or(value)
    }
}

struct ValueMapRange {
    destination_start: u64,
    source_start: u64,
    source_length: u64,
}

impl ValueMapRange {
    fn map_value(&self, value: u64) -> Option<u64> {
        let end = self.source_start + self.source_length;
        if (self.source_start..end).contains(&value) {
            let delta = value - self.source_start;
            Some(self.destination_start + delta)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn map_range_converts_numbers_correctly() {
        let mr = ValueMapRange {
            destination_start: 50,
            source_start: 98,
            source_length: 2,
        };
        assert_eq!(mr.map_value(97), None);
        assert_eq!(mr.map_value(98), Some(50));
        assert_eq!(mr.map_value(99), Some(51));
        assert_eq!(mr.map_value(100), None);
    }

    #[test]
    fn value_is_mapped_correctly() {
        let value_map = ValueMap(vec![
            ValueMapRange {
                destination_start: 50,
                source_start: 98,
                source_length: 2,
            },
            ValueMapRange {
                destination_start: 52,
                source_start: 50,
                source_length: 48,
            },
        ]);

        assert_eq!(value_map.map_value(49), 49);
        assert_eq!(value_map.map_value(50), 52);
        assert_eq!(value_map.map_value(51), 53);
        assert_eq!(value_map.map_value(97), 99);
        assert_eq!(value_map.map_value(98), 50);
        assert_eq!(value_map.map_value(99), 51);
        assert_eq!(value_map.map_value(100), 100);
    }

    fn sample_almanac() -> Almanac {
        let mut almanac = Almanac::new(SeedBehaviour::Simple);
        for line in [
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ] {
            almanac.handle_line(line).unwrap();
        }
        almanac
    }

    #[test]
    fn can_calculate_location() {
        let almanac = sample_almanac();
        assert_eq!(almanac.calculate_location(79), 82);
    }

    #[test]
    fn location_numbers() {
        let almanac = sample_almanac();
        let locations = almanac.location_numbers().collect::<Vec<u64>>();
        assert_eq!(locations, vec![82, 43, 86, 35]);
    }

    #[test]
    fn expanding_seeds() {
        let behavior = SeedBehaviour::Range;
        let seeds = behavior.expand(vec![79, 14, 55, 13]);
        assert_eq!(
            seeds,
            vec![
                79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 55, 56, 57, 58, 59, 60, 61,
                62, 63, 64, 65, 66, 67,
            ]
        );
    }
}
