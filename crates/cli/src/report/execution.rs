use std::time::Duration;

use crate::error::DayError;
use aoc_core::PartResult;

pub struct DayReport {
    pub year: usize,
    pub day: usize,
    pub outcome: Result<Vec<PartResult>, DayError>,
}

pub struct ExecutionReport {
    pub days: Vec<DayReport>,
}

impl ExecutionReport {
    pub fn total_days(&self) -> usize {
        self.days.len()
    }

    pub fn successful_days(&self) -> usize {
        self.days.iter().filter(|day| day.outcome.is_ok()).count()
    }

    pub fn failed_days(&self) -> usize {
        self.days.iter().filter(|day| day.outcome.is_err()).count()
    }

    pub fn is_success(&self) -> bool {
        self.days.iter().all(|day| day.outcome.is_ok())
    }

    pub fn total_duration(&self) -> Duration {
        self.days
            .iter()
            .filter_map(|day| day.outcome.as_ref().ok())
            .flatten()
            .map(|part| part.duration)
            .sum()
    }
}
