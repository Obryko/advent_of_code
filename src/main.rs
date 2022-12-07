use dotenv::dotenv;

use aoc2015::*;
use aoc2022::*;

mod aoc2015;
mod aoc2022;

#[tokio::main]
async fn main() {
    dotenv().ok();

    run().await;
}
