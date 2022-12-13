use advent_of_code::Day;

struct Command(usize, usize, usize);

#[derive(Default)]
pub struct Day5Of2022 {
    commands: Vec<Command>,
    board: Vec<Vec<char>>,
}

impl Day5Of2022 {
    pub fn new() -> Self {
        Self::default()
    }

    fn set_commands(&mut self, value: String) {
        let commands = value
            .split('\n')
            .filter(|command| !command.is_empty())
            .map(|command| {
                let inputs = command.split(' ').collect::<Vec<&str>>();
                Command(
                    inputs[1].parse::<usize>().unwrap(),
                    inputs[3].parse::<usize>().unwrap(),
                    inputs[5].parse::<usize>().unwrap(),
                )
            })
            .collect();
        self.commands = commands;
    }

    fn set_board(&mut self, value: String) {
        let board: Vec<Vec<char>> = value
            .split('\n')
            .filter(|row| !row.is_empty())
            .rev()
            .map(|row| {
                let chars = row.chars().collect::<Vec<char>>();
                let mut chars_iterator = chars.iter().peekable();
                let mut result: Vec<char> = Vec::new();

                while chars_iterator.peek().is_some() {
                    let s: &char = chars_iterator
                        .by_ref()
                        .take(3)
                        .find(|&&c| c.is_alphabetic())
                        .unwrap_or(&' ');
                    result.push(*s);
                    chars_iterator.by_ref().next();
                }
                result
            })
            .fold(Vec::new(), |mut acc, value| {
                for (index, c) in value.iter().enumerate() {
                    if None == acc.get(index) {
                        acc.insert(index, Vec::new())
                    }
                    if c.is_alphabetic() {
                        acc[index].push(*c);
                    }
                }
                acc
            });
        self.board = board;
    }
}

impl Day for Day5Of2022 {
    fn get_day(&self) -> (i32, i32) {
        (2022, 5)
    }

    fn parse(&mut self, data: String) {
        let inputs = data.split("\n\n").collect::<Vec<&str>>();

        self.set_board(inputs[0].to_string());
        self.set_commands(inputs[1].to_string());
    }

    fn task1(&mut self) -> String {
        self.commands
            .iter()
            .fold(self.board.clone(), |mut board, command| {
                for _ in 0..command.0 {
                    let value = board[command.1 - 1].pop().unwrap();
                    board[command.2 - 1].push(value);
                }
                board
            })
            .iter()
            .map(|chars| chars.last().unwrap())
            .collect::<String>()
    }
    fn task2(&mut self) -> String {
        self.commands
            .iter()
            .fold(self.board.clone(), |mut board, command| {
                let mut values = Vec::new();
                for _ in 0..command.0 {
                    let value = board[command.1 - 1].pop().unwrap();
                    values.push(value);
                }
                board[command.2 - 1].extend(values.iter().rev());

                board
            })
            .iter()
            .map(|chars| chars.last().unwrap())
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn task_1() {
        let mut day = Day5Of2022::new();
        day.parse(INPUT.to_string());

        assert_eq!(day.task1(), "CMZ");
    }

    #[test]
    fn task_2() {
        let mut day = Day5Of2022::new();
        day.parse(INPUT.to_string());

        assert_eq!(day.task2(), "MCD");
    }
}
