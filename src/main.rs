extern crate core;

use std::collections::HashMap;
use std::env;

use dotenv::dotenv;
use advent_of_code::Day;
use advent_of_code::inputs::get_day_input;

mod aoc2015;
mod aoc2022;
mod aoc2023;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();
    let year_value = match args[1].parse::<usize>() {
        Ok(y) if y >= 2015 && y<= 2023 => Some(y),
        _ => None
    };
    let day_value = match args[2].parse::<usize>() {
        Ok(d) if d >= 1 && d <= 25 => Some(d),
        _ => None
    };

    let mut days: HashMap<usize, Box<dyn Day>> = match year_value {
        Some(2015) => aoc2015::get_year_days(),
        Some(2022) => aoc2022::get_year_days(),
        Some(2023) => aoc2023::get_year_days(),
        _ => aoc2023::get_year_days(),
    };

    let day = match day_value {
        Some(number) => days.get_mut(&number).unwrap(),
        None => days.get_mut(&days.len()).unwrap()
    };
    let data = get_day_input(day.get_day()).await;
    day.parse(data);
    day.run();
}
