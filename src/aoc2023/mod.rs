use std::marker::Send;

use advent_of_code::Day;
use advent_of_code::run_day;

use day1::*;

mod day1;

pub async fn run() {
    let mut day1 = Day1Of2023::new();
    // let mut day7 = Day7Of2022::new();
    let mut aoc2022: Vec<&mut (dyn Day + Send)> = vec![
        &mut day1,
    ];

    let last = aoc2022.len() - 1;
    let d = &mut aoc2022[last];
    run_day(*d).await
}
