use std::fmt::{Debug, Formatter};
use crate::common::direction::Direction;
use crate::common::grid::Grid;
use crate::common::point::Point;
use crate::Day;


#[derive(Debug, PartialEq)]
enum PipeType {
    Straight(Direction, Direction),
    Curved(Direction, Direction),
    Ground,
    Start,
}

struct Pipe {
    pipe_type: PipeType,
    is_start: bool,
}

impl Debug for Pipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.pipe_type {
            PipeType::Straight(first, second) => write!(f, "{:?}-{:?}", first, second),
            PipeType::Curved(first, second) => write!(f, "{:?}-{:?}", first, second),
            PipeType::Ground => write!(f, " . "),
            PipeType::Start => write!(f, " S "),
        }
    }
}

impl Pipe {
    fn new(pipe_type: PipeType) -> Pipe {
        let is_start = pipe_type == PipeType::Start;
        Pipe {
            pipe_type,
            is_start,
        }
    }

    fn from_char(s: char) -> Pipe {
        match s {
            '|' => Self::new(PipeType::Straight(Direction::North, Direction::South)),
            '-' => Self::new(PipeType::Straight(Direction::West, Direction::East)),
            'J' => Self::new(PipeType::Curved(Direction::North, Direction::West)),
            'L' => Self::new(PipeType::Curved(Direction::North, Direction::East)),
            '7' => Self::new(PipeType::Curved(Direction::South, Direction::West)),
            'F' => Self::new(PipeType::Curved(Direction::South, Direction::East)),
            '.' => Self::new(PipeType::Ground),
            'S' => Self::new(PipeType::Start),
            _ => panic!("Invalid direction")
        }
    }

    fn has_connection(&self, direction: Direction) -> bool {
        match &self.pipe_type {
            PipeType::Straight(first, second) | PipeType::Curved(first, second) => *first == direction || *second == direction,
            _ => false
        }
    }
    fn get_connection(&self, direction: Direction) -> Direction {
        match &self.pipe_type {
            PipeType::Straight(a, b) | PipeType::Curved(a, b) if *a == direction => *b,
            PipeType::Straight(a, b) | PipeType::Curved(a, b) if *b == direction => *a,
            _ => panic!("Cannot move on this type")
        }
    }

    fn get_directions(&self) -> (Direction, Direction) {
        match &self.pipe_type {
            PipeType::Straight(a, b) | PipeType::Curved(a, b) => (*a, *b),
            _ => panic!("Cannot move on this type")
        }
    }
}

#[derive(Default)]
pub struct Day10Of2023 {
    data: Grid<Pipe>,
}

impl Debug for Day10Of2023 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl Day10Of2023 {
    fn get_start_position(&self) -> Point {
        self.data.positions().find(|point| self.data.get(*point).is_some_and(|pipe| pipe.is_start)).unwrap()
    }
    fn set_start_type(&mut self) {
        let point = self.get_start_position();
        let has_direction = |direction: Direction| -> bool {
            self.data.get(point + direction.delta())
                .is_some_and(|pipe| pipe.has_connection(direction.opposite()))
        };

        let top = has_direction(Direction::North);
        let bottom = has_direction(Direction::South);
        let left = has_direction(Direction::West);
        let right = has_direction(Direction::East);

        let pipe_type = match (top, bottom, left, right) {
            (true, true, false, false) => PipeType::Straight(Direction::North, Direction::South),
            (false, false, true, true) => PipeType::Straight(Direction::West, Direction::East),
            (true, false, false, true) => PipeType::Curved(Direction::North, Direction::East),
            (true, false, true, false) => PipeType::Curved(Direction::North, Direction::West),
            (false, true, false, true) => PipeType::Curved(Direction::South, Direction::East),
            (false, true, true, false) => PipeType::Curved(Direction::South, Direction::West),
            _ => panic!("Invalid pipe type")
        };
        let pipe = self.data.get_mut(point).unwrap();
        pipe.pipe_type = pipe_type;
    }

    fn get_pipe_polygon(&self) -> Vec<Point> {
        let mut point = self.get_start_position();
        let mut current_pipe = self.data.get(point).unwrap();
        let mut current_direction = current_pipe.get_directions().0;
        let mut vertices: Vec<Point> = Vec::new();

        loop {
            vertices.push(point);
            current_direction = current_pipe.get_connection(current_direction);
            point += current_direction.delta();
            current_direction = current_direction.opposite();
            current_pipe = self.data.get(point).unwrap();
            if current_pipe.is_start { break; }
        }
        vertices
    }
}

fn is_point_in_polygon(point: Point, vertices: &[Point]) -> bool {
    let mut is_inside = false;
    let mut j = vertices.len() - 1;
    for i in 0..vertices.len() {
        if (vertices[i].y > point.y) != (vertices[j].y > point.y) &&
            point.x < (vertices[j].x - vertices[i].x) * (point.y - vertices[i].y) / (vertices[j].y - vertices[i].y) + vertices[i].x {
            is_inside = !is_inside;
        }
        j = i;
    }
    is_inside
}

impl Day for Day10Of2023 {
    fn parse(&mut self, data: String) {
        self.data = data.lines().map(|line| {
            line.chars().map(Pipe::from_char).collect::<Vec<_>>()
        }).collect::<Vec<Vec<_>>>().try_into().unwrap();
        self.set_start_type();
    }

    fn task1(&self) -> String {
        (self.get_pipe_polygon().len() / 2).to_string()
    }

    fn task2(&self) -> String {
        let vertices: Vec<Point> = self.get_pipe_polygon();

        self.data
            .positions()
            .filter(|point| !vertices.contains(point))
            .filter(|&point| is_point_in_polygon(point, &vertices))
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";
    const INPUT_2: &str = "-L|F7\n7S-7|\nL|7||\n-L-J|\nL|-JF";
    const INPUT_3: &str = "..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...";
    const INPUT_4: &str = "7-F7-\n.FJ|7\nSJLL7\n|F--J\nLJ.LJ";
    const INPUT_5: &str = "...........\n.S-------7.\n.|F-----7|.\n.||.....||.\n.||.....||.\n.|L-7.F-J|.\n.|..|.|..|.\n.L--J.L--J.\n...........";

    #[test]
    fn task_1_1() {
        let mut day = Day10Of2023::default();
        day.parse(INPUT_1.to_string());
        assert_eq!(day.task1(), "4");
    }

    #[test]
    fn task_1_2() {
        let mut day = Day10Of2023::default();
        day.parse(INPUT_2.to_string());
        assert_eq!(day.task1(), "4");
    }

    #[test]
    fn task_1_3() {
        let mut day = Day10Of2023::default();
        day.parse(INPUT_3.to_string());
        assert_eq!(day.task1(), "8");
    }

    #[test]
    fn task_1_4() {
        let mut day = Day10Of2023::default();
        day.parse(INPUT_4.to_string());
        assert_eq!(day.task1(), "8");
    }

    #[test]
    fn task_2_1() {
        let mut day = Day10Of2023::default();
        day.parse(INPUT_1.to_string());
        assert_eq!(day.task2(), "1");
    }

    #[test]
    fn task_2_5() {
        let mut day = Day10Of2023::default();
        day.parse(INPUT_5.to_string());
        assert_eq!(day.task2(), "4");
    }

    #[test]
    fn task_2_6() {
        let input = ".F----7F7F7F7F-7....\n.|F--7||||||||FJ....\n.||.FJ||||||||L7....\nFJL7L7LJLJ||LJ.L-7..\nL--J.L7...LJS7F-7L7.\n....F-J..F7FJ|L7L7L7\n....L7.F7||L7|.L7L7|\n.....|FJLJ|FJ|F7|.LJ\n....FJL-7.||.||||...\n....L---J.LJ.LJLJ...";
        let mut day = Day10Of2023::default();
        day.parse(input.to_string());
        assert_eq!(day.task2(), "8");
    }

    #[test]
    fn task_2_7() {
        let input = "FF7FSF7F7F7F7F7F---7\nL|LJ||||||||||||F--J\nFL-7LJLJ||||||LJL-77\nF--JF--7||LJLJ7F7FJ-\nL---JF-JLJ.||-FJLJJ7\n|F|F-JF---7F7-L7L|7|\n|FFJF7L7F-JF7|JL---7\n7-L-JL7||F7|L7F-7F7|\nL.L7LFJ|||||FJL7||LJ\nL7JLJL-JLJLJL--JLJ.L";
        let mut day = Day10Of2023::default();
        day.parse(input.to_string());
        assert_eq!(day.task2(), "10");
    }
}
