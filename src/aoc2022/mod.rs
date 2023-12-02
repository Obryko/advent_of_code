use std::collections::HashMap;

use advent_of_code::Day;

use crate::aoc2022::day1::Day1Of2022;
use crate::aoc2022::day2::Day2Of2022;
use crate::aoc2022::day3::Day3Of2022;
use crate::aoc2022::day4::Day4Of2022;
use crate::aoc2022::day5::Day5Of2022;
use crate::aoc2022::day6::Day6Of2022;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;


pub fn get_year_days() -> HashMap<usize, Box<dyn Day>> {
    let mut days: HashMap<usize, Box<dyn Day>> = HashMap::new();
    days.insert(1, Box::new(Day1Of2022::new()));
    days.insert(2, Box::new(Day2Of2022::new()));
    days.insert(3, Box::new(Day3Of2022::new()));
    days.insert(4, Box::new(Day4Of2022::new()));
    days.insert(5, Box::new(Day5Of2022::new()));
    days.insert(5, Box::new(Day6Of2022::new()));
    days
}
