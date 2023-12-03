use std::collections::{HashMap, HashSet};
use advent_of_code::Day;

#[derive(Hash, Eq, PartialEq, Debug)]
enum Color {
    RED(i32),
    GREEN(i32),
    BLUE(i32),
}

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

impl Color {
    fn new(color: &str, count: i32) -> Self {
        match color {
            "red" => Color::RED(count),
            "green" => Color::GREEN(count),
            "blue" => Color::BLUE(count),
            _ => panic!("Unknown color: {}", color),
        }
    }

    fn is_possible(&self) -> bool {
        match self {
            Color::GREEN(count) => *count <= MAX_GREEN,
            Color::RED(count) => *count <= MAX_RED,
            Color::BLUE(count) => *count <= MAX_BLUE,
        }
    }
}

#[derive(Default, Debug)]
pub struct Day2Of2023 {
    data: HashMap<i32, Vec<HashSet<Color>>>,
}

impl Day2Of2023 {}

impl Day for Day2Of2023 {
    fn get_day(&self) -> (i32, i32) {
        (2023, 2)
    }

    fn parse(&mut self, data: String) {
        println!("----- Parsing data for a Day {} Year {}-----", self.get_day().1, self.get_day().0);
        self.data = data.lines().map(|line| {
            let game = line.split(":").collect::<Vec<&str>>();
            let game_number: i32 = game[0].rsplit_once(" ").unwrap().1.parse().unwrap();
            let rounds = game[1].split(";").into_iter().map(|round| {
                // round.split(",").collect::<Vec<&str>>().iter().map(|cube| {
                round.split(",").into_iter().map(|cube| {
                    let res = cube.trim().split(" ").collect::<Vec<&str>>();
                    let count = res[0].parse::<i32>().unwrap();
                    Color::new(res[1], count)
                }).collect::<HashSet<Color>>()
            }).collect::<Vec<HashSet<Color>>>();

            (game_number, rounds)
        }
        ).collect::<HashMap<i32, Vec<HashSet<Color>>>>();
    }

    fn task1(&self) -> String {
        self.data.iter().filter(|(_, rounds)|
            rounds.iter().all(|round|
                round.iter().all(|cube| cube.is_possible())
            )
        ).map(|(game, _)| { game }).sum::<i32>().to_string()
    }
    fn task2(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn task_1() {
        let mut day = Day2Of2023::new();
        day.parse(INPUT.to_string());
        assert_eq!(day.task1(), "8");
    }

    #[test]
    fn task_2() {
        let mut day = Day2Of2023::new();
        day.parse(INPUT.to_string());
        assert_eq!(day.task2(), "");
    }
}
