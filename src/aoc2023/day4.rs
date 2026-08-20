use std::collections::{HashSet};
use crate::Day;

#[derive(Debug)]
struct BingoCard {
    winning_numbers: HashSet<i32>,
    numbers: Vec<i32>,
}

impl BingoCard {
    pub fn new(winning_numbers: HashSet<i32>, numbers: Vec<i32>) -> Self {
        Self {
            winning_numbers,
            numbers,
        }
    }

    fn count_won_numbers(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count()
    }

    fn count_points(&self) -> i32 {
        match self.count_won_numbers() {
            0 => 0,
            x => 2_i32.pow((x - 1) as u32)
        }
    }

    fn parse_numbers(value: &str) -> impl Iterator<Item = i32> {
        value.split_whitespace().map(|n| n.parse().unwrap())
    }
}

impl From<(&str, &str)> for BingoCard {
    fn from(value: (&str, &str)) -> Self {
        let winning_numbers = Self::parse_numbers(value.0).collect::<HashSet<i32>>();
        let numbers = Self::parse_numbers(value.1).collect::<Vec<i32>>();
        Self::new(winning_numbers, numbers)
    }
}

#[derive(Default, Debug)]
pub struct Day4Of2023 {
    data: Vec<BingoCard>,
}

impl Day for Day4Of2023 {
    fn parse(&mut self, data: String) {
        self.data = data.lines().map(|line| {
            let (_, card) = line.split_once(':').unwrap();
            BingoCard::from(card.split_once('|').unwrap())
        }).collect();
    }

    fn task1(&self) -> String {
        self.data.iter().map(BingoCard::count_points).sum::<i32>().to_string()
    }

    fn task2(&self) -> String {
        self.data
            .iter()
            .enumerate()
            .scan(vec![1usize; self.data.len()], |copies, (index, card)| {
                let current = copies[index];
                let end = (index + card.count_won_numbers() + 1).min(copies.len());

                for next in (index + 1)..end {
                    copies[next] += current;
                }

                Some(current)
            })
            .sum::<usize>()
            .to_string()

    }
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn task_1() {
        let mut day = Day4Of2023::default();
        day.parse(INPUT.to_string());
        assert_eq!(day.task1(), "13");
    }

    #[test]
    fn task_2() {
        let mut day = Day4Of2023::default();
        day.parse(INPUT.to_string());
        assert_eq!(day.task2(), "30");
    }
}
