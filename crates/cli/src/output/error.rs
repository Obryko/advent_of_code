use crate::error::{CommandError, DayError};

pub fn print_error(error: &CommandError) {
    eprintln!("\n✗ Error: {error}\n");
}

pub fn print_day_error(year: usize, day: usize, error: &DayError) {
    eprintln!("  ✗ {year} / Day {day:02} — {error}");
}
