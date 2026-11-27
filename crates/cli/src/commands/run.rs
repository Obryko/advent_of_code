use std::panic::catch_unwind;
use crate::commands::result::CommandOutput;
use crate::commands::{Command, CommandResult};
use crate::error::{CommandError, DayError};
use crate::report::execution::{DayReport, ExecutionReport};
use aoc_client::inputs::get_day_input;
use aoc_core::{Part, PartResult, RegisteredDay};
use aoc_solutions::REGISTRY;
use clap::Args;
use clap::error::ErrorKind;

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

    /// Run all registered days
    #[arg(long, conflicts_with_all(["day", "part"]))]
    all: bool,
}

impl Command for Run {
    fn execute(self) -> CommandResult {
        let part = self.part.map(Part::try_from).transpose().map_err(CommandError::InvalidPart)?;

        let days = self.resolve_days()?;

        let results = days
            .into_iter()
            .map(|day| DayReport {
                year: day.year,
                day: day.day,
                outcome: Self::execute_day(day, part),
            })
            .collect();

        Ok(CommandOutput::Execution(ExecutionReport {
            days: results,
        }))
    }
}

impl Run {
    pub fn validate(&self) -> Result<(), clap::Error> {
        if self.year.is_some() && self.part.is_some() && self.day.is_none() {
            return Err(clap::Error::raw(
                ErrorKind::MissingRequiredArgument,
                "--day is required when using --year with --part",
            ));
        }

        Ok(())
    }
    fn resolve_days(&self) -> Result<Vec<&'static RegisteredDay>, CommandError> {
        if self.all {
            let mut days: Vec<_> = REGISTRY
                .iter()
                .filter(|day| self.year.is_none_or(|year| day.year == year))
                .collect();

            if days.is_empty() {
                return Err(match self.year {
                    Some(year) => CommandError::YearNotFound { year },
                    None => CommandError::EmptyRegistry,
                });
            }

            days.sort_unstable_by_key(|day| (day.year, day.day));

            return Ok(days);
        }

        let year_number = self
            .year
            .or_else(|| REGISTRY.latest_year())
            .ok_or(CommandError::EmptyRegistry)?;

        let day_number = self
            .day
            .map(usize::from)
            .or_else(|| REGISTRY.latest_day(year_number))
            .ok_or(CommandError::YearNotFound { year: year_number })?;

        let day = REGISTRY
            .find(year_number, day_number)
            .ok_or(CommandError::DayNotFound {
                year: year_number,
                day: day_number,
            })?;

        Ok(vec![day])
    }

    fn execute_day(day: &RegisteredDay, part: Option<Part>) -> Result<Vec<PartResult>, DayError> {

        let input = get_day_input(day.year, day.day)
            .map_err(DayError::Input)?;

        let previous_hook = std::panic::take_hook();

        std::panic::set_hook(Box::new(|_| {}));

        let result = std::panic::catch_unwind(|| {
            day.run(input, part)
        }).map_err(|payload| {
            let message = if let Some(message) =
                payload.downcast_ref::<String>()
            {
                message.clone()
            } else if let Some(message) =
                payload.downcast_ref::<&str>()
            {
                (*message).to_owned()
            } else {
                "Unknown panic payload".to_owned()
            };

            DayError::Panic(message)
        })?;

        std::panic::set_hook(previous_hook);

        Ok(result)
    }
}
