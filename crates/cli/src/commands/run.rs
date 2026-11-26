use clap::Args;
use aoc_client::inputs::get_day_input;
use aoc_solutions::REGISTRY;
use crate::commands::{Command, CommandResult};
use crate::output;

#[derive(Args)]
pub struct Run {
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

impl Command for Run {
    fn execute(self) -> CommandResult {
        let year_number = self.year
            .or_else(|| REGISTRY.latest_year())
            .ok_or_else(|| "No latest year found.".to_string())?;

        let day_number = self.day
            .map(usize::from)
            .or_else(|| REGISTRY.latest_day(year_number))
            .ok_or_else(|| {
                format!("Year {year_number} has no implemented days.")
            })?;

        let task_number = match self.part {
            Some(1) => Some(aoc_core::Part::One),
            Some(2) => Some(aoc_core::Part::Two),
            _ => None,
        };

        let day = REGISTRY.find(year_number, day_number)
            .ok_or_else(|| format!("Day {day_number} for year {year_number} not found."))?;

        let input = get_day_input(year_number, day_number)?;

        println!("\n🎄 Advent of Code {year_number} · Day {day_number:02}");
        println!("────────────────────────────────────────");

        let results = day.run(input, task_number);

        output::execution::print_day(year_number, day_number, &results);

        Ok(())
    }
}