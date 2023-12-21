use std::ops::Range;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    NorthWest,
    North,
    NorthEast,
    West,
    East,
    SouthWest,
    South,
    SouthEast,
}

impl Direction {
    pub fn all() -> &'static [Self] {
        &[
            Self::NorthWest,
            Self::North,
            Self::NorthEast,
            Self::West,
            Self::East,
            Self::SouthWest,
            Self::South,
            Self::SouthEast,
        ]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Grid(usize, usize);

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self(width, height)
    }

    pub fn width(&self) -> usize {
        self.0
    }

    pub fn height(&self) -> usize {
        self.1
    }

    pub fn to_point(&self, idx: usize) -> Point {
        let width = self.width();
        let x = idx % width;
        let y = idx / width;
        Point { x, y }
    }

    pub fn to_index(&self, point: &Point) -> usize {
        let Point { x, y } = point;
        let width = self.0;
        y * width + x
    }

    pub fn neighbour(&self, idx: usize, direction: Direction) -> Option<usize> {
        let (width, height) = (self.0, self.1);
        let Point { x, y } = self.to_point(idx);
        let max_x = width - 1;
        let max_y = height - 1;
        use Direction::*;

        let (new_x, new_y) = match direction {
            North if y > 0 => (x, y - 1),
            South if y < max_y => (x, y + 1),
            West if x > 0 => (x - 1, y),
            East if x < max_x => (x + 1, y),
            NorthWest if (x > 0 && y > 0) => (x - 1, y - 1),
            NorthEast if (x < max_x && y > 0) => (x + 1, y - 1),
            SouthWest if (x > 0 && y < max_y) => (x - 1, y + 1),
            SouthEast if (x < max_x && y < max_y) => (x + 1, y + 1),
            _ => {
                return None;
            }
        };

        Some(self.to_index(&Point::new(new_x, new_y)))
    }

    pub fn neighbours(&self, idx: usize) -> Vec<usize> {
        Direction::all()
            .iter()
            .filter_map(|direction| self.neighbour(idx, *direction))
            .collect()
    }

    pub fn len(&self) -> usize {
        self.0 * self.1
    }

    pub fn indices(&self) -> Range<usize> {
        0..self.len()
    }

    pub fn positions(&self) -> GridPositionIter {
        GridPositionIter {
            grid: self,
            current: 0,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct GridPosition {
    pub index: usize,
    pub x: usize,
    pub y: usize,
}

impl GridPosition {
    fn new(index: usize, x: usize, y: usize) -> Self {
        Self { index, x, y }
    }
}

pub struct GridPositionIter<'a> {
    grid: &'a Grid,
    current: usize,
}

impl<'a> Iterator for GridPositionIter<'a> {
    type Item = GridPosition;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.grid.len() {
            return None;
        }

        let idx = self.current;
        let Point { x, y } = self.grid.to_point(idx);
        self.current += 1;

        Some(GridPosition::new(idx, x, y))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_get_neighbours() {
        /*
         * 0123
         * 4567
         * 89ab
         */
        let grid = Grid::new(4, 3);
        assert_eq!(grid.neighbours(0), vec![1, 4, 5]);
        assert_eq!(grid.neighbours(5), vec![0, 1, 2, 4, 6, 8, 9, 10]);
        assert_eq!(grid.neighbours(10), vec![5, 6, 7, 9, 11]);
    }

    #[test]
    fn can_iterate_over_positions() {
        /*
         * 0123
         * 4567
         */
        let grid = Grid::new(4, 2);

        assert_eq!(grid.len(), 8);

        let positions: Vec<GridPosition> = grid.positions().collect();
        assert_eq!(
            positions,
            [
                GridPosition::new(0, 0, 0),
                GridPosition::new(1, 1, 0),
                GridPosition::new(2, 2, 0),
                GridPosition::new(3, 3, 0),
                GridPosition::new(4, 0, 1),
                GridPosition::new(5, 1, 1),
                GridPosition::new(6, 2, 1),
                GridPosition::new(7, 3, 1),
            ]
        );
    }
}
