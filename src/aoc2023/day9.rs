use crate::Day;

#[derive(Default, Debug)]
struct Extrapolation {
    previous_sum: i32,
    next_sum: i32,
}

impl Extrapolation {
    fn new(data: &[Vec<i32>]) -> Self {
        let (previous_sum, next_sum) = Self::sum_histories(data);
        Self {
            previous_sum,
            next_sum,
        }
    }
    fn sum_histories(data: &[Vec<i32>]) -> (i32, i32) {
        data.iter()
            .map(|line| Self::extrapolate(line))
            .fold((0, 0), |(previous_sum, next_sum), (previous, next)| {
                (previous_sum + previous, next_sum + next)
            })
    }

    fn extrapolate(line: &[i32]) -> (i32, i32) {
        if line.iter().all(|&value| value == 0) {
            return (0, 0);
        }

        let differences = line
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect::<Vec<_>>();

        let (previous_difference, next_difference) = Self::extrapolate(&differences);

        let previous = line.first().unwrap() - previous_difference;
        let next = line.last().unwrap() + next_difference;

        (previous, next)
    }
}

#[derive(Default, Debug)]
pub struct Day9Of2023 {
    data: Extrapolation
}

impl Day for Day9Of2023 {
    fn parse(&mut self, data: String) {
        let histories = data
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|value| value.parse().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        self.data = Extrapolation::new(&histories);
    }

    fn task1(&self) -> String {
        self.data.next_sum.to_string()
    }
    fn task2(&self) -> String {
        self.data.previous_sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";

    #[test]
    fn task_1() {
        let mut day = Day9Of2023::default();
        day.parse(INPUT.to_string());
        assert_eq!(day.task1(), "114");
    }

    #[test]
    fn task_2() {
        let mut day = Day9Of2023::default();
        day.parse(INPUT.to_string());
        assert_eq!(day.task2(), "2");
    }
}
