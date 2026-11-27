use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("No registered solutions found.")]
    EmptyRegistry,

    #[error("Year {year} has no implemented days.")]
    YearNotFound { year: usize },

    #[error("Day {day} of {year} is not registered.")]
    DayNotFound { year: usize, day: usize },

    #[error("Invalid part number: {0}. Expected 1 or 2.")]
    InvalidPart(u8),
}

#[derive(Debug, Error)]
pub enum DayError {
    #[error("Failed to load puzzle input: {0}")]
    Input(#[source] Box<dyn std::error::Error>),

    #[error("Solver panicked: {0}")]
    Panic(String),
}
