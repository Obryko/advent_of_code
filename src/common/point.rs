use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    /// Origin point
    pub fn origin() -> Self {
        Self::new(0, 0)
    }

    /// Manhattan distance between two points
    pub fn manhattan_distance(&self, other: Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// 4 Neighbors of a point
    pub fn neighbors4(&self) -> [Self; 4] {
        [
            Self::new(self.x, self.y - 1),
            Self::new(self.x + 1, self.y),
            Self::new(self.x, self.y + 1),
            Self::new(self.x - 1, self.y),
        ]
    }

    /// 8 Neighbors of a point
    pub fn neighbors8(&self) -> [Self; 8] {
        let neighbors4 = self.neighbors4();

        [
            neighbors4[0],
            Self::new(self.x + 1, self.y - 1),
            neighbors4[1],
            Self::new(self.x + 1, self.y + 1),
            neighbors4[2],
            Self::new(self.x - 1, self.y + 1),
            neighbors4[3],
            Self::new(self.x - 1, self.y - 1),
        ]
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Self::new(x as i64, y as i64)
    }
}
impl From<(i64, i64)> for Point {
    fn from((x, y): (i64, i64)) -> Self {
        Self::new(x, y)
    }
}
impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x as i64, y as i64)
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum ConversionError {
    OutOfRange { value: i64, target: &'static str },
}
impl Error for ConversionError {}
impl Display for ConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::OutOfRange { value, target } => {
                write!(f, "cannot convert `{value}` to `{target}`")
            }
        }
    }
}
impl TryFrom<Point> for (usize, usize) {
    type Error = ConversionError;

    fn try_from(point: Point) -> Result<Self, Self::Error> {
        Ok((
            usize::try_from(point.x).map_err(|_| ConversionError::OutOfRange {
                value: point.x,
                target: "usize",
            })?,
            usize::try_from(point.y).map_err(|_| ConversionError::OutOfRange {
                value: point.y,
                target: "usize",
            })?,
        ))
    }
}

impl TryFrom<Point> for (i32, i32) {
    type Error = ConversionError;

    fn try_from(point: Point) -> Result<Self, Self::Error> {
        Ok((
            i32::try_from(point.x).map_err(|_| ConversionError::OutOfRange {
                value: point.x,
                target: "i32",
            })?,
            i32::try_from(point.y).map_err(|_| ConversionError::OutOfRange {
                value: point.y,
                target: "i32",
            })?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_create_point() {
        let point = Point::new(1, 2);
        assert_eq!(point.x, 1);
        assert_eq!(point.y, 2);
    }

    #[test]
    fn should_create_origin_point() {
        let point = Point::origin();
        assert_eq!(point.x, 0);
        assert_eq!(point.y, 0);
    }

    #[test]
    fn should_calculate_manhattan_distance() {
        let point1 = Point::new(1, 2);
        let point2 = Point::new(3, 4);
        assert_eq!(point1.manhattan_distance(point2), 4);
    }
    #[test]
    fn should_calculate_manhattan_distance_from_origin() {
        let point1 = Point::origin();
        let point2 = Point::new(3, 4);
        assert_eq!(point1.manhattan_distance(point2), 7);
    }
    #[test]
    fn should_calculate_manhattan_distance_from_negative() {
        let point1 = Point::new(-3, -4);
        let point2 = Point::new(3, 4);
        assert_eq!(point1.manhattan_distance(point2), 14);
    }

    #[test]
    fn should_calculate_neighbors4_from_origin() {
        let point = Point::origin();
        let neighbors = point.neighbors4();
        let expected_neighbors = [
            Point::new(0, -1),
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(-1, 0),
        ];
        assert_eq!(neighbors, expected_neighbors);
    }

    #[test]
    fn should_calculate_neighbors8_from_origin() {
        let point = Point::origin();
        let neighbors = point.neighbors8();
        let expected_neighbors = [
            Point::new(0, -1),
            Point::new(1, -1),
            Point::new(1, 0),
            Point::new(1, 1),
            Point::new(0, 1),
            Point::new(-1, 1),
            Point::new(-1, 0),
            Point::new(-1, -1),
        ];
        assert_eq!(neighbors, expected_neighbors);
    }
    #[test]
    fn should_calculate_neighbors4_from_other_point() {
        let point = Point::new(5, 4);
        let neighbors = point.neighbors4();
        let expected_neighbors = [
            Point::new(5, 3),
            Point::new(6, 4),
            Point::new(5, 5),
            Point::new(4, 4),
        ];
        assert_eq!(neighbors, expected_neighbors);
    }

    #[test]
    fn should_calculate_neighbors8_from_other_point() {
        let point = Point::new(5, 4);
        let neighbors = point.neighbors8();
        let expected_neighbors = [
            Point::new(5, 3),
            Point::new(6, 3),
            Point::new(6, 4),
            Point::new(6, 5),
            Point::new(5, 5),
            Point::new(4, 5),
            Point::new(4, 4),
            Point::new(4, 3),
        ];
        assert_eq!(neighbors, expected_neighbors);
    }

    #[test]
    fn should_add_points() {
        let point1 = Point::new(1, 2);
        let point2 = Point::new(3, 4);
        assert_eq!(point1 + point2, Point::new(4, 6));
    }

    #[test]
    fn should_subtract_points() {
        let point1 = Point::new(1, 2);
        let point2 = Point::new(3, 4);
        assert_eq!(point1 - point2, Point::new(-2, -2));
    }

    #[test]
    fn should_add_assign_points() {
        let mut point1 = Point::new(1, 2);
        point1 += Point::new(3, 4);
        assert_eq!(point1, Point::new(4, 6));
    }

    #[test]
    fn should_subtract_assign_points() {
        let mut point1 = Point::new(1, 2);
        point1 -= Point::new(3, 4);
        assert_eq!(point1, Point::new(-2, -2));
    }
    #[test]
    fn should_add_assign_points_from_origin() {
        let mut point1 = Point::origin();
        point1 += Point::new(3, 4);
        assert_eq!(point1, Point::new(3, 4));
    }

    #[test]
    fn should_subtract_assign_points_from_origin() {
        let mut point1 = Point::origin();
        point1 -= Point::new(3, 4);
        assert_eq!(point1, Point::new(-3, -4));
    }

    #[test]
    fn should_convert_i32_tuple_to_point() {
        let point: Point = (1, 2).into();
        assert_eq!(point.x, 1);
        assert_eq!(point.y, 2);
    }

    #[test]
    fn should_convert_i64_tuple_to_point() {
        let x: i64 = 1;
        let y: i64 = 2;
        let point: Point = (x, y).into();
        assert_eq!(point.x, 1);
        assert_eq!(point.y, 2);
    }

    #[test]
    fn should_convert_usize_tuple_to_point() {
        let x: usize = 1;
        let y: usize = 2;
        let point: Point = (x, y).into();
        assert_eq!(point.x, 1);
        assert_eq!(point.y, 2);
    }

    #[test]
    fn should_convert_i32_point_to_tuple() {
        let point = Point::new(1, 2);
        let res: Result<(i32, i32), _> = point.try_into();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), (1, 2));
    }
    #[test]
    fn should_throw_error_when_convert_i32_point_to_tuple() {
        let point = Point::new(i32::MAX as i64 + 1, 2);
        let res: Result<(i32, i32), _> = point.try_into();
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "cannot convert `2147483648` to `i32`"
        );
    }

    #[test]
    fn should_convert_usize_point_to_tuple() {
        let point = Point::new(1, 2);
        let res: Result<(usize, usize), _> = point.try_into();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), (1, 2));
    }
    #[test]
    fn should_throw_error_when_convert_usize_point_to_tuple() {
        let point = Point::new(-1, 2);
        let res: Result<(usize, usize), _> = point.try_into();
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "cannot convert `-1` to `usize`"
        );
    }
}
