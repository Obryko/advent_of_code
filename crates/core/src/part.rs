use std::fmt::Display;
use std::time::Duration;
use crate::Day;

pub struct PartResult {
    pub part: Part,
    pub answer: String,
    pub duration: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    One = 1,
    Two = 2,
}

impl Part {
    pub fn execute(&self, solver: &dyn Day) -> String {
        match self {
            Self::One => solver.task1(),
            Self::Two => solver.task2(),
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}