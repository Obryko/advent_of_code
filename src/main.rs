use clap::Parser;
use dotenvy::dotenv;
use advent_of_code::YEARS;
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

fn main()-> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let cli = Cli::parse();

    let year_number: usize = cli.year.unwrap_or(YEARS.last().unwrap().0);

    let days = YEARS.iter()
        .find(|y| y.0 == year_number)
        .ok_or_else(|| format!("Year {year_number} not found."))?
        .1();


    let day_number = match cli.day {
        Some(d) => d as usize,
        None => *days.keys().next_back()
            .ok_or_else(|| format!("Year {year_number} has no implemented days."))?,
    };
    let task_number = match cli.part {
        Some(1) => Some(advent_of_code::Part::One),
        Some(2) => Some(advent_of_code::Part::Two),
        _ => None
    };

    days
        .get(&day_number)
        .ok_or_else(|| format!("Day {day_number} for year {year_number} not found."))?
        .run(task_number)?;

    Ok(())
}