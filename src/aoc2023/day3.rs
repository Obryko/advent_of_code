use std::collections::HashSet;
use advent_of_code::Day;

#[derive(Debug)]
struct NumberSign(i32, HashSet<char>);

#[derive(Default)]
pub struct Day3Of2023 {
    data: Vec<NumberSign>,
}

fn check_neighbors_in_grid(grid: &Vec<Vec<char>>, row_index: i32, col_index: i32) -> HashSet<char> {
    let mut neighbors = HashSet::new();
    for row in (row_index - 1)..=(row_index + 1) {
        for col in (col_index - 1)..=(col_index + 1) {
            match (row, col) {
                (r, c) if r == row_index && c == col_index => continue,
                (r, c) if r < 0 || c < 0 => continue,
                (r, c) if r as usize > grid.len()-1 || c as usize > grid[0].len()-1 => continue,
                (r,c) => {
                    let value = grid[r as usize][c as usize];
                    if !(value.is_digit(10) || value.eq(&'.')) {
                        neighbors.insert(value);
                    }
                }
            }
        }
    }
    neighbors
}

impl Day for Day3Of2023 {
    fn get_day(&self) -> (i32, i32) {
        (2023, 3)
    }

    fn parse(&mut self, data: String) {
        let grid = data.lines().into_iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let mut res: Vec<NumberSign> = Vec::new();
        for (row_index, row) in grid.iter().enumerate() {
            let mut num = String::new();
            let mut signs = HashSet::new();
            for (col_index, col) in row.iter().enumerate() {
                if !col.is_digit(10) {
                    if !num.is_empty() {
                        res.push(NumberSign(num.parse::<i32>().unwrap(), signs));
                        num = String::new();
                        signs = HashSet::new();
                    }
                    continue;
                }
                let neighbors = check_neighbors_in_grid(&grid, row_index as i32, col_index as i32);
                num.push(*col);
                signs.extend(neighbors);
            }

            if !num.is_empty() {
                res.push(NumberSign(num.parse::<i32>().unwrap(), signs));
            }
        }
        println!("{:?}", res);
        self.data = res;
    }

    fn task1(&self) -> String {
        self.data.iter()
            .filter(|sign| !sign.1.is_empty())
            .map(|sign| sign.0)
            .sum::<i32>().to_string()
    }

    fn task2(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

    #[test]
    fn task_1() {
        let mut day = Day3Of2023::new();
        day.parse(INPUT.to_string());
        assert_eq!(day.task1(), "4361");
    }

    #[test]
    fn task_2() {
        let mut day = Day3Of2023::new();
        day.parse(INPUT.to_string());
        assert_eq!(day.task2(), "");
    }
}
