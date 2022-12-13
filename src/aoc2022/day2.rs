use advent_of_code::Day;

#[derive(Debug, Copy, Clone)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl RockPaperScissors {
    fn new(c: char) -> Self {
        match c {
            'B' => RockPaperScissors::Paper,
            'Y' => RockPaperScissors::Paper,
            'A' => RockPaperScissors::Rock,
            'X' => RockPaperScissors::Rock,
            'C' => RockPaperScissors::Scissors,
            'Z' => RockPaperScissors::Scissors,
            _ => panic!("Wrong type"),
        }
    }

    fn new_winner_of(choose: RockPaperScissors) -> Self {
        match choose {
            RockPaperScissors::Rock => RockPaperScissors::Paper,
            RockPaperScissors::Paper => RockPaperScissors::Scissors,
            RockPaperScissors::Scissors => RockPaperScissors::Rock,
        }
    }

    fn new_loser_of(choose: RockPaperScissors) -> Self {
        match choose {
            RockPaperScissors::Rock => RockPaperScissors::Scissors,
            RockPaperScissors::Paper => RockPaperScissors::Rock,
            RockPaperScissors::Scissors => RockPaperScissors::Paper,
        }
    }

    /*
     * Get opponent score
     */
    fn round(self, opponent: Self) -> i32 {
        match (self, opponent) {
            (RockPaperScissors::Rock, RockPaperScissors::Rock) => 3,
            (RockPaperScissors::Paper, RockPaperScissors::Paper) => 3,
            (RockPaperScissors::Scissors, RockPaperScissors::Scissors) => 3,
            (RockPaperScissors::Rock, RockPaperScissors::Paper) => 6,
            (RockPaperScissors::Rock, RockPaperScissors::Scissors) => 0,
            (RockPaperScissors::Paper, RockPaperScissors::Rock) => 0,
            (RockPaperScissors::Paper, RockPaperScissors::Scissors) => 6,
            (RockPaperScissors::Scissors, RockPaperScissors::Rock) => 6,
            (RockPaperScissors::Scissors, RockPaperScissors::Paper) => 0,
        }
    }

    fn get_value_for_type(self) -> i32 {
        match self {
            RockPaperScissors::Rock => 1,
            RockPaperScissors::Paper => 2,
            RockPaperScissors::Scissors => 3,
        }
    }
}

#[derive(Default)]
pub struct Day2Of2022 {
    data: Vec<(char, char)>,
}

impl Day2Of2022 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Day for Day2Of2022 {
    fn get_day(&self) -> (i32, i32) {
        (2022, 2)
    }

    fn parse(&mut self, data: String) {
        let res = data
            .split('\n')
            .filter(|r| !r.is_empty())
            .map(|round| {
                let mut round = round.chars();
                (round.nth(0).unwrap(), round.nth_back(0).unwrap())
            })
            .collect();
        self.data = res;
    }

    fn task1(&mut self) -> String {
        self.data
            .iter()
            .map(|(first, second)| {
                (
                    RockPaperScissors::new(*first),
                    RockPaperScissors::new(*second),
                )
            })
            .map(|(first, second)| first.round(second) + second.get_value_for_type())
            .sum::<i32>()
            .to_string()
    }

    fn task2(&mut self) -> String {
        self.data
            .iter()
            .map(|round| {
                let first = RockPaperScissors::new(round.0);
                let second = match round.1 {
                    'Y' => first,
                    'X' => RockPaperScissors::new_loser_of(first),
                    'Z' => RockPaperScissors::new_winner_of(first),
                    _ => panic!("Wrong state."),
                };
                (first, second)
            })
            .map(|(first, second)| first.round(second) + second.get_value_for_type())
            .sum::<i32>()
            .to_string()
    }
}
