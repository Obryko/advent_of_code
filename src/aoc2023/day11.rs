use std::fmt::{Debug, Display, Formatter};
use crate::common::grid::Grid;
use crate::common::point::Point;
use crate::Day;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Space {
    Empty,
    Galaxy(usize),
}

impl Display for Space {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       match self {
            Space::Empty => write!(f, "."),
            Space::Galaxy(i) => write!(f, "{i}"),
        }
    }
}

impl Space {
    fn new(char: char, galaxy_index: usize) -> Self {
        match char {
            '.' => Self::Empty,
            '#' => Self::Galaxy(galaxy_index),
            _ => panic!("Wrong type of space!")
        }
    }
}

#[derive(Debug, Default)]
pub struct Day11Of2023 {
    data: Grid<Space>,
}

impl Display for Day11Of2023 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}


impl Day11Of2023 {

    fn empty_rows(&self) -> Vec<usize> {
        (0..self.data.height())
            .filter(|&y| {
                (0..self.data.width()).all(|x| self.data.get((x, y).into()) == Some(&Space::Empty))
            }).collect::<Vec<usize>>()
    }
    fn empty_columns(&self) -> Vec<usize> {
        (0..self.data.width())
            .filter(|&x| {
                (0..self.data.height()).all(|y| self.data.get((x, y).into()) == Some(&Space::Empty))
            }).collect::<Vec<usize>>()
    }

    fn distance_between(
        &self,
        start: Point,
        end: Point,
        expansion_factor: i64,
    ) -> i64 {
        let distance = start.manhattan_distance(end);
        let min_y = start.y.min(end.y);
        let max_y = start.y.max(end.y);

        let min_x = start.x.min(end.x);
        let max_x = start.x.max(end.x);

        let empty_rows_between = self
            .empty_rows()
            .iter()
            .filter(|&&y| {
                let y = y as i64;
                y > min_y && y < max_y
            })
            .count() as i64;

        let empty_columns_between = self
            .empty_columns()
            .iter()
            .filter(|&&x| {
                let x = x as i64;
                x > min_x && x < max_x
            })
            .count() as i64;
        distance + ((empty_columns_between + empty_rows_between) * (expansion_factor - 1))
    }

    fn solve(&self, expansion_factor: i64) -> i64 {
        let galaxies = self
            .data
            .positions()
            .filter(|&point| {
                matches!(
                self.data.get(point),
                Some(Space::Galaxy(_))
            )
            })
            .collect::<Vec<_>>();

        galaxies
            .iter()
            .enumerate()
            .flat_map(|(i, start)| {
                galaxies
                    .iter()
                    .skip(i + 1)
                    .map(move |end| {
                        self.distance_between(
                            *start,
                            *end,
                            expansion_factor,
                        )
                    })
            })
            .sum()
    }
}


impl Day for Day11Of2023 {
    fn parse(&mut self, data: String) {
        let mut galaxy_index = 1;
        self.data = data.lines()
            .map(|line| line.chars()
                .map(|char| {
                    let space = Space::new(char, galaxy_index);
                    if space == Space::Galaxy(galaxy_index) {
                        galaxy_index += 1;
                    }
                    space
                }).collect::<Vec<Space>>())
            .collect::<Vec<Vec<Space>>>().try_into().unwrap();
    }

    fn task1(&self) -> String {
        self.solve(2).to_string()
    }

    fn task2(&self) -> String {
        self.solve(1_000_000).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";

    #[test]
    fn task_1() {
        let mut day = Day11Of2023::default();
        day.parse(INPUT.to_string());
        assert_eq!(day.task1(), "374");
    }

    #[test]
    fn task_2_10() {
        let mut day = Day11Of2023::default();
        day.parse(INPUT.to_string());
        assert_eq!(day.solve(10), 1030);
    }
    #[test]
    fn task_2_100() {
        let mut day = Day11Of2023::default();
        day.parse(INPUT.to_string());
        assert_eq!(day.solve(100), 8410);
    }
}
