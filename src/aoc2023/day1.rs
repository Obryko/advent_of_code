use crate::Day;

#[derive(Default)]
pub struct Day1Of2023 {
    data: Vec<String>,
}

const DIGITS: [(&str, u32); 10] = [
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

impl Day1Of2023 {
    fn calibration_value(line: &str) -> u32 {
            let first = line
                .chars()
                .find_map(|c| c.to_digit(10))
                .unwrap_or_default();

            let last = line
                .chars()
                .rev()
                .find_map(|c| c.to_digit(10))
                .unwrap_or_default();

            first * 10 + last
    }

    fn digit_at(input: &str) -> Option<u32> {
        if let Some(digit) = input.chars().next()?.to_digit(10) {
            return Some(digit as u32);
        }

        DIGITS
            .iter()
            .find_map(|(word, digit)| {
                input.starts_with(word).then_some(*digit)
            })
    }

    fn calibration_value_with_words(line: &str) -> u32 {
        let mut digits = line
            .char_indices()
            .filter_map(|(index, _)| Self::digit_at(&line[index..]));

        let first = digits.next().unwrap();
        let last = digits.last().unwrap_or(first);

        first * 10 + last
    }
}

impl Day for Day1Of2023 {
    fn parse(&mut self, data: String) {
        self.data = data.lines().map(|line| line.to_string()).collect();
    }

    fn task1(&self) -> String {
        self.data.iter()
            .map(|s| Self::calibration_value(s))
            .sum::<u32>().to_string()
    }
    fn task2(&self) -> String {
        self.data.iter().map(|s| Self::calibration_value_with_words(s)).sum::<u32>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    const INPUT_2: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

    #[test]
    fn task_1() {
        let mut day = Day1Of2023::default();
        day.parse(INPUT_1.to_string());

        assert_eq!(day.task1(), "142");
    }

    #[test]
    fn task_2() {
        let mut day = Day1Of2023::default();
        day.parse(INPUT_2.to_string());

        assert_eq!(day.task2(), "281");
    }
}
