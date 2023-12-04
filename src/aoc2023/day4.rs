use std::collections::HashMap;
use std::ops::Add;
use advent_of_code::Day;

#[derive(Debug)]
pub struct BingoCard {
    winning_number: Vec<i32>,
    numbers: Vec<i32>,
}

impl BingoCard {
    pub fn new(winning_number: String, numbers: String) -> Self {
        Self {
            winning_number: Self::parse_numbers(winning_number),
            numbers: Self::parse_numbers(numbers),
        }
    }

    fn parse_numbers(numbers: String) -> Vec<i32> {
        numbers.split_whitespace().map(|n| n.parse().unwrap()).collect::<Vec<i32>>()
    }

    fn count_won_numbers(&self) -> usize {
        self.numbers.iter().filter(|n| self.winning_number.contains(n)).collect::<Vec<_>>().len()
    }

    fn count_points(&self) -> i32 {
        match self.count_won_numbers() {
            0 => 0,
            x => 2_i32.pow((x - 1) as u32)
        }
    }
}

#[derive(Default, Debug)]
pub struct Day4Of2023 {
    data: Vec<(usize, BingoCard)>,
}

impl Day for Day4Of2023 {
    fn get_day(&self) -> (i32, i32) {
        (2023, 4)
    }

    fn parse(&mut self, data: String) {

        self.data = data.lines().map(|line| {
            let game = line.split(":").collect::<Vec<&str>>();
            let game_number: usize = game[0].rsplit_once(" ").unwrap().1.parse().unwrap();

            let card = game[1].split("|").collect::<Vec<&str>>();
            let bingo_card: BingoCard = BingoCard::new(card[0].to_string(), card[1].to_string());
            (game_number, bingo_card)
        }).collect::<Vec<(usize, BingoCard)>>();
    }

    fn task1(&self) -> String {
        self.data.iter().map(|(_, card)| card.count_points()).sum::<i32>().to_string()
    }

    fn task2(&self) -> String {
        let mut data = self.data.iter().map(|(game, _)| (*game, 1)).collect::<HashMap<usize, i32>>();
        for (game, card) in self.data.iter() {
            for x in 1..=card.count_won_numbers() {
                let cards = data.get(&(game + x));
                if let Some(value) = cards {
                    data.insert(game + x, value.add(data.get(game).unwrap_or(&1)));
                }
            }
        }
        data.iter().map(|(_, card)| card).sum::<i32>().to_string()
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
        let mut day = Day4Of2023::new();
        day.parse(INPUT.to_string());
        assert_eq!(day.task1(), "13");
    }

    #[test]
    fn task_2() {
        let mut day = Day4Of2023::new();
        day.parse(INPUT.to_string());
        assert_eq!(day.task2(), "30");
    }
}
