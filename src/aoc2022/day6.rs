use std::collections::HashSet;

use advent_of_code::Day;

#[derive(Default)]
pub struct Day6Of2022 {
    data: Vec<char>,
}

impl Day6Of2022 {
    pub fn new() -> Self {
        Self::default()
    }

    fn find_marker(&self, size: usize) -> String {
        self.data
            .windows(size)
            .enumerate()
            .map(|(index, window)| (index, window.iter().collect::<HashSet<&char>>()))
            .find(|(_, window)| window.len().eq(&size))
            .map(|(index, _)| index + size)
            .unwrap_or(0)
            .to_string()
    }
}

impl Day for Day6Of2022 {
    fn get_day(&self) -> (i32, i32) {
        (2022, 6)
    }

    fn parse(&mut self, data: String) {
        self.data = data.split('\n').collect::<Vec<&str>>()[0].chars().collect();
    }

    fn task1(&mut self) -> String {
        self.find_marker(4)
    }
    fn task2(&mut self) -> String {
        self.find_marker(14)
    }
}
