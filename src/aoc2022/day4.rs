use crate::common::intervals::{Interval, IntervalError};
use crate::Day;

#[derive(Debug, Copy, Clone)]
struct Pair(Interval, Interval);

impl Pair {
    pub fn new(value: &str) -> Self {
        Self::from_tuple(value.split_once(',').unwrap())
    }

    fn from_tuple((first, second): (&str, &str)) -> Self {
        Pair(
            Self::parse_value(first).unwrap(),
            Self::parse_value(second).unwrap(),
        )
    }

    fn parse_value(value: &str) -> Result<Interval, IntervalError> {
        let (start,end) = value.split_once('-').unwrap();
        (start.parse::<i64>().unwrap(), end.parse::<i64>().unwrap()).try_into()
    }

    pub fn contain(&self) -> bool {
        self.0.contains_interval(&self.1) || self.1.contains_interval(&self.0)
    }

    pub fn overlap(&self) -> bool {
        self.0.overlaps(&self.1)
    }
}

#[derive(Default)]
pub struct Day4Of2022 {
    data: Vec<Pair>,
}

impl Day for Day4Of2022 {
    fn parse(&mut self, data: String) {
        self.data = data
            .lines()
            .filter(|pair| !pair.is_empty())
            .map(Pair::new)
            .collect();
    }

    fn task1(&self) -> String {
        self.data
            .iter()
            .filter(|&pair| pair.contain())
            .count()
            .to_string()
    }

    fn task2(&self) -> String {
        self.data
            .iter()
            .filter(|&pair| pair.overlap())
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn task_1() {
        let mut day = Day4Of2022::default();
        day.parse(INPUT.to_string());
        assert_eq!(day.task1(), "2");
    }

    #[test]
    fn task_2() {
        let mut day = Day4Of2022::default();
        day.parse(INPUT.to_string());
        assert_eq!(day.task2(), "4");
    }
}
