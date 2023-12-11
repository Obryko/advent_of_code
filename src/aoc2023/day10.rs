use std::fmt::{Debug, Formatter};
use advent_of_code::Day;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl Direction {
    fn move_in_direction(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Direction::N => (x, y - 1),
            Direction::S => (x, y + 1),
            Direction::W => (x - 1, y),
            Direction::E => (x + 1, y),
        }
    }

    fn get_opposite(&self) -> &Direction {
        match self {
            Direction::N => &Direction::S,
            Direction::S => &Direction::N,
            Direction::W => &Direction::E,
            Direction::E => &Direction::W
        }
    }
}

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

    fn from_char(s: &char) -> Pipe {
        match s {
            '|' => Self::new(PipeType::Straight(Direction::N, Direction::S)),
            '-' => Self::new(PipeType::Straight(Direction::W, Direction::E)),
            'J' => Self::new(PipeType::Curved(Direction::N, Direction::W)),
            'L' => Self::new(PipeType::Curved(Direction::N, Direction::E)),
            '7' => Self::new(PipeType::Curved(Direction::S, Direction::W)),
            'F' => Self::new(PipeType::Curved(Direction::S, Direction::E)),
            '.' => Self::new(PipeType::Ground),
            'S' => Self::new(PipeType::Start),
            _ => panic!("Invalid direction")
        }
    }

    fn has_connection(&self, direction: &Direction) -> bool {
        match &self.pipe_type {
            PipeType::Straight(first, second) => first == direction || second == direction,
            PipeType::Curved(first, second) => first == direction || second == direction,
            _ => false
        }
    }
    fn get_connection(&self, direction: &Direction) -> &Direction {
        match &self.pipe_type {
            PipeType::Straight(a, b) if a == direction => b,
            PipeType::Straight(a, b) if b == direction => a,
            PipeType::Curved(a, b) if a == direction => b,
            PipeType::Curved(a, b) if b == direction => a,
            _ => panic!("Cannot move on this type")
        }
    }

    fn get_directions(&self) -> (&Direction, &Direction) {
        match &self.pipe_type {
            PipeType::Straight(a, b) => (a, b),
            PipeType::Curved(a, b) => (a, b),
            _ => panic!("Cannot move on this type")
        }
    }
}

#[derive(Default)]
pub struct Day10Of2023 {
    data: Vec<Vec<Pipe>>,
}

impl Debug for Day10Of2023 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for pipe in row {
                write!(f, "{:?} ", pipe)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Day10Of2023 {
    fn new() -> Self {
        Self {
            data: vec![],
        }
    }

    fn get_start_position(&self) -> (usize, usize) {
        let y = self.data.iter().position(|row| row.iter().any(|pipe| pipe.is_start)).unwrap();
        let x = self.data[y].iter().position(|pipe| pipe.is_start).unwrap();
        (x, y)
    }

    fn get_start_pipe(&self) -> &Pipe {
        let (x, y) = self.get_start_position();
        &self.get_pipe(x, y)
    }

    fn get_pipe(&self, x: usize, y: usize) -> &Pipe {
        &self.data[y][x]
    }

    fn get_pipe_mut(&mut self, x: usize, y: usize) -> &mut Pipe {
        &mut self.data[y][x]
    }

    fn set_start_type(&mut self) {
        let (x, y) = self.get_start_position();
        let top = if y == 0 { false } else { self.get_pipe(x, y - 1).has_connection(&Direction::S)};
        let bottom = if y == self.data.len() - 1 { false } else { self.get_pipe(x, y + 1).has_connection(&Direction::N)};
        let left = if x == 0 { false } else {  self.get_pipe(x - 1, y).has_connection(&Direction::E) };
        let right = if x == self.data[y].len() - 1 { false } else { self.get_pipe(x + 1, y).has_connection(&Direction::W) };
        let pipe_type = match (top, bottom, left, right) {
            (true, true, false, false) => PipeType::Straight(Direction::N, Direction::S),
            (false, false, true, true) => PipeType::Straight(Direction::W, Direction::E),
            (true, false, false, true) => PipeType::Curved(Direction::N, Direction::E),
            (true, false, true, false) => PipeType::Curved(Direction::N, Direction::W),
            (false, true, false, true) => PipeType::Curved(Direction::S, Direction::E),
            (false, true, true, false) => PipeType::Curved(Direction::S, Direction::W),
            _ => panic!("Invalid pipe type")
        };
        let pipe = self.get_pipe_mut(x, y);
        pipe.pipe_type = pipe_type;
    }
}

impl Day for Day10Of2023 {
    fn get_day(&self) -> (i32, i32) {
        (2023, 10)
    }

    fn parse(&mut self, data: String) {
        self.data = data.lines().map(|line| {
            line.chars().map(|c| Pipe::from_char(&c)).collect::<Vec<Pipe>>()
        }).collect::<Vec<Vec<Pipe>>>();
        self.set_start_type();
    }

    fn task1(&self) -> String {

        let (mut x, mut y) = self.get_start_position();
        let mut current_pipe = self.get_pipe(x, y);
        let mut current_direction = current_pipe.get_directions().0;
        let mut steps = 0;

        loop {
            steps += 1;
            current_direction = current_pipe.get_connection(current_direction);
            (x, y) = current_direction.move_in_direction(x, y);
            current_direction = current_direction.get_opposite();
            current_pipe = self.get_pipe(x,y);
            if current_pipe.is_start { break; }
        }

        (steps / 2).to_string()
    }

    fn task2(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";
    const INPUT_2: &str = "-L|F7\n7S-7|\nL|7||\n-L-J|\nL|-JF";
    const INPUT_3: &str = "..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...";
    const INPUT_4: &str = "7-F7-\n.FJ|7\nSJLL7\n|F--J\nLJ.LJ";

    #[test]
    fn task_1_1() {
        let mut day = Day10Of2023::new();
        day.parse(INPUT_1.to_string());
        assert_eq!(day.task1(), "4");
    }

    #[test]
    fn task_1_2() {
        let mut day = Day10Of2023::new();
        day.parse(INPUT_2.to_string());
        assert_eq!(day.task1(), "4");
    }

    #[test]
    fn task_1_3() {
        let mut day = Day10Of2023::new();
        day.parse(INPUT_3.to_string());
        assert_eq!(day.task1(), "8");
    }

    #[test]
    fn task_1_4() {
        let mut day = Day10Of2023::new();
        day.parse(INPUT_4.to_string());
        assert_eq!(day.task1(), "8");
    }

    #[test]
    fn task_2() {
        let mut day = Day10Of2023::new();
        day.parse(INPUT_1.to_string());
        assert_eq!(day.task2(), "");
    }
}
