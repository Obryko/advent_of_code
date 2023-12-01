use advent_of_code::Day;

#[derive(Default)]
pub struct Day1Of2023 {
    data: Vec<String>,
}

impl Day1Of2023 {
    pub fn new() -> Self {
        Self::default()
    }
}

const DIGITS: [(&str, i32); 10] = [
    ("ZERO", 0),
    ("ONE", 1),
    ("TWO", 2),
    ("THREE", 3),
    ("FOUR", 4),
    ("FIVE", 5),
    ("SIX", 6),
    ("SEVEN", 7),
    ("EIGHT", 8),
    ("NINE", 9),
];

fn map_line_string_digits_into_digits(line: &String) -> Vec<i32> {
    (0..line.len()).into_iter().map(|i| -> Option<i32> {
        let item = line.chars().nth(i).unwrap();
        if item.is_numeric() {
            return match item.to_string().parse::<i32>() {
                Ok(value) => Some(value),
                _ => None
            };
        };
        let sub_string = &line[i..];
        for (digit, value) in DIGITS {
            if sub_string.to_uppercase().starts_with(digit) {
                return Some(value);
            }
        }

        None
    }).filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
}

fn get_digits_from_line(line: &String) -> Vec<i32> {
    line.chars().filter(|char| char.is_numeric()).map(|char| char.to_string().parse::<i32>().unwrap()).collect()
}

fn find_number_in_line(line: Vec<i32>) -> i32 {
    let first = line.first().unwrap();
    let last = line.last().unwrap();
    let value = first.to_string() + last.to_string().as_str();
    value.parse::<i32>().unwrap()
}

impl Day for Day1Of2023 {
    fn get_day(&self) -> (i32, i32) {
        (2023, 1)
    }

    fn parse(&mut self, data: String) {
        self.data = data.lines().map(|line| line.to_string()).collect();
    }

    fn task1(&mut self) -> String {
        self.data.iter().map(get_digits_from_line).map(find_number_in_line).sum::<i32>().to_string()
    }
    fn task2(&mut self) -> String {
        self.data.iter().map(map_line_string_digits_into_digits).map(find_number_in_line).sum::<i32>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    const INPUT_2: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

    #[test]
    fn task_1() {
        let mut day = Day1Of2023::new();
        day.parse(INPUT_1.to_string());

        assert_eq!(day.task1(), "142");
    }

    #[test]
    fn task_2() {
        let mut day = Day1Of2023::new();
        day.parse(INPUT_2.to_string());

        assert_eq!(day.task2(), "281");
    }
}
