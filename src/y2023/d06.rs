use crate::{
    core::{CoreError, Result, Solver},
    string_scanner::StringScanner,
};

pub fn part_1() -> Box<dyn Solver> {
    let calculator = NumWaysCalculator(Box::<SimpleRacesBuilder>::default());
    Box::new(calculator)
}

pub fn part_2() -> Box<dyn Solver> {
    let calculator = NumWaysCalculator(Box::<ConcatRacesBuilder>::default());
    Box::new(calculator)
}

struct NumWaysCalculator(Box<dyn RacesBuilder>);

impl Solver for NumWaysCalculator {
    fn handle_line(&mut self, line: &str) -> Result<()> {
        self.0.add_line(line)
    }

    fn extract_solution(&self) -> Result<String> {
        let races = self.0.build()?;
        Ok(races.margin_of_error().to_string())
    }
}

trait RacesBuilder {
    fn add_line(&mut self, line: &str) -> Result<()>;
    fn build(&self) -> Result<Races>;
}

#[derive(Default)]
struct SimpleRacesBuilder(Vec<Vec<u64>>);

impl RacesBuilder for SimpleRacesBuilder {
    fn build(&self) -> Result<Races> {
        if self.0.len() != 2 {
            return Err(CoreError::general("Need two lists of numbers"));
        }

        let times = &self.0[0];
        let distances = &self.0[1];

        if times.len() != distances.len() {
            return Err(CoreError::general(
                "Times and distances need to be of the same length",
            ));
        }

        let races = times
            .iter()
            .zip(distances.iter())
            .map(|(time, distance)| Race::new(*time, *distance))
            .collect();
        Ok(Races(races))
    }

    fn add_line(&mut self, line: &str) -> Result<()> {
        self.0.push(Self::extract_numbers(line)?);
        Ok(())
    }
}

impl SimpleRacesBuilder {
    fn extract_numbers(line: &str) -> Result<Vec<u64>> {
        let mut scanner = StringScanner::new(line);
        if !scanner.match_string("Time:") {
            scanner.match_string("Distance:");
        }
        let mut numbers = vec![];
        while !scanner.is_finished() {
            scanner.read_whitespace();
            numbers.push(scanner.expect_uint()?);
        }
        Ok(numbers)
    }
}

#[derive(Default)]
struct ConcatRacesBuilder(Vec<String>);

impl RacesBuilder for ConcatRacesBuilder {
    fn build(&self) -> Result<Races> {
        if self.0.len() != 2 {
            return Err(CoreError::general("Need two lines of numbers"));
        }

        let mut numbers = vec![];
        for line in self.0.iter() {
            let num: u64 = line.replace(' ', "").parse()?;
            numbers.push(num);
        }

        let time = numbers[0];
        let distance = numbers[1];
        let race = Race::new(time, distance);
        Ok(Races(vec![race]))
    }

    fn add_line(&mut self, line: &str) -> Result<()> {
        let second_part = match line.split(':').last() {
            Some(x) => x,
            None => {
                return Err(CoreError::general("No ':' found in input string"));
            }
        };
        self.0.push(second_part.to_string());
        Ok(())
    }
}

struct Races(Vec<Race>);

impl Races {
    fn margin_of_error(&self) -> u64 {
        self.0.iter().map(|race| race.num_ways_to_win()).product()
    }
}

struct Race {
    total_time: u64,
    distance_to_beat: u64,
}

impl Race {
    fn new(total_time: u64, distance_to_beat: u64) -> Self {
        Self {
            total_time,
            distance_to_beat,
        }
    }

    fn num_ways_to_win(&self) -> u64 {
        num_ways_to_win(self.total_time, self.distance_to_beat)
    }
}

fn num_ways_to_win(total_time: u64, distance_to_beat: u64) -> u64 {
    for hold_time in 0..total_time {
        if calculate_distance(total_time, hold_time) > distance_to_beat {
            let result = total_time - 2 * hold_time + 1;
            return result;
        }
    }
    0
}

fn calculate_distance(total_time: u64, hold_time: u64) -> u64 {
    if hold_time <= total_time {
        let speed = hold_time;
        let remaining_time = total_time - hold_time;
        speed * remaining_time
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_calculate_distance() {
        assert_eq!(calculate_distance(7, 0), 0);
        assert_eq!(calculate_distance(7, 1), 6);
        assert_eq!(calculate_distance(7, 2), 10);
        assert_eq!(calculate_distance(7, 3), 12);
        assert_eq!(calculate_distance(7, 4), 12);
        assert_eq!(calculate_distance(7, 5), 10);
        assert_eq!(calculate_distance(7, 6), 6);
        assert_eq!(calculate_distance(7, 7), 0);
    }

    #[test]
    fn can_calculate_num_ways_to_win() {
        assert_eq!(num_ways_to_win(7, 9), 4);
        assert_eq!(num_ways_to_win(15, 40), 8);
        assert_eq!(num_ways_to_win(30, 200), 9);
    }

    #[test]
    fn can_calculate_margin_of_error() {
        let races = Races(vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)]);
        assert_eq!(races.margin_of_error(), 288);
    }
}
