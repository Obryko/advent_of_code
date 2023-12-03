use std::collections::HashMap;

use advent_of_code::Day;
use crate::aoc2023::day1::Day1Of2023;
use crate::aoc2023::day2::Day2Of2023;

mod day1;
mod day2;


pub fn get_year_days() -> HashMap<usize, Box<dyn Day>> {
    let mut days: HashMap<usize, Box<dyn Day>> = HashMap::new();
    days.insert(1, Box::new(Day1Of2023::new()));
    days.insert(2, Box::new(Day2Of2023::new()));

    days
}