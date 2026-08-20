use crate::common::point::Point;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid<T> {
    cells: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Self {
            cells: Vec::new(),
            width: 0,
            height: 0,
        }
    }
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, value: T) -> Self
    where
        T: Clone,
    {
        Self {
            cells: vec![value; width * height],
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn len(&self) -> usize {
        self.cells.len()
    }
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }
    pub fn contains(&self, point: Point) -> bool {
        self.index(point).is_some()
    }
    pub fn get(&self, point: Point) -> Option<&T> {
        let index = self.index(point)?;
        self.cells.get(index)
    }
    pub fn get_mut(&mut self, point: Point) -> Option<&mut T> {
        let index = self.index(point)?;
        self.cells.get_mut(index)
    }

    pub fn positions(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.height).flat_map(move |y| (0..self.width).map(move |x| (x, y).into()))
    }

    fn index(&self, point: Point) -> Option<usize> {
        let (x, y): (usize, usize) = point.try_into().ok()?;
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(y * self.width + x)
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let point: Point = (x, y).into();
                if let Some(value) = self.get(point) {
                    write!(f, "{}", value)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GridError {
    EmptyRow {
        row: usize,
    },
    InconsistentWidth {
        expected: usize,
        actual: usize,
        row: usize,
    },
}
impl Error for GridError {}

impl Display for GridError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyRow { row } => write!(f, "Row {} is empty.", row),
            Self::InconsistentWidth {
                expected,
                actual,
                row,
            } => write!(
                f,
                "Row {} has inconsistent width. Expected {}, got {}.",
                row, expected, actual
            ),
        }
    }
}

impl<T, const W: usize, const H: usize> From<[[T; W]; H]> for Grid<T> {
    fn from(rows: [[T; W]; H]) -> Self {
        Self {
            width: W,
            height: H,
            cells: rows.into_iter().flatten().collect(),
        }
    }
}

impl<T: Clone> TryFrom<&[Vec<T>]> for Grid<T> {
    type Error = GridError;
    fn try_from(rows: &[Vec<T>]) -> Result<Self, Self::Error> {
        if rows.iter().any(|row| row.is_empty()) {
            return Err(Self::Error::EmptyRow { row: 0 });
        }
        let width = rows[0].len();
        if rows.iter().map(|row| row.len()).any(|len| len != width) {
            return Err(Self::Error::InconsistentWidth {
                expected: width,
                actual: 0,
                row: 0,
            });
        }

        Ok(Self {
            width,
            height: rows.len(),
            cells: rows.into_iter().flatten().cloned().collect(),
        })
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Grid<T> {
    type Error = GridError;
    fn try_from(rows: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        if rows.iter().any(|row| row.is_empty()) {
            return Err(Self::Error::EmptyRow { row: 0 });
        }
        let width = rows[0].len();
        if rows.iter().map(|row| row.len()).any(|len| len != width) {
            return Err(Self::Error::InconsistentWidth {
                expected: width,
                actual: 0,
                row: 0,
            });
        }

        Ok(Self {
            width,
            height: rows.len(),
            cells: rows.into_iter().flatten().collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_new_grid() {
        let grid = Grid::new(10, 10, 0);
        assert_eq!(grid.width(), 10);
        assert_eq!(grid.height(), 10);
        assert_eq!(grid.len(), 100);
    }

    #[test]
    fn should_not_contain_negative_coordinates() {
        let grid = Grid::new(10, 10, 0);

        assert!(!grid.contains(Point::new(-1, 0)));
        assert!(!grid.contains(Point::new(0, -1)));
        assert!(!grid.contains(Point::new(-1, -1)));
    }

    #[test]
    fn should_contain_points_within_bounds() {
        let grid = Grid::new(10, 10, 0);

        assert!(grid.contains(Point::new(0, 0)));
        assert!(grid.contains(Point::new(9, 9)));

        assert!(!grid.contains(Point::new(10, 9)));
        assert!(!grid.contains(Point::new(9, 10)));
    }

    #[test]
    fn should_check_if_grid_contains_point() {
        let grid = Grid::new(10, 10, 0);
        assert!(grid.contains(Point::new(5, 5)));
        assert!(!grid.contains(Point::new(10, 10)));
    }

    #[test]
    fn should_get_value_at_point() {
        let grid = Grid::new(10, 10, 0);
        assert_eq!(grid.get(Point::new(5, 5)), Some(&0));
        assert_eq!(grid.get(Point::new(10, 10)), None);
    }

    #[test]
    fn should_get_mut_value_at_point() {
        let mut grid = Grid::new(10, 10, 0);

        *grid.get_mut(Point::new(5, 5)).unwrap() = 42;

        assert_eq!(grid.get(Point::new(5, 5)), Some(&42));
    }
    #[test]
    fn should_not_get_mut_value_at_point_out_of_bounds() {
        let mut grid = Grid::new(10, 10, 0);

        assert_eq!(grid.get_mut(Point::new(10, 10)), None);
    }

    #[test]
    fn should_return_index_of_point() {
        let grid = Grid::new(10, 10, 0);
        assert_eq!(grid.index(Point::new(5, 5)), Some(55));
        assert_eq!(grid.index(Point::new(10, 10)), None);
    }

    #[test]
    fn should_iterate_over_all_positions() {
        let grid = Grid::new(3, 2, 0);

        let positions = grid.positions().collect::<Vec<_>>();

        assert_eq!(
            positions,
            vec![
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(0, 1),
                Point::new(1, 1),
                Point::new(2, 1),
            ]
        );
    }
}
