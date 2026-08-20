use crate::common::point::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn all() -> [Direction; 4] {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }

    pub fn delta(&self) -> Point {
        match self {
            Direction::North => Point::new(0, -1),
            Direction::East => Point::new(1, 0),
            Direction::South => Point::new(0, 1),
            Direction::West => Point::new(-1, 0),
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_create_direction_north() {
        let direction = Direction::North;
        assert_eq!(direction, Direction::North);
    }

    #[test]
    fn should_create_direction_east() {
        let direction = Direction::East;
        assert_eq!(direction, Direction::East);
    }
    #[test]
    fn should_create_direction_south() {
        let direction = Direction::South;
        assert_eq!(direction, Direction::South);
    }

    #[test]
    fn should_create_direction_west() {
        let direction = Direction::West;
        assert_eq!(direction, Direction::West);
    }
    #[test]
    fn should_get_delta_for_direction_north() {
        let direction = Direction::North;
        assert_eq!(direction.delta(), Point::new(0, -1));
    }

    #[test]
    fn should_get_delta_for_direction_east() {
        let direction = Direction::East;
        assert_eq!(direction.delta(), Point::new(1, 0));
    }

    #[test]
    fn should_get_delta_for_direction_south() {
        let direction = Direction::South;
        assert_eq!(direction.delta(), Point::new(0, 1));
    }

    #[test]
    fn should_get_delta_for_direction_west() {
        let direction = Direction::West;
        assert_eq!(direction.delta(), Point::new(-1, 0));
    }

    #[test]
    fn should_get_opposite_direction_to_north() {
        let direction = Direction::North;
        assert_eq!(direction.opposite(), Direction::South);
    }

    #[test]
    fn should_get_opposite_direction_to_east() {
        let direction = Direction::East;
        assert_eq!(direction.opposite(), Direction::West);
    }

    #[test]
    fn should_get_opposite_direction_to_south() {
        let direction = Direction::South;
        assert_eq!(direction.opposite(), Direction::North);
    }

    #[test]
    fn should_get_opposite_direction_to_west() {
        let direction = Direction::West;
        assert_eq!(direction.opposite(), Direction::East);
    }

    #[test]
    fn should_get_all_directions() {
        let directions = Direction::all();
        assert_eq!(
            directions,
            [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West
            ]
        );
    }
}
