use advent_of_code::Day;

#[derive(Default, Debug)]
pub struct Day9Of2023 {
    data: Vec<Vec<i32>>,
}

impl Day9Of2023 {
    pub fn new() -> Self {
        Self {
            data: vec![],
        }
    }

    fn get_sum_of_history(&self, get_element: fn(&Vec<i32>) -> &i32, operation: fn(&i32, &i32) -> i32) -> i32 {
        self.data.iter()
            .map(|l| get_history_of_the_line(l, get_element, operation))
            .sum::<i32>()
    }
}

fn get_history_of_the_line(line: &Vec<i32>, get_element: fn(&Vec<i32>) -> &i32, operation: fn(&i32, &i32) -> i32) -> i32 {
    if line.iter().all(|x| x == &0) { return 0; }
    let first =  get_element(line);

    let next_line = (0..(line.len()-1)).map(|i| line[i+1]-line[i]).collect::<Vec<i32>>();

    operation(&first, &get_history_of_the_line(&next_line, get_element, operation))
}

impl Day for Day9Of2023 {
    fn get_day(&self) -> (i32, i32) {
        (2023, 9)
    }

    fn parse(&mut self, data: String) {
        self.data = data
            .lines()
            .map(|l|
                l.split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>();
    }

    fn task1(&self) -> String {
        self.get_sum_of_history(|l: &Vec<i32>| l.last().unwrap(), |a,b| a+b).to_string()
    }

    fn task2(&self) -> String {
        self.get_sum_of_history(|l: &Vec<i32>| l.first().unwrap(), |a,b| a-b).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";

    #[test]
    fn task_1() {
        let mut day = Day9Of2023::new();
        day.parse(INPUT.to_string());
        assert_eq!(day.task1(), "114");
    }

    #[test]
    fn task_2() {
        let mut day = Day9Of2023::new();
        day.parse(INPUT.to_string());
        assert_eq!(day.task2(), "2");
    }
}
