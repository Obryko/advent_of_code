extern crate core;

use dotenv::dotenv;

use aoc2015::*;
use aoc2022::*;
use aoc2023::run as run2023;

mod aoc2015;
mod aoc2022;
mod aoc2023;

#[tokio::main]
async fn main() {
    dotenv().ok();

    run2023().await;
}
