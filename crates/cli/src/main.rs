use aoc_solutions::{REGISTRY};
use clap::Parser;
use dotenvy::dotenv;
use aoc_client::inputs::get_day_input;
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Year to run [default: latest implemented year]
    #[arg(short, long)]
    year: Option<usize>,
    /// Day to run [default: latest implemented day for the year]
    #[arg(short, long,value_parser = clap::value_parser!(u8).range(1..=25))]
    day: Option<u8>,
    /// Part to run (1 or 2) [default: run both]
    #[arg(short, long,value_parser = clap::value_parser!(u8).range(1..=2))]
    part: Option<u8>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let cli = Cli::parse();

    let year_number = cli.year
        .or_else(|| REGISTRY.latest_year())
        .ok_or_else(|| "No latest year found.".to_string())?;

    let day_number = cli.day
        .map(usize::from)
        .or_else(|| REGISTRY.latest_day(year_number))
        .ok_or_else(|| {
            format!("Year {year_number} has no implemented days.")
        })?;

    let task_number = match cli.part {
        Some(1) => Some(aoc_core::Part::One),
        Some(2) => Some(aoc_core::Part::Two),
        _ => None,
    };

    let day = REGISTRY.find(year_number, day_number)
        .ok_or_else(|| format!("Day {day_number} for year {year_number} not found."))?;

    let input = get_day_input(year_number, day_number)?;
    day.run(input, task_number);

    Ok(())
}
