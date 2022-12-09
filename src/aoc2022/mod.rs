use advent_of_code::Day;
use day1::*;
use day2::*;
use day3::*;
use day4::*;

mod day1;
mod day2;
mod day3;
mod day4;

pub async fn run() {
    let mut day1 = Day1Of2022::new();
    let mut day2 = Day2Of2022::new();
    let mut day3 = Day3Of2022::new();
    let mut day4 = Day4Of2022::new();
    let mut aoc2022: Vec<&mut dyn Day> = vec![&mut day1, &mut day2, &mut day3, &mut day4];

    let last = aoc2022.len() - 1;
    let d = &mut aoc2022[last];
    run_day(*d).await
}

async fn run_day<T: Day + ?Sized>(day: &mut T) {
    println!("----- Paring Day -----");
    day.init().await;
    println!("----- Task 1 -----");
    let task1 = day.task1();
    println!("Task 1: {}", task1);
    println!("----- Task 2 -----");
    let task2 = day.task2();
    println!("Task 2: {}", task2);
}
