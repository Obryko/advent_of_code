use std::collections::HashMap;

use advent_of_code::Day;
use day1::*;

mod day1;



pub fn get_year_days() -> HashMap<usize, Box<dyn Day>> {
    let mut days: HashMap<usize, Box<dyn Day>> = HashMap::new();
    days.insert(1, Box::new(Day1Of2023::new()));

    days
}