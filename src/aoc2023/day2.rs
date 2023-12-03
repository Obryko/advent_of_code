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

    fn get_value(&self) -> i32 {
        match self {
            Color::RED(count) => *count,
            Color::GREEN(count) => *count,
            Color::BLUE(count) => *count,
        }
    }

    fn is_possible(&self) -> bool {
        match self {
            Color::GREEN(count) => *count <= MAX_GREEN,
            Color::RED(count) => *count <= MAX_RED,
            Color::BLUE(count) => *count <= MAX_BLUE,
        }
    }

    fn eq_to_string(&self, color: &str) -> bool {
        match self {
            Color::RED(_) => color == "red",
            Color::GREEN(_) => color == "green",
            Color::BLUE(_) => color == "blue",
        }
    }
}

#[derive(Default, Debug)]
pub struct Day2Of2023 {
    data: HashMap<i32, Vec<HashSet<Color>>>,
}

impl Day2Of2023 {
    fn get_max_in_round(rounds: &Vec<HashSet<Color>>, color: &str) -> i32 {
        rounds.iter()
            .map(|round| {
                let res = round.iter().find(|&cube| cube.eq_to_string(color));
                match res {
                    Some(cube) => cube.get_value(),
                    None => 1,
                }
            }).max().unwrap_or(1)
    }
}

impl Day for Day2Of2023 {
    fn get_day(&self) -> (i32, i32) {
        (2023, 2)
    }

    fn parse(&mut self, data: String) {
        self.data = data.lines().map(|line| {
            let game = line.split(":").collect::<Vec<&str>>();
            let game_number: i32 = game[0].rsplit_once(" ").unwrap().1.parse().unwrap();
            let rounds = game[1].split(";").into_iter().map(|round| {
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
        self.data.iter().map(|(_, rounds)| {
            let red = Self::get_max_in_round(rounds, "red");
            let green = Self::get_max_in_round(rounds, "green");
            let blue = Self::get_max_in_round(rounds, "blue");
            red * green * blue
        }).sum::<i32>().to_string()
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
        assert_eq!(day.task2(), "2286");
    }
}
