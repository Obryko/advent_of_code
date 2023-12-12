use std::fmt::{Debug, Display, Formatter};

use advent_of_code::Day;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Space {
    Empty,
    Galaxy(usize),
}

impl Display for Space {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Space::Empty => '.'.to_string(),
            Space::Galaxy(i) => i.to_string(),
        })
    }
}

impl Space {
    fn new(char: &char, galaxy_index: usize) -> Self {
        match char {
            '.' => Self::Empty,
            '#' => Self::Galaxy(galaxy_index),
            _ => panic!("Wrong type of space!")
        }
    }
}

#[derive(Default, Debug)]
pub struct Day11Of2023 {
    data: Vec<Vec<Space>>,
}

impl Display for Day11Of2023 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for space in row {
                write!(f, "{} ", space)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


impl Day11Of2023 {
    pub fn new() -> Self {
        Self::default()
    }

    fn expand_rows(&mut self) {
        let indexes = self.data
            .iter()
            .enumerate()
            .filter_map(|(i, row)| {
                match row.iter().all(|space| space == &Space::Empty) {
                    true => Some(i),
                    false => None
                }
            }).collect::<Vec<usize>>();

        for (k, i) in indexes.iter().enumerate() {
            self.data.insert(i + k, vec![Space::Empty; self.data[0].len()]);
        }
    }

    fn expand_columns(&mut self) {
        let indexes = self.data[0]
            .iter()
            .enumerate()
            .filter_map(|(i, _)| {
                match self.data.iter().all(|row| row[i] == Space::Empty) {
                    true => Some(i),
                    false => None
                }
            }).collect::<Vec<usize>>();

        for (k, i) in indexes.iter().enumerate() {
            self.data.iter_mut().for_each(|row| {
                row.insert(i + k, Space::Empty);
            });
        }
    }

    fn expand(&mut self) {
        self.expand_rows();
        self.expand_columns();
    }
}


impl Day for Day11Of2023 {
    fn get_day(&self) -> (i32, i32) {
        (2023, 11)
    }

    fn parse(&mut self, data: String) {
        let mut galaxy_index = 1;
        self.data = data.lines()
            .map(|line| line.chars()
                .map(|char| {
                    let space = Space::new(&char, galaxy_index);
                    if space == Space::Galaxy(galaxy_index) {
                        galaxy_index += 1;
                    }
                    return space;
                }).collect::<Vec<Space>>())
            .collect::<Vec<Vec<Space>>>();
        self.expand();
    }

    fn task1(&self) -> String {
        let galaxies = self.data.iter().enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate()
                    .filter_map(move |(x, space)| {
                        match space {
                            Space::Galaxy(_) => Some((x, y)),
                            _ => None
                        }
                    })
            })
            .collect::<Vec<(usize, usize)>>();


        galaxies
            .iter()
            .enumerate()
            .flat_map(|(i, start)| {
                galaxies.iter().skip(i + 1).map(move |end| end.0.abs_diff(start.0) + end.1.abs_diff(start.1))
            })
            .sum::<usize>()
            .to_string()
    }

    fn task2(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";

    #[test]
    fn task_1() {
        let mut day = Day11Of2023::new();
        day.parse(INPUT.to_string());
        assert_eq!(day.task1(), "374");
    }

    #[test]
    fn task_2() {
        let mut day = Day11Of2023::new();
        day.parse(INPUT.to_string());
        assert_eq!(day.task2(), "");
    }
}
