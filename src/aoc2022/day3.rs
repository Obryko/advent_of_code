use std::ops::Add;

use advent_of_code::Day;

#[derive(Default)]
pub struct Day3Of2022 {
    data: Vec<(String, String)>,
}

impl Day3Of2022 {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_all_from_rucksack(&self, index: usize) -> String {
        self.data
            .get(index)
            .map(|rucksack| {
                String::new()
                    .add(rucksack.0.as_str())
                    .add(rucksack.1.as_str())
            })
            .unwrap_or_default()
    }
}

fn match_to_value(value: char) -> u32 {
    match value {
        'a'..='z' => (value as u32) - 96,
        'A'..='Z' => (value as u32) - 38,
        _ => 0,
    }
}

impl Day for Day3Of2022 {
    fn get_day(&self) -> (i32, i32) {
        (2022, 3)
    }

    fn parse(&mut self, data: String) {
        let res = data
            .split('\n')
            .map(|rucksack| {
                let (first, second) = rucksack.split_at(rucksack.len() / 2);
                (first.to_string(), second.to_string())
            })
            .collect();
        self.data = res;
    }

    fn task1(&mut self) -> String {
        self.data
            .iter()
            .map(|(first, second)| {
                first
                    .chars()
                    .find(|&char| second.contains(char))
                    .unwrap_or(' ')
            })
            .map(match_to_value)
            .sum::<u32>()
            .to_string()
    }

    fn task2(&mut self) -> String {
        (0..self.data.len())
            .step_by(3)
            .map(|step| {
                let first = self.get_all_from_rucksack(step);
                let second = self.get_all_from_rucksack(step + 1);
                let third = self.get_all_from_rucksack(step + 2);

                first
                    .chars()
                    .find(|&char| second.contains(char) && third.contains(char))
                    .unwrap_or(' ')
            })
            .map(match_to_value)
            .sum::<u32>()
            .to_string()
    }
}
