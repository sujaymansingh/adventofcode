use crate::core::{Result, Solver};
use crate::grid::{Grid, Point};

pub fn part_1() -> Box<dyn Solver> {
    Box::new(Solution(UniverseBuilder::default(), 2))
}

pub fn part_2() -> Box<dyn Solver> {
    Box::new(Solution(UniverseBuilder::default(), 1_000_000))
}

#[derive(Debug)]
struct Solution(UniverseBuilder, usize);

impl Solver for Solution {
    fn handle_line(&mut self, line: &str) -> Result<()> {
        self.0.add_line(line)
    }

    fn extract_solution(&self) -> Result<String> {
        let mut universe = self.0.build()?;
        universe.expand(self.1);
        Ok(universe.sum_of_shortest_paths().to_string())
    }
}

#[derive(Debug)]
struct Universe {
    grid: Grid,
    galaxies: Vec<Point>,
}

impl Universe {
    fn expand(&mut self, factor: usize) {
        let columns: Vec<usize> = (0..self.grid.width())
            .filter(|x| !self.galaxies.iter().any(|p| p.x == *x))
            .collect();
        let rows: Vec<usize> = (0..self.grid.width())
            .filter(|y| !self.galaxies.iter().any(|p| p.y == *y))
            .collect();

        if factor == 0 {
            // ??
            return;
        }

        let delta = factor - 1;

        for x in columns.iter().rev() {
            self.add_column(*x, delta);
        }

        for y in rows.iter().rev() {
            self.add_row(*y, delta);
        }

        let grid = Grid::new(
            (columns.len() * delta) + self.grid.width(),
            (rows.len() * delta) + self.grid.height(),
        );
        self.grid = grid;
    }

    fn add_column(&mut self, x: usize, delta: usize) {
        for galaxy in &mut self.galaxies {
            if galaxy.x >= x {
                galaxy.x += delta;
            }
        }
    }

    fn add_row(&mut self, y: usize, delta: usize) {
        for galaxy in &mut self.galaxies {
            if galaxy.y >= y {
                galaxy.y += delta;
            }
        }
    }

    fn sum_of_shortest_paths(&self) -> usize {
        let mut distance = 0;
        for (i, g1) in self.galaxies.iter().enumerate() {
            let other_galaxies = &self.galaxies[i + 1..];

            for g2 in other_galaxies {
                distance += get_shortest_distance(g1, g2);
            }
        }

        distance
    }
}

fn get_shortest_distance(p1: &Point, p2: &Point) -> usize {
    let width = if p1.x > p2.x {
        p1.x - p2.x
    } else {
        p2.x - p1.x
    };
    let height = if p1.y > p2.y {
        p1.y - p2.y
    } else {
        p2.y - p1.y
    };
    width + height
}

impl ToString for Universe {
    fn to_string(&self) -> String {
        let mut chars: Vec<char> = self.grid.indices().map(|_| '.').collect();
        for point in self.galaxies.iter() {
            let idx = self.grid.to_index(point);
            chars[idx] = '#';
        }
        // wait, newlines!!!
        let mut i = self.grid.len();
        let width = self.grid.width();
        loop {
            chars.insert(i, '\n');
            if i <= width {
                break;
            }
            i -= width;
        }
        chars.iter().collect()
    }
}

#[derive(Debug, Default)]
struct UniverseBuilder {
    width: usize,
    height: usize,
    galaxies: Vec<Point>,
}

impl UniverseBuilder {
    fn add_line(&mut self, line: &str) -> Result<()> {
        self.width = line.len();
        let y = self.height;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                self.galaxies.push(Point { x, y });
            }
        }

        self.height += 1;
        Ok(())
    }

    fn build(&self) -> Result<Universe> {
        let galaxies = self.galaxies.clone();
        let grid = Grid::new(self.width, self.height);

        Ok(Universe { grid, galaxies })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn simple_universe() -> Result<Universe> {
        let lines = [
            "...#......",
            ".......#..",
            "#.........",
            "..........",
            "......#...",
            ".#........",
            ".........#",
            "..........",
            ".......#..",
            "#...#.....",
        ];
        let mut ub = UniverseBuilder::default();
        for line in lines {
            ub.add_line(line)?;
        }

        ub.build()
    }

    fn simple_universe_expanded() -> Result<Universe> {
        let mut universe = simple_universe()?;
        universe.expand(2);
        Ok(universe)
    }

    #[test]
    fn can_build_universe_from_strings() -> Result<()> {
        let univ = simple_universe()?;

        assert_eq!(
            univ.galaxies,
            vec![
                Point { x: 3, y: 0 },
                Point { x: 7, y: 1 },
                Point { x: 0, y: 2 },
                Point { x: 6, y: 4 },
                Point { x: 1, y: 5 },
                Point { x: 9, y: 6 },
                Point { x: 7, y: 8 },
                Point { x: 0, y: 9 },
                Point { x: 4, y: 9 }
            ]
        );

        Ok(())
    }

    #[test]
    fn can_expand_universe() -> Result<()> {
        let mut univ = simple_universe()?;
        univ.expand(2);

        let expected = concat!(
            "....#........\n",
            ".........#...\n",
            "#............\n",
            ".............\n",
            ".............\n",
            "........#....\n",
            ".#...........\n",
            "............#\n",
            ".............\n",
            ".............\n",
            ".........#...\n",
            "#....#.......\n",
        );
        assert_eq!(univ.to_string(), expected);
        Ok(())
    }

    #[test]
    fn to_from_string() -> Result<()> {
        let univ = simple_universe()?;

        let expected = concat!(
            "...#......\n",
            ".......#..\n",
            "#.........\n",
            "..........\n",
            "......#...\n",
            ".#........\n",
            ".........#\n",
            "..........\n",
            ".......#..\n",
            "#...#.....\n",
        );
        assert_eq!(univ.to_string(), expected);
        Ok(())
    }

    #[test]
    fn can_calculate_distances() -> Result<()> {
        let univ = simple_universe_expanded()?;
        assert_eq!(
            get_shortest_distance(&univ.galaxies[0], &univ.galaxies[6]),
            15
        );
        assert_eq!(univ.sum_of_shortest_paths(), 374);
        Ok(())
    }

    #[test]
    fn can_expand_by_custom_factor() -> Result<()> {
        let mut univ = simple_universe()?;
        univ.expand(10);
        assert_eq!(univ.sum_of_shortest_paths(), 1030);
        Ok(())
    }
}
