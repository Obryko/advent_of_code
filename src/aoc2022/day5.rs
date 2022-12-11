use std::borrow::BorrowMut;
use std::collections::BTreeMap;

use async_trait::async_trait;

use advent_of_code::{inputs, Day};

#[derive(Debug)]
struct Command(usize, usize, usize);

pub struct Day5Of2022 {
    commands: Vec<Command>,
    board: Vec<Vec<char>>,
}

impl Day5Of2022 {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            board: Vec::new(),
        }
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

#[async_trait]
impl Day for Day5Of2022 {
    async fn init(&mut self) {
        let data: String = inputs::get_day_input(2022, 5).await;
        let inputs = data.split("\n\n").collect::<Vec<&str>>();

        self.set_board(inputs[0].to_string());
        self.set_commands(inputs[1].to_string());
    }

    fn task1(&mut self) -> String {
        let mut board = self.board.clone();

        self.commands
            .iter()
            .fold(board, |mut board, command| {
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
        let mut board = self.board.clone();

        self.commands
            .iter()
            .fold(board, |mut board, command| {
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
