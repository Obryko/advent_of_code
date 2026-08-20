use crate::Day;

#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
struct CubeSet {
    red: i32,
    green: i32,
    blue: i32,
}

impl CubeSet {
    fn new(red: i32, green:i32, blue:i32) -> Self {
        Self { red, green, blue }
    }
    fn set(&mut self, color: &str, count: i32) {
        match color {
            "red" => self.red = count,
            "green" => self.green = count,
            "blue" => self.blue = count,
            _ => panic!("Unknown color: {}", color),
        }
    }
    fn product(&self) -> i32 {
        self.red * self.green * self.blue
    }

    fn fits(&self, other: &Self) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
}

impl From<&str> for CubeSet {
    fn from(s: &str) -> Self {
        let mut cube = CubeSet::default();
        let parts = s.split(',');

        for part in parts {
            let (count, color) = part.trim().split_once(' ').unwrap();
            cube.set(color, count.parse().unwrap());
        }
        cube
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    rounds: Vec<CubeSet>,
}

impl Game {

    const MAX_CUBE_SET: CubeSet = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    fn new(id: i32, rounds: Vec<CubeSet>) -> Self {
        Self { id, rounds }
    }

    fn is_possible(&self) -> bool {
        self.rounds.iter().all(|round| round.fits(&Self::MAX_CUBE_SET))
    }

    fn max_cube_set(&self) -> CubeSet {
        self.rounds.iter().fold(CubeSet::default(), |acc, cube|
            CubeSet::new(
                acc.red.max(cube.red),
                acc.green.max(cube.green),
                acc.blue.max(cube.blue)
            )
        )
    }
}

impl From<&str> for Game {
    fn from(s: &str) -> Self {
        let (name, game) = s.split_once(':').unwrap();
        let rounds = game.split(';')
            .map(|round| CubeSet::from(round)).collect::<Vec<CubeSet>>();
        let id = name
            .strip_prefix("Game ")
            .unwrap()
            .parse()
            .unwrap();
        Self::new(id, rounds)
    }
}

#[derive(Default, Debug)]
pub struct Day2Of2023 {
    data: Vec<Game>,
}

impl Day for Day2Of2023 {
    fn parse(&mut self, data: String) {
        self.data = data.lines().map(|line| Game::from(line)).collect();
    }

    fn task1(&self) -> String {
        self.data.iter().filter(|game| game.is_possible()).map(|game| game.id).sum::<i32>().to_string()
    }

    fn task2(&self) -> String {
        self.data.iter().map(|game| game.max_cube_set().product()).sum::<i32>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn task_1() {
        let mut day = Day2Of2023::default();
        day.parse(INPUT.to_string());
        assert_eq!(day.task1(), "8");
    }

    #[test]
    fn task_2() {
        let mut day = Day2Of2023::default();
        day.parse(INPUT.to_string());
        assert_eq!(day.task2(), "2286");
    }

    #[test]
    fn should_detect_impossible_cube_set() {
        let cubes = CubeSet::new(11, 100, 1);

        assert!(!cubes.fits(&Game::MAX_CUBE_SET));
    }
}
