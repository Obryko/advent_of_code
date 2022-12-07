use async_trait::async_trait;

use advent_of_code::inputs;
use advent_of_code::Day;

pub struct Day1Of2022 {
    data: Vec<i32>,
}

impl Day1Of2022 {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

#[async_trait]
impl Day for Day1Of2022 {
    async fn init(&mut self) {
        let data: String = inputs::get_day_input(2022, 1).await;
        let res: Vec<i32> = data
            .split("\n\n")
            .map(|elf| {
                elf.split('\n')
                    .filter(|x| !x.is_empty())
                    .map(|calories| calories.parse::<i32>().unwrap())
                    .sum::<i32>()
            })
            .collect();
        self.data = res;
    }

    fn task1(&mut self) -> String {
        self.data.iter().max().unwrap_or(&0).to_string()
    }
    fn task2(&mut self) -> String {
        self.data.sort();
        self.data.iter().rev().take(3).sum::<i32>().to_string()
    }
}
