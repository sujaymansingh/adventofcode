use crate::{
    core::{CoreError, Result, Solver},
    grid::{Direction, Grid},
};

pub fn part_1() -> Box<dyn Solver> {
    Box::new(Solution(MazeBuilder::default(), Part::One))
}

pub fn part_2() -> Box<dyn Solver> {
    Box::new(Solution(MazeBuilder::default(), Part::Two))
}

#[derive(Debug)]
enum Part {
    One,
    Two,
}

#[derive(Debug)]
struct Solution(MazeBuilder, Part);

impl Solver for Solution {
    fn handle_line(&mut self, line: &str) -> Result<()> {
        self.0.add_line(line)?;
        Ok(())
    }

    fn extract_solution(&self) -> Result<String> {
        let maze = self.0.build()?.solve()?;
        let distance = match &self.1 {
            Part::One => maze.max_distance_from_start(),
            Part::Two => maze.num_contained_points(),
        };
        Ok(distance.to_string())
    }
}

#[derive(Debug)]
struct SolvedMaze {
    tiles: Vec<Tile>,
    grid: Grid,
    path: Path,
}

impl SolvedMaze {
    fn max_distance_from_start(&self) -> usize {
        self.path.len() / 2
    }

    fn num_contained_points(&self) -> usize {
        let mut inside = false;
        use Tile::{Ground, NorthEast, NorthWest, Vertical};
        let num = self
            .tiles
            .iter()
            .filter(|tile| match tile {
                Ground => inside,
                Vertical | NorthEast | NorthWest => {
                    inside = !inside;
                    false
                }
                _ => false,
            })
            .count();

        num
    }
}

impl ToString for SolvedMaze {
    fn to_string(&self) -> String {
        tiles_to_string(&self.tiles, self.grid.width())
    }
}

#[derive(Debug)]
struct Maze {
    start_index: usize,
    tiles: Vec<Tile>,
    grid: Grid,
}

impl ToString for Maze {
    fn to_string(&self) -> String {
        tiles_to_string(&self.tiles, self.grid.width())
    }
}

impl Maze {
    fn solve(&self) -> Result<SolvedMaze> {
        let path = self.find_path()?;
        let mut tiles: Vec<Tile> = self.grid.indices().map(|_| Tile::Ground).collect();
        for idx in path.0.iter() {
            tiles[*idx] = self.tiles[*idx];
        }

        Ok(SolvedMaze {
            tiles,
            grid: self.grid.clone(),
            path,
        })
    }

    fn find_path(&self) -> Result<Path> {
        let paths = self.extend_paths_to_convergence()?;
        let head = paths[0].0.iter();

        let start = 1;
        let end = paths[1].0.len() - 1;
        let tail = paths[1].0[start..end].iter().rev();
        let path = Path(head.chain(tail).copied().collect());

        Ok(path)
    }

    fn extend_paths_to_convergence(&self) -> Result<Vec<Path>> {
        let (mut x, mut y) = self.starting_paths()?;

        loop {
            let new_x = x.extend(self)?;
            let new_y = y.extend(self)?;

            if new_x == new_y {
                break;
            }
        }

        Ok(vec![x, y])
    }

    fn starting_paths(&self) -> Result<(Path, Path)> {
        let mut paths: Vec<Path> = self
            .grid
            .neighbours(self.start_index)
            .iter()
            .filter_map(|neighbour_idx| {
                if self.neighbours(*neighbour_idx).contains(&self.start_index) {
                    let path = Path::new(self.start_index, *neighbour_idx);
                    Some(path)
                } else {
                    None
                }
            })
            .collect();

        if paths.len() != 2 {
            let msg = format!("Expected to find exactly two paths but got {:?}", paths);
            Err(CoreError::general(&msg))
        } else {
            let x2 = paths.pop().unwrap();
            let x1 = paths.pop().unwrap();
            Ok((x1, x2))
        }
    }

    fn neighbours(&self, idx: usize) -> Vec<usize> {
        use Direction::*;
        let directions = match self.tiles.get(idx) {
            Some(Tile::Vertical) => vec![North, South],
            Some(Tile::Horizontal) => vec![East, West],
            Some(Tile::NorthEast) => vec![North, East],
            Some(Tile::NorthWest) => vec![North, West],
            Some(Tile::SouthWest) => vec![South, West],
            Some(Tile::SouthEast) => vec![South, East],
            _ => vec![],
        };

        directions
            .iter()
            .flat_map(|direction| self.grid.neighbour(idx, *direction))
            .collect()
    }
}

#[derive(Debug, Default)]
struct MazeBuilder(Vec<String>);

impl MazeBuilder {
    fn add_line(&mut self, line: &str) -> Result<()> {
        self.0.push(line.to_string());
        Ok(())
    }

    fn build(&self) -> Result<Maze> {
        let height = self.0.len();
        let width = self.0[0].len();
        let grid = Grid::new(width, height);

        let mut start_index = None;
        let mut i = 0;
        let mut tiles = vec![];

        for line in self.0.iter() {
            for c in line.chars() {
                let tile = Tile::from_char(c)?;
                tiles.push(tile);
                if tile == Tile::Start {
                    start_index = Some(i);
                }
                i += 1;
            }
        }

        let start_index = match start_index {
            Some(x) => x,
            None => {
                return Err(CoreError::general("No start tile found"));
            }
        };

        let start_tile = calculate_start_tile(&tiles, start_index, &grid)?;
        tiles[start_index] = start_tile;

        Ok(Maze {
            tiles,
            start_index,
            grid,
        })
    }
}

#[derive(Debug, Eq, Clone, Copy, PartialEq)]
enum Tile {
    Ground,
    Vertical,
    Horizontal,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
    Start,
}

impl Tile {
    fn from_char(c: char) -> Result<Self> {
        let tile = match c {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => {
                return Err(CoreError::general(&format!(
                    "'{}' is not a valid char for a tile",
                    c
                )))
            }
        };
        Ok(tile)
    }

    fn to_display_char(self) -> char {
        match self {
            Self::Vertical => '\u{2503}',
            Self::Horizontal => '\u{2501}',
            Self::NorthWest => '\u{251b}',
            Self::NorthEast => '\u{2517}',
            Self::SouthWest => '\u{2513}',
            Self::SouthEast => '\u{250f}',
            Self::Ground => '.',
            Self::Start => 'S',
        }
    }
}

fn calculate_start_tile(tiles: &[Tile], start_index: usize, grid: &Grid) -> Result<Tile> {
    let compass_directions = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
    let neighbours: Vec<Tile> = compass_directions
        .iter()
        .map(|dir| {
            if let Some(neighbour_idx) = grid.neighbour(start_index, *dir) {
                let neighbour = tiles.get(neighbour_idx).unwrap_or(&Tile::Ground);
                *neighbour
            } else {
                Tile::Ground
            }
        })
        .collect();

    let north = neighbours[0];
    let east = neighbours[1];
    let south = neighbours[2];
    let west = neighbours[3];

    let north_conn =
        north == Tile::Vertical || north == Tile::SouthWest || north == Tile::SouthEast;
    let east_conn = east == Tile::Horizontal || east == Tile::NorthWest || east == Tile::SouthWest;
    let south_conn =
        south == Tile::Vertical || south == Tile::NorthWest || south == Tile::NorthEast;
    let west_conn = west == Tile::Horizontal || west == Tile::NorthEast || west == Tile::SouthEast;

    let tile = match (north_conn, east_conn, south_conn, west_conn) {
        (true, true, false, false) => Tile::NorthEast,
        (true, false, true, false) => Tile::Vertical,
        (true, false, false, true) => Tile::NorthWest,
        (false, true, true, false) => Tile::SouthEast,
        (false, true, false, true) => Tile::Horizontal,
        (false, false, true, true) => Tile::SouthWest,
        _ => {
            return Err(CoreError::general("Couldn't work out start tile"));
        }
    };

    Ok(tile)
}

fn tiles_to_string(tiles: &[Tile], width: usize) -> String {
    tiles
        .iter()
        .enumerate()
        .flat_map(|(i, tile)| {
            let c = tile.to_display_char();
            if (i + 1) % width == 0 {
                vec![c, '\n']
            } else {
                vec![c]
            }
        })
        .collect()
}

#[derive(Debug, Default, Clone)]
struct Path(Vec<usize>);

impl Path {
    fn new(start: usize, end: usize) -> Self {
        Self(vec![start, end])
    }

    fn last_two(&self) -> (usize, usize) {
        let max_idx = self.len() - 1;
        let x1 = self.0.get(max_idx - 1).unwrap();
        let x2 = self.0.get(max_idx).unwrap();
        (*x1, *x2)
    }

    fn extend(&mut self, maze: &Maze) -> Result<usize> {
        let (second_last, last) = self.last_two();
        if let Some(next) = maze
            .neighbours(last)
            .iter()
            .find(|neighbour| **neighbour != second_last)
        {
            self.0.push(*next);
            Ok(*next)
        } else {
            Err(CoreError::general("Couldn't find next element"))
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_calculate_start_tile() {
        #[rustfmt::skip]
        let sample = vec![
            Tile::Ground, Tile::Ground, Tile::Ground,
            Tile::Ground, Tile::Start, Tile::Horizontal,
            Tile::Ground, Tile::Vertical, Tile::Ground,
        ];
        let grid = Grid::new(3, 3);

        assert_eq!(
            calculate_start_tile(&sample, 4, &grid).unwrap(),
            Tile::SouthEast
        );
    }

    fn maze(lines: &[&str]) -> Maze {
        let mut mb = MazeBuilder(vec![]);
        for line in lines {
            mb.add_line(line).unwrap();
        }
        mb.build().unwrap()
    }

    fn simple_maze() -> Maze {
        let lines = [".....", ".S-7.", ".|.|.", ".L-J.", "....."];
        maze(&lines)
    }

    fn complex_maze() -> Maze {
        let lines = ["7-F7-", ".FJ|7", "SJLL7", "|F--J", "LJ.LJ"];
        maze(&lines)
    }

    fn very_complex_maze() -> Maze {
        let lines = [
            ".F----7F7F7F7F-7....",
            ".|F--7||||||||FJ....",
            ".||.FJ||||||||L7....",
            "FJL7L7LJLJ||LJ.L-7..",
            "L--J.L7...LJS7F-7L7.",
            "....F-J..F7FJ|L7L7L7",
            "....L7.F7||L7|.L7L7|",
            ".....|FJLJ|FJ|F7|.LJ",
            "....FJL-7.||.||||...",
            "....L---J.LJ.LJLJ...",
        ];
        maze(&lines)
    }

    #[test]
    fn can_build_maze_from_lines() {
        let maze = simple_maze();
        assert_eq!(maze.grid.width(), 5);
        assert_eq!(maze.grid.height(), 5);
        assert_eq!(maze.start_index, 6);
        let expected: Vec<Tile> = "......F-7..|.|..L-J......"
            .chars()
            .map(|c| Tile::from_char(c).unwrap())
            .collect();
        assert_eq!(maze.tiles, expected);
    }

    #[test]
    fn can_solve_maze() {
        let maze = simple_maze().solve().unwrap();
        assert_eq!(maze.path.0, vec![6, 7, 8, 13, 18, 17, 16, 11]);

        let maze = complex_maze().solve().unwrap();
        assert_eq!(
            maze.path.0,
            vec![10, 11, 6, 7, 2, 3, 8, 13, 14, 19, 18, 17, 16, 21, 20, 15]
        );
    }

    #[test]
    fn solved_maze_has_no_nonloop_pipes() {
        let maze = complex_maze().solve().unwrap();
        let expected: Vec<Tile> = "..F7..FJ|.FJ.L7|F--JLJ..."
            .chars()
            .map(|c| Tile::from_char(c).unwrap())
            .collect();
        assert_eq!(maze.tiles, expected);
    }

    #[test]
    fn can_count_num_contained_points() -> Result<()> {
        assert_eq!(1, simple_maze().solve()?.num_contained_points());
        assert_eq!(1, complex_maze().solve()?.num_contained_points());
        assert_eq!(8, very_complex_maze().solve()?.num_contained_points());

        Ok(())
    }
}
