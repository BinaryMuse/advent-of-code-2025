use std::fmt;

/// A coordinate in a grid (row, col)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub row: isize,
    pub col: isize,
}

impl Coord {
    pub fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    /// Move in a direction by the given number of steps
    pub fn step<D: Direction>(&self, direction: D, steps: isize) -> Self {
        let (dr, dc) = direction.delta();
        Self {
            row: self.row + dr * steps,
            col: self.col + dc * steps,
        }
    }

    /// Convert to (usize, usize) if both coordinates are non-negative
    pub fn as_unsigned(&self) -> Option<(usize, usize)> {
        if self.row >= 0 && self.col >= 0 {
            Some((self.row as usize, self.col as usize))
        } else {
            None
        }
    }
}

impl From<(isize, isize)> for Coord {
    fn from((row, col): (isize, isize)) -> Self {
        Self { row, col }
    }
}

impl From<(usize, usize)> for Coord {
    fn from((row, col): (usize, usize)) -> Self {
        Self {
            row: row as isize,
            col: col as isize,
        }
    }
}

impl From<(i32, i32)> for Coord {
    fn from((row, col): (i32, i32)) -> Self {
        Self {
            row: row as isize,
            col: col as isize,
        }
    }
}

/// Relative direction for turning
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relative {
    Left,
    Right,
    Back,
}

/// Common trait for direction types
pub trait Direction: Copy + Eq {
    /// Get the (row_delta, col_delta) for this direction
    /// North is negative row, East is positive col
    fn delta(&self) -> (isize, isize);

    /// Turn in a relative direction by the given number of steps
    fn turn(&self, relative: Relative, steps: usize) -> Self;

    /// Get the opposite direction
    fn opposite(&self) -> Self {
        self.turn(Relative::Back, 1)
    }
}

/// 4-cardinal directions (N, E, S, W)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction4 {
    North,
    East,
    South,
    West,
}

impl Direction4 {
    /// All directions in clockwise order starting from North
    pub const ALL: [Direction4; 4] = [
        Direction4::North,
        Direction4::East,
        Direction4::South,
        Direction4::West,
    ];
}

impl Direction for Direction4 {
    fn delta(&self) -> (isize, isize) {
        match self {
            Direction4::North => (-1, 0),
            Direction4::East => (0, 1),
            Direction4::South => (1, 0),
            Direction4::West => (0, -1),
        }
    }

    fn turn(&self, relative: Relative, steps: usize) -> Self {
        let current_idx = Self::ALL.iter().position(|d| d == self).unwrap();
        let offset = match relative {
            Relative::Right => steps,
            Relative::Left => 4 - (steps % 4),
            Relative::Back => 2,
        };
        Self::ALL[(current_idx + offset) % 4]
    }
}

/// 8-cardinal directions (N, NE, E, SE, S, SW, W, NW)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction8 {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction8 {
    /// All directions in clockwise order starting from North
    pub const ALL: [Direction8; 8] = [
        Direction8::North,
        Direction8::NorthEast,
        Direction8::East,
        Direction8::SouthEast,
        Direction8::South,
        Direction8::SouthWest,
        Direction8::West,
        Direction8::NorthWest,
    ];

    /// Convert to Direction4 if this is a cardinal direction
    pub fn to_direction4(&self) -> Option<Direction4> {
        match self {
            Direction8::North => Some(Direction4::North),
            Direction8::East => Some(Direction4::East),
            Direction8::South => Some(Direction4::South),
            Direction8::West => Some(Direction4::West),
            _ => None,
        }
    }
}

impl Direction for Direction8 {
    fn delta(&self) -> (isize, isize) {
        match self {
            Direction8::North => (-1, 0),
            Direction8::NorthEast => (-1, 1),
            Direction8::East => (0, 1),
            Direction8::SouthEast => (1, 1),
            Direction8::South => (1, 0),
            Direction8::SouthWest => (1, -1),
            Direction8::West => (0, -1),
            Direction8::NorthWest => (-1, -1),
        }
    }

    fn turn(&self, relative: Relative, steps: usize) -> Self {
        let current_idx = Self::ALL.iter().position(|d| d == self).unwrap();
        let offset = match relative {
            Relative::Right => steps,
            Relative::Left => 8 - (steps % 8),
            Relative::Back => 4,
        };
        Self::ALL[(current_idx + offset) % 8]
    }
}

impl From<Direction4> for Direction8 {
    fn from(d: Direction4) -> Self {
        match d {
            Direction4::North => Direction8::North,
            Direction4::East => Direction8::East,
            Direction4::South => Direction8::South,
            Direction4::West => Direction8::West,
        }
    }
}

/// A 2D grid with optional cell contents
pub struct Grid<T> {
    width: usize,
    height: usize,
    cells: Vec<Option<T>>,
}

impl<T> Grid<T> {
    /// Create a new grid with all cells set to None
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: (0..width * height).map(|_| None).collect(),
        }
    }

    /// Create a grid from a 2D vector (row-major order)
    pub fn from_vec(data: Vec<Vec<T>>) -> Self {
        let height = data.len();
        let width = data.first().map(|row| row.len()).unwrap_or(0);
        let cells = data
            .into_iter()
            .flat_map(|row| row.into_iter().map(Some))
            .collect();
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// Convert (row, col) to linear index
    fn index(&self, row: usize, col: usize) -> Option<usize> {
        if row < self.height && col < self.width {
            Some(row * self.width + col)
        } else {
            None
        }
    }

    /// Check if a coordinate is within bounds
    pub fn in_bounds(&self, coord: Coord) -> bool {
        coord.row >= 0
            && coord.col >= 0
            && (coord.row as usize) < self.height
            && (coord.col as usize) < self.width
    }

    /// Get a reference to the cell contents at (row, col)
    pub fn get(&self, coord: impl Into<Coord>) -> Option<&T> {
        let coord = coord.into();
        let (row, col) = coord.as_unsigned()?;
        self.index(row, col)
            .and_then(|idx| self.cells[idx].as_ref())
    }

    /// Get a mutable reference to the cell contents at (row, col)
    pub fn get_mut(&mut self, coord: impl Into<Coord>) -> Option<&mut T> {
        let coord = coord.into();
        let (row, col) = coord.as_unsigned()?;
        self.index(row, col)
            .and_then(|idx| self.cells[idx].as_mut())
    }

    /// Set the cell contents at (row, col), returning the old value
    pub fn set(&mut self, coord: impl Into<Coord>, value: T) -> Option<T> {
        let coord = coord.into();
        let (row, col) = coord.as_unsigned()?;
        let idx = self.index(row, col)?;
        self.cells[idx].replace(value)
    }

    /// Take the value from a cell, leaving None in its place
    pub fn take(&mut self, coord: impl Into<Coord>) -> Option<T> {
        let coord = coord.into();
        let (row, col) = coord.as_unsigned()?;
        let idx = self.index(row, col)?;
        self.cells[idx].take()
    }

    /// Clear a cell, setting it to None
    pub fn clear(&mut self, coord: impl Into<Coord>) -> Option<T> {
        self.take(coord)
    }

    /// Swap the contents of two cells
    pub fn swap(&mut self, a: impl Into<Coord>, b: impl Into<Coord>) -> bool {
        let a = a.into();
        let b = b.into();
        let (row_a, col_a) = match a.as_unsigned() {
            Some(v) => v,
            None => return false,
        };
        let (row_b, col_b) = match b.as_unsigned() {
            Some(v) => v,
            None => return false,
        };

        let idx_a = match self.index(row_a, col_a) {
            Some(idx) => idx,
            None => return false,
        };
        let idx_b = match self.index(row_b, col_b) {
            Some(idx) => idx,
            None => return false,
        };

        self.cells.swap(idx_a, idx_b);
        true
    }

    /// Move the contents from one cell to another, returning what was at the destination
    pub fn move_to(&mut self, from: impl Into<Coord>, to: impl Into<Coord>) -> Option<T> {
        let from = from.into();
        let to = to.into();
        let value = self.take(from)?;
        self.set(to, value)
    }

    /// Iterate over all coordinates in row-major order
    pub fn coords(&self) -> impl Iterator<Item = Coord> {
        let width = self.width;
        let height = self.height;
        (0..height)
            .flat_map(move |row| (0..width).map(move |col| Coord::new(row as isize, col as isize)))
    }

    /// Iterate over all cells (row-major order)
    pub fn iter(&self) -> std::slice::Iter<'_, Option<T>> {
        self.cells.iter()
    }

    /// Mutably iterate over all cells (row-major order)
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Option<T>> {
        self.cells.iter_mut()
    }

    /// Iterate over all cells with Some value and their coordinates
    pub fn iter_filled(&self) -> impl Iterator<Item = (Coord, &T)> {
        self.enumerate()
            .filter_map(|(coord, cell)| cell.as_ref().map(|v| (coord, v)))
    }

    /// Get a coordinate in a direction, if it's in bounds
    pub fn coord_in_dir<D: Direction>(
        &self,
        coord: impl Into<Coord>,
        dir: D,
        steps: isize,
    ) -> Option<Coord> {
        let new_coord = coord.into().step(dir, steps);
        if self.in_bounds(new_coord) {
            Some(new_coord)
        } else {
            None
        }
    }

    /// Get all 4-cardinal neighbors of a coordinate that are in bounds
    pub fn neighbors4(&self, coord: impl Into<Coord>) -> impl Iterator<Item = Coord> + '_ {
        let coord = coord.into();
        Direction4::ALL
            .iter()
            .filter_map(move |dir| self.coord_in_dir(coord, *dir, 1))
    }

    /// Get all 8-cardinal neighbors of a coordinate that are in bounds
    pub fn neighbors8(&self, coord: impl Into<Coord>) -> impl Iterator<Item = Coord> + '_ {
        let coord = coord.into();
        Direction8::ALL
            .iter()
            .filter_map(move |dir| self.coord_in_dir(coord, *dir, 1))
    }
}

impl<T: Clone> Grid<T> {
    /// Create a grid filled with a default value
    pub fn filled(width: usize, height: usize, value: T) -> Self {
        Self {
            width,
            height,
            cells: vec![Some(value); width * height],
        }
    }
}

impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
            cells: self.cells.clone(),
        }
    }
}

/// Iterator over grid cells with their coordinates
pub struct GridEnumerate<'a, T> {
    grid: &'a Grid<T>,
    index: usize,
}

impl<'a, T> Iterator for GridEnumerate<'a, T> {
    type Item = (Coord, &'a Option<T>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.grid.cells.len() {
            return None;
        }
        let row = self.index / self.grid.width;
        let col = self.index % self.grid.width;
        let coord = Coord::new(row as isize, col as isize);
        let cell = &self.grid.cells[self.index];
        self.index += 1;
        Some((coord, cell))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.grid.cells.len() - self.index;
        (remaining, Some(remaining))
    }
}

impl<'a, T> ExactSizeIterator for GridEnumerate<'a, T> {}

/// Mutable iterator over grid cells with their coordinates
pub struct GridEnumerateMut<'a, T> {
    width: usize,
    cells: std::slice::IterMut<'a, Option<T>>,
    index: usize,
}

impl<'a, T> Iterator for GridEnumerateMut<'a, T> {
    type Item = (Coord, &'a mut Option<T>);

    fn next(&mut self) -> Option<Self::Item> {
        let cell = self.cells.next()?;
        let row = self.index / self.width;
        let col = self.index % self.width;
        let coord = Coord::new(row as isize, col as isize);
        self.index += 1;
        Some((coord, cell))
    }
}

impl<T> Grid<T> {
    /// Iterate over all cells with their coordinates (row-major order)
    pub fn enumerate(&self) -> GridEnumerate<'_, T> {
        GridEnumerate {
            grid: self,
            index: 0,
        }
    }

    /// Mutably iterate over all cells with their coordinates (row-major order)
    pub fn enumerate_mut(&mut self) -> GridEnumerateMut<'_, T> {
        GridEnumerateMut {
            width: self.width,
            cells: self.cells.iter_mut(),
            index: 0,
        }
    }
}

/// Iterate over references to cell contents
impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = &'a Option<T>;
    type IntoIter = std::slice::Iter<'a, Option<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter()
    }
}

/// Iterate over mutable references to cell contents
impl<'a, T> IntoIterator for &'a mut Grid<T> {
    type Item = &'a mut Option<T>;
    type IntoIter = std::slice::IterMut<'a, Option<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter_mut()
    }
}

/// Consume grid and iterate over cell contents
impl<T> IntoIterator for Grid<T> {
    type Item = Option<T>;
    type IntoIter = std::vec::IntoIter<Option<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.into_iter()
    }
}

impl<T: fmt::Debug> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Grid {}x{} {{", self.width, self.height)?;
        for row in 0..self.height {
            write!(f, "  ")?;
            for col in 0..self.width {
                let idx = row * self.width + col;
                match &self.cells[idx] {
                    Some(v) => write!(f, "{:?} ", v)?,
                    None => write!(f, ". ")?,
                }
            }
            writeln!(f)?;
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_step_dir4() {
        let coord = Coord::new(5, 5);
        assert_eq!(coord.step(Direction4::North, 2), Coord::new(3, 5));
        assert_eq!(coord.step(Direction4::South, 3), Coord::new(8, 5));
        assert_eq!(coord.step(Direction4::East, 1), Coord::new(5, 6));
        assert_eq!(coord.step(Direction4::West, 4), Coord::new(5, 1));
    }

    #[test]
    fn test_coord_step_dir8() {
        let coord = Coord::new(5, 5);
        assert_eq!(coord.step(Direction8::NorthEast, 2), Coord::new(3, 7));
        assert_eq!(coord.step(Direction8::SouthWest, 3), Coord::new(8, 2));
    }

    #[test]
    fn test_direction4_turn() {
        assert_eq!(Direction4::North.turn(Relative::Right, 1), Direction4::East);
        assert_eq!(Direction4::North.turn(Relative::Left, 1), Direction4::West);
        assert_eq!(Direction4::North.turn(Relative::Back, 1), Direction4::South);
        assert_eq!(Direction4::East.turn(Relative::Right, 2), Direction4::West);
        assert_eq!(Direction4::South.turn(Relative::Left, 3), Direction4::West);
    }

    #[test]
    fn test_direction8_turn() {
        assert_eq!(
            Direction8::East.turn(Relative::Right, 1),
            Direction8::SouthEast
        );
        assert_eq!(Direction8::East.turn(Relative::Right, 2), Direction8::South);
        assert_eq!(
            Direction8::North.turn(Relative::Left, 1),
            Direction8::NorthWest
        );
        assert_eq!(Direction8::North.turn(Relative::Back, 1), Direction8::South);
    }

    #[test]
    fn test_grid_basic() {
        let mut grid: Grid<char> = Grid::new(3, 3);
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 3);

        grid.set((0, 0), 'A');
        grid.set((1, 1), 'B');
        grid.set((2, 2), 'C');

        assert_eq!(grid.get((0, 0)), Some(&'A'));
        assert_eq!(grid.get((1, 1)), Some(&'B'));
        assert_eq!(grid.get((2, 2)), Some(&'C'));
        assert_eq!(grid.get((0, 1)), None);
    }

    #[test]
    fn test_grid_swap() {
        let mut grid: Grid<char> = Grid::new(3, 3);
        grid.set((0, 0), 'A');
        grid.set((1, 1), 'B');

        grid.swap((0, 0), (1, 1));

        assert_eq!(grid.get((0, 0)), Some(&'B'));
        assert_eq!(grid.get((1, 1)), Some(&'A'));
    }

    #[test]
    fn test_grid_move_to() {
        let mut grid: Grid<char> = Grid::new(3, 3);
        grid.set((0, 0), 'A');
        grid.set((1, 1), 'B');

        let replaced = grid.move_to((0, 0), (1, 1));
        assert_eq!(replaced, Some('B'));
        assert_eq!(grid.get((0, 0)), None);
        assert_eq!(grid.get((1, 1)), Some(&'A'));
    }

    #[test]
    fn test_grid_neighbors() {
        let grid: Grid<char> = Grid::new(3, 3);

        let neighbors: Vec<_> = grid.neighbors4((1, 1)).collect();
        assert_eq!(neighbors.len(), 4);

        let corner_neighbors: Vec<_> = grid.neighbors4((0, 0)).collect();
        assert_eq!(corner_neighbors.len(), 2);

        let neighbors8: Vec<_> = grid.neighbors8((1, 1)).collect();
        assert_eq!(neighbors8.len(), 8);

        let corner_neighbors8: Vec<_> = grid.neighbors8((0, 0)).collect();
        assert_eq!(corner_neighbors8.len(), 3);
    }

    #[test]
    fn test_grid_from_vec() {
        let data = vec![vec!['A', 'B', 'C'], vec!['D', 'E', 'F']];
        let grid = Grid::from_vec(data);

        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 2);
        assert_eq!(grid.get((0, 0)), Some(&'A'));
        assert_eq!(grid.get((1, 2)), Some(&'F'));
    }

    #[test]
    fn test_coord_from_tuple() {
        let coord: Coord = (5usize, 10usize).into();
        assert_eq!(coord.row, 5);
        assert_eq!(coord.col, 10);

        let coord: Coord = (5isize, 10isize).into();
        assert_eq!(coord.row, 5);
        assert_eq!(coord.col, 10);
    }

    #[test]
    fn test_grid_iterate() {
        let data = vec![vec!['A', 'B'], vec!['C', 'D']];
        let grid = Grid::from_vec(data);

        // Test for loop iteration
        let chars: Vec<_> = (&grid).into_iter().filter_map(|c| c.as_ref()).collect();
        assert_eq!(chars, vec![&'A', &'B', &'C', &'D']);
    }

    #[test]
    fn test_grid_enumerate() {
        let data = vec![vec!['A', 'B'], vec!['C', 'D']];
        let grid = Grid::from_vec(data);

        let items: Vec<_> = grid.enumerate().collect();
        assert_eq!(items.len(), 4);
        assert_eq!(items[0], (Coord::new(0, 0), &Some('A')));
        assert_eq!(items[1], (Coord::new(0, 1), &Some('B')));
        assert_eq!(items[2], (Coord::new(1, 0), &Some('C')));
        assert_eq!(items[3], (Coord::new(1, 1), &Some('D')));
    }

    #[test]
    fn test_grid_enumerate_mut() {
        let data = vec![vec![1, 2], vec![3, 4]];
        let mut grid = Grid::from_vec(data);

        for (coord, cell) in grid.enumerate_mut() {
            if coord.row == coord.col {
                *cell = cell.map(|v| v * 10);
            }
        }

        assert_eq!(grid.get((0, 0)), Some(&10));
        assert_eq!(grid.get((0, 1)), Some(&2));
        assert_eq!(grid.get((1, 0)), Some(&3));
        assert_eq!(grid.get((1, 1)), Some(&40));
    }

    #[test]
    fn test_grid_into_iter() {
        let data = vec![vec!['X', 'Y']];
        let grid = Grid::from_vec(data);

        let chars: Vec<_> = grid.into_iter().flatten().collect();
        assert_eq!(chars, vec!['X', 'Y']);
    }
}
