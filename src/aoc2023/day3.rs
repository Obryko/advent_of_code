use std::collections::HashSet;
use crate::common::grid::Grid;
use crate::common::point::Point;
use crate::Day;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct SignWithPosition{
    sign: char,
    position: Point
}

#[derive(Debug, PartialEq, Eq)]
struct NumberSign(i32, HashSet<SignWithPosition>);

#[derive(Default)]
pub struct Day3Of2023 {
    data: Vec<NumberSign>,
}

fn check_neighbors_in_grid(grid: &Grid<char>, point: Point) -> HashSet<SignWithPosition> {
    let mut neighbors: HashSet<SignWithPosition> = HashSet::new();
    for neighbor in point.neighbors8() {
        let Some(value) = grid.get(neighbor) else {
            continue;
        };
        if !value.is_ascii_digit() && *value != '.' {
            neighbors.insert(SignWithPosition{sign: *value, position: neighbor});
        }
    }
    neighbors
}

impl Day for Day3Of2023 {
    fn parse(&mut self, data: String) {
        let rows = data.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let grid = Grid::try_from(rows.as_slice()).unwrap();
        let mut res: Vec<NumberSign> = Vec::new();
        for (row_index, row) in rows.iter().enumerate() {
            let mut num = String::new();
            let mut signs = HashSet::new();
            for (col_index, col) in row.iter().enumerate() {
                if !col.is_ascii_digit() {
                    if !num.is_empty() {
                        res.push(NumberSign(num.parse::<i32>().unwrap(), signs));
                        num = String::new();
                        signs = HashSet::new();
                    }
                    continue;
                }
                let neighbors = check_neighbors_in_grid(&grid, (col_index as i32, row_index as i32).into());
                num.push(*col);
                signs.extend(neighbors);
            }

            if !num.is_empty() {
                res.push(NumberSign(num.parse::<i32>().unwrap(), signs));
            }
        }
        self.data = res;
    }

    fn task1(&self) -> String {
        self.data.iter()
            .filter(|sign| !sign.1.is_empty())
            .map(|sign| sign.0)
            .sum::<i32>().to_string()
    }

    fn task2(&self) -> String {
        self.data
            .iter()
            .filter(|number_sign| number_sign.1.iter().any(|sign_with_position| sign_with_position.sign == '*'))
            .flat_map(|number_sign| number_sign.1.iter())
            .collect::<HashSet<_>>()
            .iter()
            .map(|sign| {
                let gears = self.data.iter()
                    .filter(|number_sign|
                        number_sign.1.iter().any(|other| sign.position == other.position))
                    .map(|number_sign| number_sign.0).collect::<Vec<i32>>();
                if gears.len() != 2 {
                    return 0;
                }
                gears.iter().product::<i32>()
            }).sum::<i32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

    #[test]
    fn task_1() {
        let mut day = Day3Of2023::default();
        day.parse(INPUT.to_string());
        assert_eq!(day.task1(), "4361");
    }

    #[test]
    fn task_2() {
        let mut day = Day3Of2023::default();
        day.parse(INPUT.to_string());
        assert_eq!(day.task2(), "467835");
    }
}
